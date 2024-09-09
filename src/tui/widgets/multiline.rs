use promptuity::{
    event::{KeyCode, KeyModifiers},
    style::Styled,
    InputCursor, Prompt, PromptInput, PromptState, RenderPayload,
};

pub struct MultiInput {
    message: String,
    index: usize,
    input: InputCursor,
}

impl MultiInput {
    pub fn new() -> MultiInput {
        Self {
            message: String::new(),
            index: 0,
            input: InputCursor::default(),
        }
    }

    pub fn enter(&mut self) {
        self.input.insert('\n');
    }
}

impl Prompt for MultiInput {
    type Output = String;

    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> promptuity::PromptState {
        match (code, modifiers) {
            (KeyCode::Enter, KeyModifiers::CONTROL) => {
                self.enter();
                PromptState::Active
            }
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => PromptState::Cancel,
            (KeyCode::Char('s'), KeyModifiers::CONTROL) | (KeyCode::Enter, _) => {
                PromptState::Submit
            }
            (KeyCode::Up, _)
            | (KeyCode::Char('k'), KeyModifiers::CONTROL)
            | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.index = self.index.saturating_sub(1);
                PromptState::Active
            }
            (KeyCode::Down, _)
            | (KeyCode::Char('j'), KeyModifiers::CONTROL)
            | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                _ = self.index.saturating_add(1);
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
                PromptState::Active
            }
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                self.input.delete_left_word();
                PromptState::Active
            }
            (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.input.delete_right_char();
                PromptState::Active
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.input.delete_line();
                PromptState::Active
            }
            (KeyCode::Char(c), _) => {
                self.input.insert(c);
                PromptState::Active
            }
            _ => PromptState::Active,
        }
    }

    fn submit(&mut self) -> Self::Output {
        todo!()
    }

    // TODO: finish to correctly render the things
    fn render(
        &mut self,
        state: &promptuity::PromptState,
    ) -> Result<promptuity::RenderPayload, String> {
        let payload = RenderPayload::new(self.message.clone(), None, None);
        match state {
            PromptState::Submit => Ok(payload.input(PromptInput::Raw(String::from("uploaded")))),
            _ => Ok(payload.input(promptuity::PromptInput::Raw(
                self.input
                    .value()
                    .split('\n')
                    .map(|line| {
                        format!(
                            "{}  {}",
                            Styled::new("│").fg(promptuity::style::Color::DarkGrey),
                            line
                        )
                    })
                    // .map(|line| Styled::new(line).to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ))),
        }
    }
}
