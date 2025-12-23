use crate::paint::ResourceIri;
use crate::prelude::Color;
use crate::primitives::{GradientTransform, Length, Stop};
use std::fmt::{Display, Formatter};

type GradientUnit = Length<false, true>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct LinearGradient {
    stops: Vec<Stop>,
    x1: GradientUnit,
    y1: GradientUnit,
    x2: GradientUnit,
    y2: GradientUnit,
    transform: GradientTransform,
}

impl Default for LinearGradient {
    fn default() -> Self {
        LinearGradient {
            stops: Vec::new(),
            x1: GradientUnit::zero(),
            y1: GradientUnit::zero(),
            x2: GradientUnit::percent(100),
            y2: GradientUnit::zero(),
            transform: GradientTransform::default(),
        }
    }
}

impl LinearGradient {
    // pub fn new() -> Self {
    //     LinearGradient::default()
    // }

    pub fn new() -> Self {
        LinearGradient {
            stops: vec![
                Stop::new()
                    .offset(0.1)
                    .color(Color::parse("red"))
                    .opacity(0.5),
                Stop::new()
                    .offset(0.4)
                    .color(Color::parse("blue"))
                    .opacity(0.5),
                Stop::new()
                    .offset(0.9)
                    .color(Color::parse("yellow"))
                    .opacity(0.5),
            ],
            ..Default::default()
        }
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

    // TODO: use trait here
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
