use crate::paint::{IntoResources, Resource, ResourceIri};
use crate::primitives::{
    ColorInterpolation, GradientTransform, GradientUnits, IntoOptionalLength, Length, SpreadMethod,
    Stop,
};
use crate::utils::{ElementWriter, IsDefault, angle_to_line};
use smart_default::SmartDefault;
use std::fmt::{Display, Formatter};

type GradientUnit = Length<false, true>;

#[derive(Debug, Hash, Eq, PartialEq, Clone, SmartDefault)]
pub struct LinearGradient {
    x1: GradientUnit,
    y1: GradientUnit,
    #[default(GradientUnit::percent(100.0))]
    x2: GradientUnit,
    y2: GradientUnit,
    stops: Vec<Stop>,
    units: GradientUnits,
    spread_method: SpreadMethod,
    transform: GradientTransform,
    #[default(ColorInterpolation::SRgb)]
    color_interpolation: ColorInterpolation,
}

impl LinearGradient {
    pub fn new() -> Self {
        LinearGradient::default()
    }

    pub fn angle(angle: f32) -> Self {
        let (x1, y1, x2, y2) = angle_to_line(angle);
        LinearGradient {
            x1: GradientUnit::percent_normalized(x1),
            y1: GradientUnit::percent_normalized(y1),
            x2: GradientUnit::percent_normalized(x2),
            y2: GradientUnit::percent_normalized(y2),
            ..Default::default()
        }
    }

    pub fn top() -> Self {
        Self::new()
            .x1(Length::zero())
            .y1(Length::percent(100.0))
            .x2(Length::zero())
            .y2(Length::zero())
    }

    pub fn right() -> Self {
        Self::new()
    }

    pub fn bottom() -> Self {
        Self::new()
            .x1(Length::zero())
            .y1(Length::zero())
            .x2(Length::zero())
            .y2(Length::percent(100.0))
    }

    pub fn left() -> Self {
        Self::new()
            .x1(Length::percent(100.0))
            .y1(Length::zero())
            .x2(Length::zero())
            .y2(Length::zero())
    }

    pub fn top_left() -> Self {
        Self::new()
            .x1(Length::percent(100.0))
            .y1(Length::percent(100.0))
            .x2(Length::zero())
            .y2(Length::zero())
    }

    pub fn top_right() -> Self {
        Self::new()
            .x1(Length::zero())
            .y1(Length::percent(100.0))
            .x2(Length::percent(100.0))
            .y2(Length::zero())
    }

    pub fn bottom_left() -> Self {
        Self::new()
            .x1(Length::percent(100.0))
            .y1(Length::zero())
            .x2(Length::zero())
            .y2(Length::percent(100.0))
    }

    pub fn bottom_right() -> Self {
        Self::new()
            .x1(Length::zero())
            .y1(Length::zero())
            .x2(Length::percent(100.0))
            .y2(Length::percent(100.0))
    }

    //

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
        self.stops.extend(stops.into_iter().map(Into::into));
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
        T: IntoOptionalLength<false, true>,
    {
        self.x1 = value.into_optional_length().unwrap_or_default();
        self
    }

    pub fn y1<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.y1 = value.into_optional_length().unwrap_or_default();
        self
    }

    pub fn x2<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.x2 = value
            .into_optional_length()
            .unwrap_or(GradientUnit::percent(100.0));
        self
    }

    pub fn y2<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.y2 = value.into_optional_length().unwrap_or_default();
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

    pub fn color_interpolation<T>(mut self, value: T) -> Self
    where
        T: Into<Option<ColorInterpolation>>,
    {
        self.color_interpolation = value.into().unwrap_or(ColorInterpolation::SRgb);
        self
    }
}

impl ResourceIri for LinearGradient {}

impl IntoResources for LinearGradient {
    fn into_resources(self) -> Vec<Resource> {
        vec![self.into()]
    }
}

impl Display for LinearGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let gradient = ElementWriter::new(f, "linearGradient")?
            .attr("id", (self.iri(),))?
            .attr_if("x1", self.x1, !self.x1.is_zero())?
            .attr_if("y1", self.y1, !self.y1.is_zero())?
            .attr_if("x2", self.x2, self.x2 != GradientUnit::percent(100.0))?
            .attr_if("y2", self.y2, !self.y2.is_zero())?
            .attr_if("gradientUnits", (&self.units,), !self.units.is_default())?
            .attr_if(
                "spreadMethod",
                (&self.spread_method,),
                !self.spread_method.is_default(),
            )?
            .attr_if(
                "color-interpolation",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::SRgb,
            )?
            .write(|out| self.transform.write(out, "gradientTransform"))?;

        if self.stops.is_empty() {
            gradient.close()
        } else {
            gradient
                .content(|out| self.stops.iter().try_for_each(|stop| stop.fmt(out)))?
                .close()
        }
    }
}
