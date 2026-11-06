use super::*;

#[test]
fn a_value_names_its_theme_and_any_other_is_red() {
    assert_eq!(palette("mango"), &MANGO);
    assert_eq!(palette("blue"), &BLUE);
    assert_eq!(palette(""), &RED);
    assert_eq!(palette("plaid"), &RED);
}

/// How bright a colour reads, 0 to 255.
fn brightness([r, g, b]: Rgb) -> u32 {
    (u32::from(r) * 3 + u32::from(g) * 6 + u32::from(b)) / 10
}

#[test]
fn every_theme_reads_the_same_way_round() {
    for (name, theme) in THEMES {
        assert!(brightness(theme.panel) < brightness(theme.button), "{name}: buttons stand out from the panel");
        assert!(brightness(theme.button) < brightness(theme.dim), "{name}: dim text reads on a button");
        assert!(brightness(theme.dim) < brightness(theme.text), "{name}: text is brighter than dim text");
        assert!(brightness(theme.row) + 100 < brightness(theme.text), "{name}: text reads on what is chosen");
    }
}
