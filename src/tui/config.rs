use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

/// Generates the config for the TUI
///
/// Returns a [`RenderConfig`] from the inquire TUI.
pub fn generate_config<'config>() -> RenderConfig<'config> {
    let prefix = Styled::new("").with_fg(Color::White);
    let selected = Styled::new("►").with_fg(Color::DarkGrey);
    let skipped = Styled::new("*skipepd*").with_fg(Color::DarkGrey);
    let msg = StyleSheet::empty()
        .with_fg(Color::DarkGrey)
        .with_attr(Attributes::BOLD);
    RenderConfig::default_colored()
        .with_prompt_prefix(prefix)
        .with_highlighted_option_prefix(selected)
        .with_canceled_prompt_indicator(skipped)
        .with_help_message(msg)
}
