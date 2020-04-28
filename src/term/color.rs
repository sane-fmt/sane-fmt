pub trait Color {
    fn paint<'a>(&self, text: &'a str) -> &'a str;
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

    pub struct Colorless;
    impl Color for Colorless {
        fn paint<'a>(&self, text: &'a str) -> &'a str {
            text
        }
    }

    pub struct Colorful<F: Fn(&str) -> &str>(F);
    impl<F> Color for Colorful<F>
    where
        F: Fn(&str) -> &str,
    {
        fn paint<'a>(&self, text: &'a str) -> &'a str {
            self.0(text)
        }
    }
}
