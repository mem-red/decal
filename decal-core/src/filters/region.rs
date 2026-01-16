use crate::primitives::IntoOptionalLength;
use crate::primitives::Length;
use crate::utils::IsDefault;
use std::fmt::{Display, Formatter};

mod private {
    use crate::primitives::Length;
    use crate::utils::IsDefault;
    use smart_default::SmartDefault;

    #[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
    pub struct FilterRegion {
        #[default(Length::percent(-10.0))]
        pub(super) x: Length<false, true>,
        #[default(Length::percent(-10.0))]
        pub(super) y: Length<false, true>,
        #[default(Length::percent(120.0))]
        pub(super) width: Length<false, true>,
        #[default(Length::percent(120.0))]
        pub(super) height: Length<false, true>,
    }

    impl IsDefault for FilterRegion {}

    pub trait HasFilterRegion: Sized {
        #[allow(private_interfaces)]
        fn region_mut(&mut self) -> &mut FilterRegion;
    }
}

pub(crate) use private::*;

pub trait FilterRegionConfig: private::HasFilterRegion {
    fn x<T>(mut self, x: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.region_mut().x = x.into_optional_length().unwrap_or(Length::percent(-10.0));
        self
    }

    fn y<T>(mut self, y: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.region_mut().y = y.into_optional_length().unwrap_or(Length::percent(-10.0));
        self
    }

    fn width<T>(mut self, width: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.region_mut().width = width
            .into_optional_length()
            .unwrap_or(Length::percent(120.0));
        self
    }

    fn height<T>(mut self, height: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.region_mut().height = height
            .into_optional_length()
            .unwrap_or(Length::percent(120.0));
        self
    }
}

impl Display for FilterRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_default() {
            return Ok(());
        }

        if self.x != Length::percent(-10.0) {
            write!(f, r#" x="{}""#, self.x)?;
        }

        if self.y != Length::percent(-10.0) {
            write!(f, r#" y="{}""#, self.y)?;
        }

        if self.width != Length::percent(120.0) {
            write!(f, r#" width="{}""#, self.width)?;
        }

        if self.height != Length::percent(120.0) {
            write!(f, r#" height="{}""#, self.height)?;
        }

        Ok(())
    }
}
