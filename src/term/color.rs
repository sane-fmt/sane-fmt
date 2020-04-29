pub use ansi_term::Style;

/// Define a scheme of color
pub trait ColorScheme {
    fn scan(&self) -> Style;
    fn skip(&self) -> Style;
    fn skip_name(&self) -> Style;
    fn find_indicator(&self) -> Style;
    fn find_name(&self) -> Style;
}

/// Scheme of no color
pub struct ColorlessScheme;
impl ColorScheme for ColorlessScheme {
    fn scan(&self) -> Style {
        Style::new()
    }
    fn skip(&self) -> Style {
        Style::new()
    }
    fn skip_name(&self) -> Style {
        Style::new()
    }
    fn find_indicator(&self) -> Style {
        Style::new()
    }
    fn find_name(&self) -> Style {
        Style::new()
    }
}

/// Scheme of styles
pub struct ColorfulScheme;
impl ColorScheme for ColorfulScheme {
    fn scan(&self) -> Style {
        Style::default().dimmed()
    }
    fn skip(&self) -> Style {
        Style::default().dimmed()
    }
    fn skip_name(&self) -> Style {
        Style::default().dimmed().strikethrough()
    }
    fn find_indicator(&self) -> Style {
        Style::default()
    }
    fn find_name(&self) -> Style {
        Style::default().bold()
    }
}

pub type BoxedColorScheme = Box<dyn ColorScheme>;
