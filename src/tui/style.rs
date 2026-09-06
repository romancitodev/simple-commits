use nobubbles::rimel::{self, Block, Color, Ramp, colorgrad, palette};

/// Deep green to a bright "added line" green to pale mint — the colors of a commit, for the
/// banner's app name.
fn commit_green() -> Ramp {
  Ramp::new(
    colorgrad::GradientBuilder::new()
      .html_colors(&["#0b3d2e", "#2ecc71", "#b7f7c9"])
      .build::<colorgrad::LinearGradient>()
      .expect("static hex stops always build"),
  )
}

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

/// The same box, once the commit is real: green instead of the accent, the way `styles.rs`
/// swaps a theme's accent to signal state rather than changing the shape.
pub fn success_card(message: &str) -> Block {
  rimel::text(message)
    .px(2)
    .py(1)
    .rounded()
    .border_color(palette::GREEN)
}

/// One-time banner printed above the whole session, before `intro` puts the terminal in raw
/// mode.
pub fn banner() {
  // `.px()` pads *after* the gradient is painted, so the padding cells would miss it and
  // break the pill — spaces baked into the text itself sit inside the painted rows instead.
  let badge = rimel::text(" simple-commits ")
    .fg(palette::BASE)
    .bold()
    .gradient(commit_green())
    .on_bg();
  let aside = rimel::text("  conventional commits, guided").fg(palette::OVERLAY1);

  println!();
  println!("{}", rimel::row([badge, aside]));
  println!("{}", rimel::separator(44).fg(palette::SURFACE1));
}
