use crate::paint::ResourceIri;
use crate::prelude::Length;
use crate::primitives::{
    PatternContentUnits, PatternTransform, PatternUnits, PreserveAspectRatio, ViewBox,
};
use crate::utils::{ElementWriter, IsDefault};
use std::fmt::{Display, Formatter};

type PatternUnit = Length<false, true>;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct Pattern {
    content: String,
    pattern_units: PatternUnits,
    pattern_content_units: PatternContentUnits,
    x: PatternUnit,
    y: PatternUnit,
    width: PatternUnit,
    height: PatternUnit,
    preserve_aspect_ratio: PreserveAspectRatio,
    view_box: Option<ViewBox>,
    transform: PatternTransform,
}

impl Pattern {
    pub fn new(content: String) -> Self {
        Self {
            content,
            ..Default::default()
        }
    }

    pub fn pattern_units<T>(mut self, value: T) -> Self
    where
        T: Into<Option<PatternUnits>>,
    {
        self.pattern_units = value.into().unwrap_or_default();
        self
    }

    pub fn pattern_content_units<T>(mut self, value: T) -> Self
    where
        T: Into<Option<PatternContentUnits>>,
    {
        self.pattern_content_units = value.into().unwrap_or_default();
        self
    }

    pub fn x<T>(mut self, value: T) -> Self
    where
        T: Into<Option<PatternUnit>>,
    {
        self.x = value.into().unwrap_or_default();
        self
    }

    pub fn y<T>(mut self, value: T) -> Self
    where
        T: Into<Option<PatternUnit>>,
    {
        self.y = value.into().unwrap_or_default();
        self
    }

    pub fn width<T>(mut self, value: T) -> Self
    where
        T: Into<Option<PatternUnit>>,
    {
        self.width = value.into().unwrap_or_default();
        self
    }

    pub fn height<T>(mut self, value: T) -> Self
    where
        T: Into<Option<PatternUnit>>,
    {
        self.height = value.into().unwrap_or_default();
        self
    }

    pub fn preserve_aspect_ratio<T>(mut self, value: T) -> Self
    where
        T: Into<Option<PreserveAspectRatio>>,
    {
        self.preserve_aspect_ratio = value.into().unwrap_or_default();
        self
    }

    pub fn view_box<T>(mut self, value: T) -> Self
    where
        T: Into<Option<ViewBox>>,
    {
        self.view_box = value.into();
        self
    }

    pub fn transform<T>(mut self, value: T) -> Self
    where
        T: Into<Option<PatternTransform>>,
    {
        self.transform = value.into().unwrap_or_default();
        self
    }
}

impl ResourceIri for Pattern {}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "pattern")?
            .attr("id", (self.iri(),))?
            .attr("viewBox", self.view_box.map(|x| (x,)))?
            .attr_if("x", self.x, !self.x.is_zero())?
            .attr_if("y", self.y, !self.y.is_zero())?
            .attr_if("width", self.width, !self.width.is_zero())?
            .attr_if("height", self.height, !self.height.is_zero())?
            .attr_if(
                "preserveAspectRatio",
                (&self.preserve_aspect_ratio,),
                !self.preserve_aspect_ratio.is_default(),
            )?
            .attr_if(
                "patternUnits",
                (&self.pattern_units,),
                !self.pattern_units.is_default(),
            )?
            .attr_if(
                "patternContentUnits",
                (&self.pattern_content_units,),
                !self.pattern_content_units.is_default(),
            )?
            .write(|out| self.transform.write(out, "patternTransform"))?
            .content(|out| out.write_str(self.content.as_str()))?
            .close()
    }
}
