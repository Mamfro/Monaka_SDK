//! Monaka's colour themes (`theme`, its setting): the colours of the headset's panels and the 2D
//! window alike, each drawing them its own way. What a colour means (on, stop, apply, a warning)
//! is the same in every theme.

/// Red, green and blue.
pub type Rgb = [u8; 3];

/// A theme's colours.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    /// Behind everything.
    pub panel: Rgb,
    pub button: Rgb,
    /// What is chosen: a tab, the game in the list.
    pub row: Rgb,
    pub text: Rgb,
    pub dim: Rgb,
    /// Separators and the scroll bar's track.
    pub line: Rgb,
    /// The outline of what is in front, the cursor, a hovered target.
    pub accent: Rgb,
}

/// A game playing, a setting that applies now, a check passed.
pub const ON: Rgb = [143, 212, 160];
pub const STOP: Rgb = [184, 48, 42];
pub const APPLY: Rgb = [74, 125, 79];
/// A problem, or a number past its limit.
pub const WARN: Rgb = [255, 106, 92];

pub const RED: Palette = Palette { panel: [25, 15, 17], button: [59, 38, 41], row: [110, 31, 43], text: [244, 231, 232], dim: [179, 150, 153], line: [61, 42, 45], accent: [255, 106, 92] };
pub const BLUE: Palette = Palette { panel: [15, 19, 28], button: [38, 46, 64], row: [31, 62, 118], text: [231, 236, 244], dim: [150, 161, 179], line: [42, 50, 66], accent: [92, 168, 255] };
pub const GREEN: Palette = Palette { panel: [14, 24, 18], button: [36, 56, 44], row: [31, 98, 62], text: [231, 243, 235], dim: [150, 176, 160], line: [42, 60, 50], accent: [94, 224, 146] };
pub const MANGO: Palette = Palette { panel: [28, 20, 10], button: [62, 46, 26], row: [118, 70, 14], text: [246, 238, 226], dim: [184, 165, 138], line: [66, 50, 32], accent: [255, 176, 46] };

/// Every theme by its setting's value.
pub const THEMES: [(&str, Palette); 4] = [("red", RED), ("blue", BLUE), ("green", GREEN), ("mango", MANGO)];

/// The theme the setting's value `id` names; red for any other.
pub fn palette(id: &str) -> &'static Palette {
    THEMES.iter().find(|(name, _)| *name == id).map_or(&RED, |(_, palette)| palette)
}

#[cfg(test)]
mod tests;
