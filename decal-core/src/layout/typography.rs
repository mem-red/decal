use crate::primitives::Paint;
use crate::text::{FontStyle, FontWeight, TextAlign, TextWrap};

#[derive(Debug, Clone, Default)]
pub struct Typography {
    pub family: Option<String>,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub weight: Option<FontWeight>,
    pub color: Option<Paint>,
    pub style: Option<FontStyle>,
    pub letter_spacing: Option<f32>,
    pub align: Option<TextAlign>,
    pub wrap: Option<TextWrap>,
    // TODO pub word_spacing: Option<f32>,
    // TODO pub decoration: Option<TextDecoration>,
    // computed
    pub(crate) resolved_family: String,
}

impl Typography {
    //noinspection RsLiveness
    pub fn cascade_from(&mut self, parent: &Typography) {
        macro_rules! inherit {
            ($field:ident) => {
                if self.$field.is_none() {
                    self.$field = parent.$field.clone();
                }
            };
        }

        inherit!(family);
        inherit!(size);
        inherit!(line_height);
        inherit!(weight);
        inherit!(color);
        inherit!(style);
        inherit!(letter_spacing);
        inherit!(align);
        inherit!(wrap);
    }
}
