use crate::filters::Filter;
use crate::primitives::{ClipPath, LinearGradient, Paint, Path, Pattern, RadialGradient};
use crate::utils::IsDefault;
use enum_display::EnumDisplay;
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq, Clone, EnumDisplay)]
pub(crate) enum Resource {
    #[display("{0}")]
    LinearGradient(LinearGradient),
    #[display("{0}")]
    RadialGradient(RadialGradient),
    #[display("{0}")]
    Pattern(Pattern),
    #[display("{0}")]
    ClipPath(ClipPath),
    #[display("{0}")]
    Path(Path),
    #[display("{0}")]
    Filter(Filter),
}

pub(crate) trait IntoResources {
    fn into_resources(self) -> Vec<Resource>;
}

impl IntoResources for Vec<Resource> {
    #[inline]
    fn into_resources(self) -> Vec<Resource> {
        self
    }
}

impl IntoResources for Paint {
    #[inline]
    fn into_resources(self) -> Vec<Resource> {
        match self {
            Paint::None | Paint::Color(_) => Vec::new(),
            Paint::LinearGradient(value) => value.into_resources(),
            Paint::RadialGradient(value) => value.into_resources(),
            Paint::Pattern(value) => value.into_resources(),
        }
    }
}

impl IntoResources for Pattern {
    #[inline]
    fn into_resources(self) -> Vec<Resource> {
        vec![Resource::Pattern(self)]
    }
}

impl IntoResources for Filter {
    #[inline]
    fn into_resources(self) -> Vec<Resource> {
        if self.is_default() {
            Vec::new()
        } else {
            vec![Resource::Filter(self)]
        }
    }
}

//

impl From<LinearGradient> for Resource {
    #[inline]
    fn from(value: LinearGradient) -> Self {
        Self::LinearGradient(value)
    }
}

impl From<RadialGradient> for Resource {
    #[inline]
    fn from(value: RadialGradient) -> Self {
        Self::RadialGradient(value)
    }
}

impl From<Pattern> for Resource {
    #[inline]
    fn from(value: Pattern) -> Self {
        Self::Pattern(value)
    }
}

impl From<ClipPath> for Resource {
    #[inline]
    fn from(value: ClipPath) -> Self {
        Self::ClipPath(value)
    }
}

impl From<Path> for Resource {
    #[inline]
    fn from(value: Path) -> Self {
        Self::Path(value)
    }
}

impl From<Filter> for Resource {
    #[inline]
    fn from(value: Filter) -> Self {
        Self::Filter(value)
    }
}
