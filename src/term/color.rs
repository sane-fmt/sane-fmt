pub use ansi_term::Style;

/// Define a single color
pub trait Color {
    fn paint(&self, text: &str) -> String;
}

/// Define a scheme of color
pub struct ColorScheme<C: Color> {
    pub scan: C,
    pub skip: C,
    pub skip_name: C,
    pub find_indicator: C,
    pub find_name: C,
}

/// Scheme of no color
pub struct Colorless;
impl Color for Colorless {
    fn paint(&self, text: &str) -> String {
        text.to_owned()
    }
}

/// Scheme with styles
pub struct Colorful {
    style: Style,
}

impl Colorful {
    pub fn new(style: Style) -> Self {
        Colorful { style }
    }
}

impl Color for Colorful {
    fn paint(&self, text: &str) -> String {
        format!("{}", self.style.paint(text))
    }
}

/// Create a scheme of no color
pub fn colorless() -> ColorScheme<Colorless> {
    ColorScheme {
        scan: Colorless,
        skip: Colorless,
        skip_name: Colorless,
        find_indicator: Colorless,
        find_name: Colorless,
    }
}

/// Create a scheme of styles
pub fn colorful() -> ColorScheme<Colorful> {
    let style = || Style::default();

    ColorScheme {
        scan: Colorful::new(style().dimmed()),
        skip: Colorful::new(style().dimmed()),
        skip_name: Colorful::new(style().dimmed().strikethrough()),
        find_indicator: Colorful::new(style()),
        find_name: Colorful::new(style().bold()),
    }
}
