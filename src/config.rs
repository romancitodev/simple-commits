use crate::files;
use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

fn generate_config_data<'a>() -> RenderConfig<'a> {
    let prompt_prefix = Styled::new("").with_fg(Color::White);
    let selected = Styled::new("➤").with_fg(Color::Grey);
    let skipped = Styled::new("*skipped*").with_fg(Color::DarkGrey);
    let help_message = StyleSheet::empty()
        .with_fg(Color::DarkGrey)
        .with_attr(Attributes::BOLD);
    let config = RenderConfig::default_colored()
        .with_prompt_prefix(prompt_prefix)
        .with_highlighted_option_prefix(selected)
        .with_canceled_prompt_indicator(skipped)
        .with_help_message(help_message);
    config
}

/// Set the render config and other things just in one call
pub fn install() -> () {
    inquire::set_global_render_config(generate_config_data());
    files::init();
}
