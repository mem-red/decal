use crate::{
    macros::nf32,
    primitives::Color,
    utils::ElementWriter,
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};
use strict_num::NormalizedF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
pub struct Stop {
    #[default(NormalizedF32::ZERO)]
    offset: NormalizedF32,
    color: Color,
    #[default(NormalizedF32::ONE)]
    opacity: NormalizedF32,
}

impl Stop {
    pub fn new() -> Self {
        Stop::default()
    }

    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = nf32!(offset);
        self
    }

    pub fn offset_pct(self, offset: f32) -> Self {
        self.offset(offset / 100.0);
        self
    }

    pub fn color<T>(mut self, color: T) -> Self
    where
        T: Into<Color>,
    {
        self.color = color.into();
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = nf32!(opacity);
        self
    }
}

impl Display for Stop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "stop")?
            .attr("stop-color", (self.color,))?
            .attr("offset", self.offset)?
            .attr_if(
                "stop-opacity",
                self.opacity,
                self.opacity != NormalizedF32::ONE,
            )?
            .close()
    }
}

impl From<(f32, Color)> for Stop {
    fn from((offset, color): (f32, Color)) -> Self {
        Stop::new().offset(offset).color(color)
    }
}

impl From<(Color, f32)> for Stop {
    fn from((color, opacity): (Color, f32)) -> Self {
        Stop::new().color(color).opacity(opacity)
    }
}

impl From<(f32, Color, f32)> for Stop {
    fn from((offset, color, opacity): (f32, Color, f32)) -> Self {
        Stop::new().offset(offset).color(color).opacity(opacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_xml;

    #[test]
    fn renders() {
        assert_xml(
            Stop::new().to_string(),
            r#"<stop offset="0" stop-color="rgb(0,0,0)" />"#,
        );
    }

    #[test]
    fn renders_with_attrs() {
        assert_xml(
            Stop::new()
                .offset(0.5)
                .color([1, 2, 3])
                .opacity(0.65)
                .to_string(),
            r#"
<stop
    offset="0.5"
    stop-color="rgb(1,2,3)"
    stop-opacity="0.65"
/>
"#,
        );
    }

    //

    #[test]
    fn from_offset_and_color() {
        assert_eq!(
            Stop::from((0.25, Color::rgb(1, 2, 3))),
            Stop::new().offset(0.25).color([1, 2, 3])
        );
    }

    #[test]
    fn from_color_and_opacity() {
        assert_eq!(
            Stop::from((Color::rgb(1, 2, 3), 0.5)),
            Stop::new().opacity(0.5).color([1, 2, 3])
        );
    }

    #[test]
    fn from_offset_color_and_opacity() {
        assert_eq!(
            Stop::from((0.2, Color::rgb(1, 2, 3), 0.5)),
            Stop::new().offset(0.2).opacity(0.5).color([1, 2, 3])
        );
    }
}
