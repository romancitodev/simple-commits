use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use promptuity::event::*;
use promptuity::pagination::paginate;
use promptuity::prompts::{DefaultSelectFormatter, SelectFormatter, SelectOption};
use promptuity::style::*;
use promptuity::{Error, InputCursor, Prompt, PromptBody, PromptInput, PromptState, RenderPayload};

pub struct Autocomplete {
    formatter: DefaultSelectFormatter,
    message: String,
    page_size: usize,
    options: Vec<SelectOption<String>>,
    filtered_options: Vec<(usize, i64)>,
    index: usize,
    input: InputCursor,
    matcher: SkimMatcherV2,
    priority: AutocompletePriority,
    strict: bool,
    skip: bool,
}

#[derive(Clone, Copy)]
pub enum AutocompletePriority {
    Hint,
    Label,
}

impl From<AutocompletePriority> for (i64, i64) {
    fn from(value: AutocompletePriority) -> (i64, i64) {
        match value {
            AutocompletePriority::Hint => (1, 4),
            AutocompletePriority::Label => (4, 1),
        }
    }
}

impl Autocomplete {
    pub fn new(
        message: impl std::fmt::Display,
        strict: bool,
        priority: AutocompletePriority,
        options: Vec<SelectOption<String>>,
    ) -> Self {
        Self {
            formatter: DefaultSelectFormatter::new(),
            message: message.to_string(),
            page_size: 8,
            options,
            filtered_options: Vec::new(),
            index: 0,
            input: InputCursor::default(),
            matcher: SkimMatcherV2::default(),
            priority,
            strict,
            skip: false,
        }
    }

    fn run_filter(&mut self) {
        let pattern = self.input.value();
        let (priority_label, priority_hint): (i64, i64) = self.priority.into();

        self.filtered_options = self
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, option)| {
                let label = &option.label;
                let hint = option.hint.clone().unwrap_or_default();
                let a = self
                    .matcher
                    .fuzzy_match(label, &pattern)
                    .unwrap_or_default();
                let b = self
                    .matcher
                    .fuzzy_match(&hint, &pattern)
                    .unwrap_or_default();

                let c = (a.saturating_mul(priority_label))
                    .saturating_add(b.saturating_mul(priority_hint))
                    .saturating_sub(i as i64);

                log::trace!("{pattern} -> {label}; {a} & {b} = {c}");
                if c <= 0 && !pattern.is_empty() {
                    return None;
                }

                Some((i, c))
            })
            .collect::<Vec<_>>();

        self.filtered_options.sort_by_key(|(_, s)| *s);
        self.filtered_options.reverse();

        self.index = std::cmp::min(self.filtered_options.len().saturating_sub(1), self.index);
    }

    fn current_option(&self) -> Option<&SelectOption<String>> {
        self.filtered_options
            .get(self.index)
            .and_then(|(idx, _)| self.options.get(*idx))
    }
}

impl AsMut<Autocomplete> for Autocomplete {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Prompt for Autocomplete {
    type Output = String;

    fn setup(&mut self) -> Result<(), Error> {
        if self.options.is_empty() {
            self.filtered_options = Vec::new();
            return Ok(());
        }

        self.filtered_options = (0..self.options.len()).map(|i| (i, 0)).collect();

        Ok(())
    }

    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> promptuity::PromptState {
        match (code, modifiers) {
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => PromptState::Cancel,
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                if self.strict {
                    PromptState::Active
                } else {
                    self.skip = true;
                    PromptState::Submit
                }
            }
            (KeyCode::Enter, _) => match self.current_option() {
                Some(_) => PromptState::Submit,
                _ => {
                    if self.strict {
                        PromptState::Error("No matches found".into())
                    } else {
                        PromptState::Submit
                    }
                }
            },
            (KeyCode::Up, _)
            | (KeyCode::Char('k'), KeyModifiers::CONTROL)
            | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.index = self.index.saturating_sub(1);
                PromptState::Active
            }
            (KeyCode::Down, _)
            | (KeyCode::Char('j'), KeyModifiers::CONTROL)
            | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                self.index = std::cmp::min(
                    self.filtered_options.len().saturating_sub(1),
                    self.index.saturating_add(1),
                );
                PromptState::Active
            }
            (KeyCode::Left, _) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                self.input.move_left();
                PromptState::Active
            }
            (KeyCode::Right, _) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                self.input.move_right();
                PromptState::Active
            }
            (KeyCode::Home, _) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                self.input.move_home();
                PromptState::Active
            }
            (KeyCode::End, _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                self.input.move_end();
                PromptState::Active
            }
            (KeyCode::Backspace, _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.input.delete_left_char();
                self.run_filter();
                PromptState::Active
            }
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                self.input.delete_left_word();
                self.run_filter();
                PromptState::Active
            }
            (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.input.delete_right_char();
                self.run_filter();
                PromptState::Active
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.input.delete_line();
                self.run_filter();
                PromptState::Active
            }
            (KeyCode::Char(c), _) => {
                self.input.insert(c);
                self.run_filter();
                PromptState::Active
            }
            _ => PromptState::Active,
        }
    }

    fn submit(&mut self) -> Self::Output {
        if self.skip {
            return String::new();
        };
        if self.strict {
            self.current_option().unwrap().value.clone()
        } else {
            self.current_option()
                .map_or(self.input.value(), |option| option.value.clone())
        }
    }

    fn render(&mut self, state: &PromptState) -> Result<RenderPayload, String> {
        let hint = (!self.strict).then_some(String::from("Ctrl + S to skip"));
        let payload = RenderPayload::new(self.message.clone(), hint, None);

        match state {
            PromptState::Submit => {
                if self.skip {
                    return Ok(payload.input(PromptInput::Raw(String::from("skipped"))));
                }
                let option = self
                    .current_option()
                    .map_or(self.input.value(), |option| option.value.clone());
                Ok(payload.input(PromptInput::Raw(option)))
            }

            _ => {
                let page = paginate(self.page_size, &self.filtered_options, self.index);
                let options = page
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, (idx, _))| {
                        let option = self.options.get(*idx).unwrap();
                        let active = i == page.cursor;
                        self.formatter.option(
                            self.formatter.option_icon(active),
                            self.formatter.option_label(option.label.clone(), active),
                            self.formatter.option_hint(option.hint.clone(), active),
                            active,
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let raw = if options.is_empty() {
                    Styled::new("<No matches found>")
                        .fg(Color::DarkGrey)
                        .to_string()
                } else {
                    options.to_string()
                };

                Ok(payload
                    .input(PromptInput::Cursor(self.input.clone()))
                    .body(PromptBody::Raw(raw)))
            }
        }
    }
}
