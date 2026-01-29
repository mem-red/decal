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
        SpreadMethod,
        Stop,
    },
    utils::{
        ElementWriter,
        IsDefault,
        angle_to_line,
    },
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};

type GradientUnit = Length<false, true>;

/// The linear gradient element.
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
    /// Creates a new [`LinearGradient`] instance.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn new() -> Self {
        LinearGradient::default()
    }

    /// Creates a new [`LinearGradient`] oriented at the given angle.
    ///
    /// # Arguments
    /// - `angle`: The angle of the gradient in degrees.
    ///
    /// # Returns
    /// - [`Self`]
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

    /// Creates a gradient flowing from bottom to top.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn top() -> Self {
        Self::new()
            .x1(Length::zero())
            .y1(Length::percent(100.0))
            .x2(Length::zero())
            .y2(Length::zero())
    }

    /// Creates a gradient flowing from left to right.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn right() -> Self {
        Self::new()
    }

    /// Creates a gradient flowing from top to bottom.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn bottom() -> Self {
        Self::new()
            .x1(Length::zero())
            .y1(Length::zero())
            .x2(Length::zero())
            .y2(Length::percent(100.0))
    }

    /// Creates a gradient flowing from right to left.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn left() -> Self {
        Self::new()
            .x1(Length::percent(100.0))
            .y1(Length::zero())
            .x2(Length::zero())
            .y2(Length::zero())
    }

    /// Creates a gradient flowing from bottom-right to top-left.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn top_left() -> Self {
        Self::new()
            .x1(Length::percent(100.0))
            .y1(Length::percent(100.0))
            .x2(Length::zero())
            .y2(Length::zero())
    }

    /// Creates a gradient flowing from bottom-left to top-right.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn top_right() -> Self {
        Self::new()
            .x1(Length::zero())
            .y1(Length::percent(100.0))
            .x2(Length::percent(100.0))
            .y2(Length::zero())
    }

    /// Creates a gradient flowing from top-right to bottom-left.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn bottom_left() -> Self {
        Self::new()
            .x1(Length::percent(100.0))
            .y1(Length::zero())
            .x2(Length::zero())
            .y2(Length::percent(100.0))
    }

    /// Creates a gradient flowing from top-left to bottom-right.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn bottom_right() -> Self {
        Self::new()
            .x1(Length::zero())
            .y1(Length::zero())
            .x2(Length::percent(100.0))
            .y2(Length::percent(100.0))
    }

    /// Adds a color stop to the gradient.
    ///
    /// # Arguments
    /// - `value`: The [`Stop`] value.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn stop<T>(mut self, value: T) -> Self
    where
        T: Into<Stop>,
    {
        self.stops.push(value.into());
        self
    }

    // noinspection DuplicatedCode (used by radial gradient too)
    /// Adds multiple color stops to the gradient.
    ///
    /// # Arguments
    /// - `stops`: Iterable collection of [`Stop`] values.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn stops<I, T>(mut self, stops: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Stop>,
    {
        self.stops.extend(stops.into_iter().map(Into::into));
        self
    }

    // noinspection DuplicatedCode (used by radial gradient too)
    /// Sets the coordinate system used for resolving gradient geometry.
    ///
    /// # Arguments
    /// - `value`: The [`GradientUnits`] value.
    ///
    /// # Returns
    /// - [`Self`]
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

    // noinspection DuplicatedCode (used by radial gradient too)
    /// Applies a transformation to the gradient.
    ///
    /// # Arguments
    /// - `value`: The [`GradientTransform`] to apply.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn transform<T>(mut self, value: T) -> Self
    where
        T: Into<Option<GradientTransform>>,
    {
        self.transform = value.into().unwrap_or_default();
        self
    }

    // noinspection DuplicatedCode (used by radial gradient too)
    /// Sets the color interpolation space used by the gradient.
    ///
    /// # Arguments
    /// - `value`: The [`ColorInterpolation`] space to apply.
    ///
    /// # Returns
    /// - [`Self`]
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
    // noinspection DuplicatedCode (used by radial gradient too)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_xml;

    #[test]
    fn renders_default_gradient() {
        let lg = LinearGradient::new();
        assert_xml(
            lg.to_string(),
            format!(r#"<linearGradient id="{}" />"#, lg.iri()),
        );
    }

    #[test]
    fn renders_to_top() {
        let lg = LinearGradient::top();
        assert_xml(
            lg.to_string(),
            format!(r#"<linearGradient id="{}" y1="100%" x2="0" />"#, lg.iri()),
        );
    }

    #[test]
    fn renders_to_right() {
        let lg = LinearGradient::right();
        assert_xml(
            lg.to_string(),
            format!(r#"<linearGradient id="{}" />"#, lg.iri()),
        );
    }

    #[test]
    fn renders_to_bottom() {
        let lg = LinearGradient::bottom();
        assert_xml(
            lg.to_string(),
            format!(r#"<linearGradient id="{}" x2="0" y2="100%" />"#, lg.iri()),
        );
    }

    #[test]
    fn renders_to_left() {
        let lg = LinearGradient::left();
        assert_xml(
            lg.to_string(),
            format!(r#"<linearGradient id="{}" x1="100%" x2="0" />"#, lg.iri()),
        );
    }

    #[test]
    fn renders_to_top_left() {
        let lg = LinearGradient::top_left();
        assert_xml(
            lg.to_string(),
            format!(
                r#"<linearGradient id="{}" x1="100%" y1="100%" x2="0" />"#,
                lg.iri()
            ),
        );
    }

    #[test]
    fn renders_to_top_right() {
        let lg = LinearGradient::top_right();
        assert_xml(
            lg.to_string(),
            format!(r#"<linearGradient id="{}" y1="100%" />"#, lg.iri()),
        );
    }

    #[test]
    fn renders_to_bottom_left() {
        let lg = LinearGradient::bottom_left();
        assert_xml(
            lg.to_string(),
            format!(
                r#"<linearGradient id="{}" x1="100%" x2="0" y2="100%" />"#,
                lg.iri()
            ),
        );
    }

    #[test]
    fn renders_to_bottom_right() {
        let lg = LinearGradient::bottom_right();
        assert_xml(
            lg.to_string(),
            format!(r#"<linearGradient id="{}" y2="100%" />"#, lg.iri()),
        );
    }

    #[test]
    fn renders_with_angle() {
        let lg = LinearGradient::angle(90.0);
        assert_xml(
            lg.to_string(),
            format!(r#"<linearGradient id="{}" y1="50%" y2="50%" />"#, lg.iri()),
        );
    }

    #[test]
    fn self_closes_when_no_stops() {
        let lg = LinearGradient::new();
        assert_xml(
            lg.to_string(),
            format!(r#"<linearGradient id="{}" />"#, lg.iri()),
        );
    }

    #[test]
    fn renders_stops() {
        let lg = LinearGradient::new()
            .stop(Stop::new().offset(0.0).color("#000"))
            .stop(Stop::new().offset(1.0).color("#fff"));

        assert_xml(
            lg.to_string(),
            format!(
                r#"
<linearGradient id="{}">
    <stop stop-color="rgb(0,0,0)" offset="0" />
    <stop stop-color="rgb(255,255,255)" offset="1" />
</linearGradient>"#,
                lg.iri()
            ),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let gradient_units = GradientUnits::UserSpaceOnUse;
        let spread_method = SpreadMethod::Reflect;
        let color_interpolation = ColorInterpolation::LinearRgb;
        let lg = LinearGradient::new()
            .units(gradient_units)
            .spread_method(spread_method)
            .transform(GradientTransform::new().translate_x(10.0))
            .color_interpolation(color_interpolation);

        assert_xml(
            lg.to_string(),
            format!(
                r#"
<linearGradient
    id="{}"
    gradientUnits="{gradient_units}"
    spreadMethod="{spread_method}"
    gradientTransform="matrix(1 0 0 1 10 0)"
    color-interpolation="{color_interpolation}"
/>
"#,
                lg.iri(),
            ),
        );
    }
}
