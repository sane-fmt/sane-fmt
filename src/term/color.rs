pub use ansi_term::Style;

/// Define a single color
pub trait Color {
    fn paint(&self, text: &str) -> String;
}

/// Define a scheme of color
pub trait ColorScheme {
    fn scan(&self) -> BoxedColor;
    fn skip(&self) -> BoxedColor;
    fn skip_name(&self) -> BoxedColor;
    fn find_indicator(&self) -> BoxedColor;
    fn find_name(&self) -> BoxedColor;
}

/// Without color
pub struct Colorless;
impl Color for Colorless {
    fn paint(&self, text: &str) -> String {
        text.to_owned()
    }
}

/// With color
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

/// Scheme of no color
pub struct ColorlessScheme;
impl ColorScheme for ColorlessScheme {
    fn scan(&self) -> BoxedColor {
        Box::new(Colorless)
    }
    fn skip(&self) -> BoxedColor {
        Box::new(Colorless)
    }
    fn skip_name(&self) -> BoxedColor {
        Box::new(Colorless)
    }
    fn find_indicator(&self) -> BoxedColor {
        Box::new(Colorless)
    }
    fn find_name(&self) -> BoxedColor {
        Box::new(Colorless)
    }
}

/// Scheme of styles
pub struct ColorfulScheme;
impl ColorScheme for ColorfulScheme {
    fn scan(&self) -> BoxedColor {
        Box::new(Colorful::new(Style::default().dimmed()))
    }
    fn skip(&self) -> BoxedColor {
        Box::new(Colorful::new(Style::default().dimmed()))
    }
    fn skip_name(&self) -> BoxedColor {
        Box::new(Colorful::new(Style::default().dimmed().strikethrough()))
    }
    fn find_indicator(&self) -> BoxedColor {
        Box::new(Colorful::new(Style::default()))
    }
    fn find_name(&self) -> BoxedColor {
        Box::new(Colorful::new(Style::default().bold()))
    }
}

pub type BoxedColor = Box<dyn Color>;
pub type BoxedColorScheme = Box<dyn ColorScheme>;
