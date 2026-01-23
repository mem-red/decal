use crate::{
    paint::{
        IntoResources,
        Resource,
        ResourceIri,
    },
    primitives::{
        ColorInterpolation,
        GradientTransform,
        GradientUnits,
        IntoOptionalLength,
        Length,
        Stop,
    },
    utils::{
        ElementWriter,
        IsDefault,
    },
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};

type GradientUnit = Length<false, true>;

#[derive(Debug, Hash, Eq, PartialEq, Clone, SmartDefault)]
pub struct RadialGradient {
    #[default(GradientUnit::percent(50.0))]
    cx: GradientUnit,
    #[default(GradientUnit::percent(50.0))]
    cy: GradientUnit,
    fx: Option<GradientUnit>,
    fy: Option<GradientUnit>,
    fr: GradientUnit,
    #[default(GradientUnit::percent(50.0))]
    r: GradientUnit,
    stops: Vec<Stop>,
    units: GradientUnits,
    transform: GradientTransform,
    #[default(ColorInterpolation::SRgb)]
    color_interpolation: ColorInterpolation,
}

impl RadialGradient {
    pub fn new() -> Self {
        RadialGradient::default()
    }

    pub fn r<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.r = value
            .into_optional_length()
            .unwrap_or(GradientUnit::percent(50.0));
        self
    }

    pub fn cx<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.cx = value
            .into_optional_length()
            .unwrap_or(GradientUnit::percent(50.0));
        self
    }

    pub fn cy<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.cy = value
            .into_optional_length()
            .unwrap_or(GradientUnit::percent(50.0));
        self
    }

    pub fn fx<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.fx = value.into_optional_length();
        self
    }

    pub fn fy<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.fy = value.into_optional_length();
        self
    }

    pub fn fr<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength<false, true>,
    {
        self.fr = value.into_optional_length().unwrap_or_default();
        self
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

impl ResourceIri for RadialGradient {}

impl IntoResources for RadialGradient {
    fn into_resources(self) -> Vec<Resource> {
        vec![self.into()]
    }
}

impl Display for RadialGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let gradient = ElementWriter::new(f, "radialGradient")?
            .attr("id", (self.iri(),))?
            .attr_if("r", self.r, self.r != GradientUnit::percent(50.0))?
            .attr_if("cx", self.cx, self.cx != GradientUnit::percent(50.0))?
            .attr_if("cy", self.cy, self.cy != GradientUnit::percent(50.0))?
            .attrs([("fx", self.fx), ("fy", self.fy)])?
            .attr_if("fr", self.fr, !self.fr.is_zero())?
            .attr_if("gradientUnits", (&self.units,), !self.units.is_default())?
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_xml;

    #[test]
    fn renders_default_gradient() {
        let lg = RadialGradient::new();
        assert_xml(
            lg.to_string(),
            format!(r#"<radialGradient id="{}" />"#, lg.iri()),
        );
    }

    #[test]
    fn self_closes_when_no_stops() {
        let lg = RadialGradient::new();
        assert_xml(
            lg.to_string(),
            format!(r#"<radialGradient id="{}" />"#, lg.iri()),
        );
    }

    #[test]
    fn renders_stops() {
        let lg = RadialGradient::new()
            .stop(Stop::new().offset(0.0).color("#000"))
            .stop(Stop::new().offset(1.0).color("#fff"));

        assert_xml(
            lg.to_string(),
            format!(
                r#"
<radialGradient id="{}">
    <stop stop-color="rgb(0,0,0)" offset="0" />
    <stop stop-color="rgb(255,255,255)" offset="1" />
</radialGradient>"#,
                lg.iri()
            ),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let gradient_units = GradientUnits::UserSpaceOnUse;
        let color_interpolation = ColorInterpolation::LinearRgb;
        let lg = RadialGradient::new()
            .r(25.0)
            .cx(10.0)
            .cy(15.0)
            .fx(4.5)
            .fy(6.5)
            .fr(8.5)
            .units(gradient_units)
            .transform(GradientTransform::new().translate_x(10.0))
            .color_interpolation(color_interpolation);

        assert_xml(
            lg.to_string(),
            format!(
                r#"
<radialGradient
    id="{}"
    r="25"
    cx="10"
    cy="15"
    fx="4.5"
    fy="6.5"
    fr="8.5"
    gradientUnits="{gradient_units}"
    gradientTransform="matrix(1 0 0 1 10 0)"
    color-interpolation="{color_interpolation}"
/>
"#,
                lg.iri(),
            ),
        );
    }
}
