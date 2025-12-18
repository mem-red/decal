use crate::capabilities::*;
use crate::layout::TextMeta;
use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use crate::primitives::Paint;
use crate::text::{FontStyle, FontWeight};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Text {
    meta: TextMeta,
    layout: Style,
    visual: Appearance,
    typography: Typography,
}

impl_node_builder!(
    Text,
    build(this) {
        let mut meta = this.meta.to_owned();
        meta.set_typography(this.typography.to_owned());

        Node::new(
            NodeKind::Text(meta),
            this.layout.to_owned(),
            this.visual.to_owned(),
            Some(this.typography.to_owned()),
        )
    }
);

impl Text {
    pub fn new<S>(content: S) -> Self
    where
        S: IntoText,
    {
        Self {
            meta: TextMeta::new(content.into_text_spans()),
            layout: Default::default(),
            visual: Default::default(),
            typography: Default::default(),
        }
    }
}

impl Hideable for Text {
    fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value { Display::None } else { Display::Block };
        self
    }
}

impl Dimensions for Text {}
impl Margin for Text {}
impl Opacity for Text {}
impl Positioned for Text {}
impl Transformation for Text {}
impl Textual for Text {}
impl SelfAlignment for Text {}
impl Visibility for Text {}

#[derive(Debug, Clone)]
pub struct TextSpan {
    pub(crate) content: String,
    pub(crate) typography: Typography,
    pub(crate) hidden: bool,
}

impl TextSpan {
    pub fn new(content: String) -> Self {
        Self {
            content,
            typography: Default::default(),
            hidden: false,
        }
    }

    pub fn family<T>(&mut self, family: T) -> &mut Self
    where
        T: Into<Option<String>>,
    {
        self.typography.family = family.into();
        self
    }

    pub fn size<T>(&mut self, size: T) -> &mut Self
    where
        T: Into<Option<f32>>,
    {
        self.typography.size = size.into();
        self
    }

    pub fn line_height<T>(&mut self, line_height: T) -> &mut Self
    where
        T: Into<Option<f32>>,
    {
        self.typography.line_height = line_height.into();
        self
    }

    pub fn weight<T>(&mut self, weight: T) -> &mut Self
    where
        T: Into<Option<FontWeight>>,
    {
        self.typography.weight = weight.into();
        self
    }

    pub fn color<T>(&mut self, color: T) -> &mut Self
    where
        T: Into<Option<Paint>>,
    {
        self.typography.color = color.into();
        self
    }

    pub fn style<T>(&mut self, style: T) -> &mut Self
    where
        T: Into<Option<FontStyle>>,
    {
        self.typography.style = style.into();
        self
    }

    pub fn letter_spacing<T>(&mut self, letter_spacing: T) -> &mut Self
    where
        T: Into<Option<f32>>,
    {
        self.typography.letter_spacing = letter_spacing.into();
        self
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.hidden = value;
        self
    }
}

pub trait IntoTextSpan {
    fn into_text_span(self) -> TextSpan;
}

impl IntoTextSpan for &str {
    fn into_text_span(self) -> TextSpan {
        TextSpan::new(self.to_string())
    }
}

impl IntoTextSpan for String {
    fn into_text_span(self) -> TextSpan {
        TextSpan::new(self)
    }
}

impl IntoTextSpan for TextSpan {
    fn into_text_span(self) -> TextSpan {
        self
    }
}

pub trait IntoText {
    fn into_text_spans(self) -> Vec<TextSpan>;
}

impl IntoText for &str {
    fn into_text_spans(self) -> Vec<TextSpan> {
        vec![self.into_text_span()]
    }
}

impl IntoText for String {
    fn into_text_spans(self) -> Vec<TextSpan> {
        vec![self.into_text_span()]
    }
}

impl<T> IntoText for Vec<T>
where
    T: IntoTextSpan,
{
    fn into_text_spans(self) -> Vec<TextSpan> {
        self.into_iter().map(|x| x.into_text_span()).collect()
    }
}
