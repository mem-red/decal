use crate::{
    attributes::IntoPaintStack,
    capabilities::*,
    layout::{
        Node,
        NodeKind,
        StencilScope,
        StencilType,
        TextMeta,
        Typography,
    },
    macros::impl_node_builder,
    paint::{
        Appearance,
        IntoResources,
        Resource,
    },
    primitives::Paint,
    text::{
        FontStyle,
        FontWeight,
    },
};
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Text {
    meta: TextMeta,
    layout: Style,
    visual: Appearance,
    typography: Typography,
    resources: Vec<Resource>,
}

impl_node_builder!(
    Text,
    build(this) {
        let mut meta = this.meta;
        meta.typography(this.typography.clone());

        Node::new(
            NodeKind::Text(meta),
            this.layout,
            this.visual,
            Some(this.typography),
            this.resources
        )
    }
);

impl Text {
    pub fn new<S>(content: S) -> Self
    where
        S: IntoText,
    {
        let spans = content.into_text_spans();
        let mut resources = Vec::new();

        for span in &spans {
            resources.extend(span.resources.clone());
        }

        Self {
            meta: TextMeta::new(spans),
            resources,
            ..Default::default()
        }
    }

    pub fn stencil<T>(mut self, value: T) -> Self
    where
        T: IntoPaintStack,
    {
        let paint = value.into_paint_stack();
        self.meta.stencil_paint(paint.clone());
        self.add_resources(paint);
        self
    }

    pub fn stencil_scope(mut self, scope: StencilScope) -> Self {
        self.meta.stencil_scope(scope);
        self
    }

    pub fn stencil_type(mut self, value: StencilType) -> Self {
        self.meta.stencil_type(value);
        self
    }
}

impl Hideable for Text {
    fn hidden(mut self, value: bool) -> Self {
        self.layout.display = if value { Display::None } else { Display::Block };
        self
    }
}

impl Background for Text {}
impl Dimensions for Text {}
impl Margin for Text {}
impl Opacity for Text {}
impl Positioned for Text {}
impl Transformation for Text {}
impl Textual for Text {}
impl SelfAlignment for Text {}
impl Visibility for Text {}
impl FilterEffects for Text {}
impl Blendable for Text {}

//

#[derive(Debug, Clone)]
pub struct TextSpan {
    pub(crate) content: String,
    pub(crate) typography: Typography,
    pub(crate) resources: Vec<Resource>,
    pub(crate) hidden: bool,
}

impl TextSpan {
    pub fn new(content: String) -> Self {
        Self {
            content,
            typography: Default::default(),
            resources: Vec::new(),
            hidden: false,
        }
    }

    pub fn family<T>(mut self, family: T) -> Self
    where
        T: Into<Option<String>>,
    {
        self.typography.family = family.into();
        self
    }

    pub fn size<T>(mut self, size: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.typography.size = size.into();
        self
    }

    pub fn line_height<T>(mut self, line_height: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.typography.line_height = line_height.into();
        self
    }

    pub fn weight<T>(mut self, weight: T) -> Self
    where
        T: Into<Option<FontWeight>>,
    {
        self.typography.weight = weight.into();
        self
    }

    pub fn color<T>(mut self, color: T) -> Self
    where
        T: Into<Option<Paint>>,
    {
        let color = color.into();
        self.typography.color = color.clone();

        if let Some(resources) = color.map(|c| c.into_resources()) {
            self.resources.extend(resources);
        }

        self
    }

    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Option<FontStyle>>,
    {
        self.typography.style = style.into();
        self
    }

    pub fn letter_spacing<T>(mut self, letter_spacing: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.typography.letter_spacing = letter_spacing.into();
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        self.hidden = value;
        self
    }
}

//

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
