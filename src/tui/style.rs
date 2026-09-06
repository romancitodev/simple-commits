use nobubbles::rimel::{self, palette, Block, Color};

/// Accent color used for every prompt title and banner in this CLI.
pub const ACCENT: Color = palette::MAUVE;

/// A prompt title painted as a solid badge (`fg` on `bg`, bold) instead of a bare string.
///
/// Every `select`/`confirm`/`input`/`task` takes anything that turns `Into<Block>`, and a
/// styled `Block` is just another one — same trick nobubbles' own examples use
/// (`commit.rs`, `deep-thought.rs`).
pub fn subtitle(text: impl Into<String>) -> Block {
    rimel::text(text.into())
        .fg(palette::TEXT)
        .bg(palette::SURFACE0)
        .bold()
        .px(1)
}
/// A boxed preview of the commit message, shown before confirming execution — same shape as
/// the `summary` card in nobubbles' own `commit.rs` example.
pub fn preview_card(message: &str) -> Block {
    rimel::text(message)
        .px(2)
        .py(1)
        .rounded()
        .border_color(ACCENT)
}

/// One-time banner printed above the whole session, before `intro` puts the terminal in raw
/// mode.
pub fn banner() {
    let badge = rimel::text("simple-commits")
        .fg(palette::BASE)
        .bg(ACCENT)
        .bold()
        .px(1);
    let aside = rimel::text("  conventional commits, guided").fg(palette::OVERLAY1);

    println!();
    println!("{}", rimel::row([badge, aside]));
    println!("{}", rimel::separator(44).fg(palette::SURFACE1));
}
