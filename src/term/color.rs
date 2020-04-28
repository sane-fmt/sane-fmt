pub trait Color {
    fn paint(&self, text: &str) -> String;
}

pub struct ColorScheme<C: Color> {
    pub scan: C,
    pub skip: C,
    pub skip_name: C,
    pub find_indicator: C,
    pub find_name: C,
}

pub mod collection {
    use super::Color;
    pub use ansi_term::Style;

    pub struct Colorless;
    impl Color for Colorless {
        fn paint(&self, text: &str) -> String {
            text.to_owned()
        }
    }

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
}
