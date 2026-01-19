use crate::{
    filters::PrimitiveNode,
    paint::Iri,
};
use enum_display::EnumDisplay;
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, EnumDisplay)]
enum FilterInputInner {
    #[display("SourceGraphic")]
    SourceGraphic,
    #[display("SourceAlpha")]
    SourceAlpha,
    #[display("BackgroundImage")]
    BackgroundImage,
    #[display("{0}")]
    Reference(Iri),
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct FilterInput(FilterInputInner);

impl FilterInput {
    pub const fn source_graphic() -> Self {
        Self(FilterInputInner::SourceGraphic)
    }

    pub const fn source_alpha() -> Self {
        Self(FilterInputInner::SourceAlpha)
    }

    pub const fn background_image() -> Self {
        Self(FilterInputInner::BackgroundImage)
    }

    pub(crate) const fn reference(iri: Iri) -> Self {
        Self(FilterInputInner::Reference(iri))
    }
}

impl Display for FilterInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PrimitiveNode> for FilterInput {
    fn from(value: PrimitiveNode) -> Self {
        Self::reference(value.iri())
    }
}
