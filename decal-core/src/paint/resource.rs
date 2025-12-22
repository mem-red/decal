use crate::primitives::{ClipPath, Filter, LinearGradient, Paint, Pattern, RadialGradient};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub(crate) enum Resource {
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Pattern(Pattern),
    ClipPath(ClipPath),
    Filter(Filter),
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::LinearGradient(value) => write!(f, "{value}"),
            Resource::RadialGradient(value) => write!(f, "{value}"),
            Resource::Pattern(value) => write!(f, "{value}"),
            Resource::ClipPath(value) => write!(f, "{value}"),
            Resource::Filter(value) => write!(f, "{value}"),
        }
    }
}

pub(crate) trait IntoResource {
    fn into_resource(self) -> Option<Resource>;
}

impl IntoResource for Option<Resource> {
    #[inline]
    fn into_resource(self) -> Option<Resource> {
        self
    }
}

impl IntoResource for Resource {
    #[inline]
    fn into_resource(self) -> Option<Resource> {
        Some(self)
    }
}

impl IntoResource for Paint {
    #[inline]
    fn into_resource(self) -> Option<Resource> {
        match self {
            Paint::None | Paint::Color(_) => None,
            Paint::LinearGradient(value) => Some(value.into()),
            Paint::RadialGradient(value) => Some(value.into()),
            Paint::Pattern(value) => Some(value.into()),
        }
    }
}

impl IntoResource for Filter {
    #[inline]
    fn into_resource(self) -> Option<Resource> {
        Some(Resource::Filter(self))
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

impl From<Filter> for Resource {
    #[inline]
    fn from(value: Filter) -> Self {
        Self::Filter(value)
    }
}
