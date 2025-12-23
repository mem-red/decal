use crate::paint::ResourceIri;
use crate::prelude::Color;
use crate::primitives::{GradientTransform, GradientUnits, Length, SpreadMethod, Stop};
use crate::utils::IsDefault;
use std::fmt::{Display, Formatter};

type GradientUnit = Length<false, true>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct LinearGradient {
    stops: Vec<Stop>,
    units: GradientUnits,
    x1: GradientUnit,
    y1: GradientUnit,
    x2: GradientUnit,
    y2: GradientUnit,
    spread_method: SpreadMethod,
    transform: GradientTransform,
}

impl Default for LinearGradient {
    fn default() -> Self {
        LinearGradient {
            stops: Vec::new(),
            units: GradientUnits::default(),
            x1: GradientUnit::zero(),
            y1: GradientUnit::zero(),
            x2: GradientUnit::percent(100),
            y2: GradientUnit::zero(),
            spread_method: SpreadMethod::default(),
            transform: GradientTransform::default(),
        }
    }
}

impl LinearGradient {
    pub fn new() -> Self {
        LinearGradient::default()
    }

    pub fn stop<T>(mut self, value: T) -> Self
    where
        T: Into<Stop>,
    {
        self.stops.push(value.into());
        self
    }

    pub fn stops<I, T>(mut self, stops: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Stop>,
    {
        self.stops.extend(stops.into_iter().map(|x| x.into()));
        self
    }

    pub fn units<T>(mut self, value: T) -> Self
    where
        T: Into<Option<GradientUnits>>,
    {
        self.units = value.into().unwrap_or_default();
        self
    }

    pub fn x1<T>(mut self, value: T) -> Self
    where
        T: Into<Option<GradientUnit>>,
    {
        self.x1 = value.into().unwrap_or_default();
        self
    }

    pub fn y1<T>(mut self, value: T) -> Self
    where
        T: Into<Option<GradientUnit>>,
    {
        self.y1 = value.into().unwrap_or_default();
        self
    }

    pub fn x2<T>(mut self, value: T) -> Self
    where
        T: Into<Option<GradientUnit>>,
    {
        self.x2 = value.into().unwrap_or(GradientUnit::percent(100));
        self
    }

    pub fn y2<T>(mut self, value: T) -> Self
    where
        T: Into<Option<GradientUnit>>,
    {
        self.y2 = value.into().unwrap_or_default();
        self
    }

    pub fn spread_method<T>(mut self, value: T) -> Self
    where
        T: Into<Option<SpreadMethod>>,
    {
        self.spread_method = value.into().unwrap_or_default();
        self
    }

    pub fn transform<T>(mut self, value: T) -> Self
    where
        T: Into<Option<GradientTransform>>,
    {
        self.transform = value.into().unwrap_or_default();
        self
    }
}

impl ResourceIri for LinearGradient {}

impl Display for LinearGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"<linearGradient id="{}""#, self.iri())?;

        if !self.x1.is_zero() {
            write!(f, r#" x1="{}""#, self.x1)?;
        }

        if !self.y1.is_zero() {
            write!(f, r#" y1="{}""#, self.y1)?;
        }

        if self.x2 != GradientUnit::percent(100) {
            write!(f, r#" x2="{}""#, self.x2)?;
        }

        if !self.y2.is_zero() {
            write!(f, r#" y2="{}""#, self.y2)?;
        }

        if !self.units.is_default() {
            write!(f, r#" gradientUnits="{}""#, self.units)?;
        }

        if !self.spread_method.is_default() {
            write!(f, r#" spreadMethod="{}""#, self.spread_method)?;
        }

        self.transform.write(f)?;

        if self.stops.is_empty() {
            write!(f, " />")
        } else {
            write!(f, ">")?;

            for stop in &self.stops {
                write!(f, "{stop}")?;
            }

            write!(f, r#"</linearGradient>"#)
        }
    }
}

fn testing() {
    let e = LinearGradient::new().stops([
        (0.1, Color::parse("red"), 0.75),
        (0.4, Color::parse("blue"), 0.75),
        (0.9, Color::parse("yellow"), 0.75),
    ]);
}
