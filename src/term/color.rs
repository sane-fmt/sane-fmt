pub use ansi_term::{Color, Style};

/// Define a scheme of color
pub trait ColorScheme {
    fn scan(&self) -> Style;
    fn skip(&self) -> Style;
    fn skip_name(&self) -> Style;
    fn same(&self) -> Style;
    fn diff(&self) -> Style;
    fn diff_line_same(&self) -> Style;
    fn diff_line_add(&self) -> Style;
    fn diff_line_rem(&self) -> Style;
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
    fn same(&self) -> Style {
        Style::new()
    }
    fn diff(&self) -> Style {
        Style::new()
    }
    fn diff_line_same(&self) -> Style {
        Style::new()
    }
    fn diff_line_add(&self) -> Style {
        Style::new()
    }
    fn diff_line_rem(&self) -> Style {
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
        Style::default().bold().dimmed()
    }
    fn skip_name(&self) -> Style {
        self.skip().strikethrough()
    }
    fn same(&self) -> Style {
        Color::RGB(64, 255, 64).bold()
    }
    fn diff(&self) -> Style {
        Color::RGB(255, 64, 64).bold()
    }
    fn diff_line_same(&self) -> Style {
        Style::default().dimmed()
    }
    fn diff_line_add(&self) -> Style {
        Color::RGB(0, 127, 0).into()
    }
    fn diff_line_rem(&self) -> Style {
        Color::RGB(127, 0, 0).into()
    }
}

pub type BoxedColorScheme = Box<dyn ColorScheme>;
