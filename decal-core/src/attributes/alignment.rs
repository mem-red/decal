#[derive(Debug, Copy, Clone)]
pub enum AlignItems {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Debug, Copy, Clone)]
pub enum AlignContent {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceEvenly,
    SpaceAround,
}

pub type JustifyContent = AlignContent;
pub type JustifyItems = AlignItems;
pub type AlignSelf = AlignItems;
pub type JustifySelf = AlignItems;

impl Into<taffy::AlignItems> for AlignItems {
    fn into(self) -> taffy::AlignItems {
        match self {
            AlignItems::Start => taffy::AlignItems::Start,
            AlignItems::End => taffy::AlignItems::End,
            AlignItems::FlexStart => taffy::AlignItems::FlexStart,
            AlignItems::FlexEnd => taffy::AlignItems::FlexEnd,
            AlignItems::Center => taffy::AlignItems::Center,
            AlignItems::Baseline => taffy::AlignItems::Baseline,
            AlignItems::Stretch => taffy::AlignItems::Stretch,
        }
    }
}

impl Into<taffy::AlignContent> for AlignContent {
    fn into(self) -> taffy::AlignContent {
        match self {
            AlignContent::Start => taffy::AlignContent::Start,
            AlignContent::End => taffy::AlignContent::End,
            AlignContent::FlexStart => taffy::AlignContent::FlexStart,
            AlignContent::FlexEnd => taffy::AlignContent::FlexEnd,
            AlignContent::Center => taffy::AlignContent::Center,
            AlignContent::Stretch => taffy::AlignContent::Stretch,
            AlignContent::SpaceBetween => taffy::AlignContent::SpaceBetween,
            AlignContent::SpaceAround => taffy::AlignContent::SpaceAround,
            AlignContent::SpaceEvenly => taffy::AlignContent::SpaceEvenly,
        }
    }
}
