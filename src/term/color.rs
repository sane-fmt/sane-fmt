pub use ansi_term::Style;

/// Define a single color
pub trait Color {
    fn paint(&self, text: &str) -> String;
}

/// Define a scheme of color
pub trait ColorScheme {
    fn scan(&self) -> Box<dyn Color>;
    fn skip(&self) -> Box<dyn Color>;
    fn skip_name(&self) -> Box<dyn Color>;
    fn find_indicator(&self) -> Box<dyn Color>;
    fn find_name(&self) -> Box<dyn Color>;
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
    fn scan(&self) -> Box<dyn Color> {
        Box::new(Colorless)
    }
    fn skip(&self) -> Box<dyn Color> {
        Box::new(Colorless)
    }
    fn skip_name(&self) -> Box<dyn Color> {
        Box::new(Colorless)
    }
    fn find_indicator(&self) -> Box<dyn Color> {
        Box::new(Colorless)
    }
    fn find_name(&self) -> Box<dyn Color> {
        Box::new(Colorless)
    }
}

/// Scheme of styles
pub struct ColorfulScheme;
impl ColorScheme for ColorfulScheme {
    fn scan(&self) -> Box<dyn Color> {
        Box::new(Colorful::new(Style::default().dimmed()))
    }
    fn skip(&self) -> Box<dyn Color> {
        Box::new(Colorful::new(Style::default().dimmed()))
    }
    fn skip_name(&self) -> Box<dyn Color> {
        Box::new(Colorful::new(Style::default().dimmed().strikethrough()))
    }
    fn find_indicator(&self) -> Box<dyn Color> {
        Box::new(Colorful::new(Style::default()))
    }
    fn find_name(&self) -> Box<dyn Color> {
        Box::new(Colorful::new(Style::default().bold()))
    }
}
