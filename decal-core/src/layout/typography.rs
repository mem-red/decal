use crate::primitives::Paint;
use crate::text::{FontStyle, FontWeight, TextAlign, TextWrap};

#[derive(Debug, Clone, Default)]
pub(crate) struct Typography {
    pub(crate) family: Option<String>,
    pub(crate) size: Option<f32>,
    pub(crate) line_height: Option<f32>,
    pub(crate) weight: Option<FontWeight>,
    pub(crate) color: Option<Paint>,
    pub(crate) style: Option<FontStyle>,
    pub(crate) letter_spacing: Option<f32>,
    pub(crate) align: Option<TextAlign>,
    pub(crate) wrap: Option<TextWrap>,
    // TODO pub word_spacing: Option<f32>,
    // TODO pub decoration: Option<TextDecoration>,
    // computed
    pub(crate) resolved_family: String,
}

impl Typography {
    //noinspection RsLiveness
    pub(crate) fn cascade_from(&mut self, parent: &Typography) {
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
