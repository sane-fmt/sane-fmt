use pipe_trait::*;

pub use yansi::{Color, Style};

/// Define a scheme of color
pub trait ColorScheme {
    fn scan(&self) -> Style;
    fn same(&self) -> Style;
    fn diff(&self) -> Style;
    fn diff_line_equal(&self) -> Style;
    fn diff_line_insert(&self) -> Style;
    fn diff_line_delete(&self) -> Style;
}

/// Scheme of no color
pub struct ColorlessScheme;
impl ColorScheme for ColorlessScheme {
    fn scan(&self) -> Style {
        Style::default()
    }
    fn same(&self) -> Style {
        Style::default()
    }
    fn diff(&self) -> Style {
        Style::default()
    }
    fn diff_line_equal(&self) -> Style {
        Style::default()
    }
    fn diff_line_insert(&self) -> Style {
        Style::default()
    }
    fn diff_line_delete(&self) -> Style {
        Style::default()
    }
}

/// Scheme of styles
pub struct ColorfulScheme;
impl ColorScheme for ColorfulScheme {
    fn scan(&self) -> Style {
        Style::default().dimmed()
    }
    fn same(&self) -> Style {
        Color::RGB(64, 255, 64).pipe(Style::new).bold()
    }
    fn diff(&self) -> Style {
        Color::RGB(255, 64, 64).pipe(Style::new).bold()
    }
    fn diff_line_equal(&self) -> Style {
        Style::default().dimmed()
    }
    fn diff_line_insert(&self) -> Style {
        Color::RGB(0, 127, 0).pipe(Style::new)
    }
    fn diff_line_delete(&self) -> Style {
        Color::RGB(127, 0, 0).pipe(Style::new)
    }
}
