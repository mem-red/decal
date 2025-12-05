macro_rules! impl_text_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn color<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoFill,
            {
                self.typography.color = value.into_fill();
                self
            }

            pub fn font_family<T, S>(&mut self, family: T) -> &mut Self
            where
                T: Into<Option<S>>,
                S: Into<String>,
            {
                self.typography.family = family.into().map(|s| s.into());
                self
            }

            pub fn font_size<T, F>(&mut self, font_size: T) -> &mut Self
            where
                T: Into<Option<F>>,
                F: Into<f64>,
            {
                self.typography.size = font_size.into().map(|x| x.into() as f32);
                self
            }

            pub fn line_height<T, F>(&mut self, line_height: T) -> &mut Self
            where
                T: Into<Option<F>>,
                F: Into<f64>,
            {
                self.typography.line_height = line_height.into().map(|x| x.into() as f32);
                self
            }

            pub fn font_weight<T>(&mut self, font_weight: T) -> &mut Self
            where
                T: Into<Option<crate::text::FontWeight>>,
            {
                self.typography.weight = font_weight.into();
                self
            }

            pub fn letter_spacing<T, F>(&mut self, letter_spacing: T) -> &mut Self
            where
                T: Into<Option<F>>,
                F: Into<f64>,
            {
                self.typography.letter_spacing = letter_spacing.into().map(|x| x.into() as f32);
                self
            }

            pub fn font_style<T>(&mut self, font_style: T) -> &mut Self
            where
                T: Into<Option<crate::text::FontStyle>>,
            {
                self.typography.style = font_style.into();
                self
            }

            pub fn text_align<T>(&mut self, text_align: T) -> &mut Self
            where
                T: Into<Option<crate::text::TextAlign>>,
            {
                self.typography.align = text_align.into();
                self
            }

            pub fn text_wrap<T>(&mut self, text_wrap: T) -> &mut Self
            where
                T: Into<Option<crate::text::TextWrap>>,
            {
                self.typography.wrap = text_wrap.into();
                self
            }
        }
    };
}

pub(crate) use impl_text_methods;
