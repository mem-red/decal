use crate::primitives::IntoOptionalLength;
use crate::utils::IsDefault;
use std::fmt::{Display, Formatter};

mod private {
    use crate::prelude::Length;
    use crate::utils::IsDefault;

    #[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
    pub struct FilterRegion {
        pub(super) x: Option<Length<false, true>>,
        pub(super) y: Option<Length<false, true>>,
        pub(super) width: Option<Length<false, true>>,
        pub(super) height: Option<Length<false, true>>,
    }

    impl IsDefault for FilterRegion {}

    pub trait HasFilterRegion: Sized {
        #[allow(private_interfaces)]
        fn region_mut(&mut self) -> &mut FilterRegion;
    }
}

pub(crate) use private::*;

//

pub trait FilterRegionConfig: private::HasFilterRegion {
    fn x<T>(mut self, x: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.region_mut().x = x.into_optional_length();
        self
    }

    fn y<T>(mut self, y: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.region_mut().y = y.into_optional_length();
        self
    }

    fn width<T>(mut self, width: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.region_mut().width = width.into_optional_length();
        self
    }

    fn height<T>(mut self, height: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.region_mut().height = height.into_optional_length();
        self
    }
}

impl Display for FilterRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_default() {
            return Ok(());
        }

        if let Some(x) = self.x {
            write!(f, r#" x="{x}""#)?;
        }

        if let Some(y) = self.y {
            write!(f, r#" y="{y}""#)?;
        }

        if let Some(width) = self.width {
            write!(f, r#" width="{width}""#)?;
        }

        if let Some(height) = self.height {
            write!(f, r#" height="{height}""#)?;
        }

        Ok(())
    }
}
