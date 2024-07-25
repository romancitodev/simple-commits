use promptuity::themes::FancyTheme;
use promptuity::{Promptuity, Term, Terminal, Theme};

/// Generates the config for the TUI
///
/// Returns a [`RenderConfig`] from the inquire TUI.
pub fn generate_prompt<'a, W>(
    term: &'a mut dyn Terminal<W>,
    theme: &'a mut dyn Theme<W>,
) -> Promptuity<'a, W>
where
    W: std::io::Write,
{
    Promptuity::new(term, theme)
}
pub fn prepare() -> (Term<std::io::Stderr>, FancyTheme) {
    (Term::default(), FancyTheme::default())
}
