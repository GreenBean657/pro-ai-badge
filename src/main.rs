use std::fs;
use badgelib::{Badge, Color};

fn main() {

  // Generate "YES AI" SVG file.
  let badge = Badge::default()
    .for_version("version", "1.0.0")
    .label("YES")
    .value("AI")
    .label_color(Color::Green)
    .value_color(Color::White)
    .to_svg();

  fs::write("./badge.svg", badge).expect("");
}
