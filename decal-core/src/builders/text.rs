use crate::layout::text::CosmicTextContext;
use crate::layout::{Node, NodeKind};
use cosmic_text::{Attrs, FontSystem, Metrics};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Text {
    meta: TextMeta,
    style: Style,
}

#[derive(Debug, Clone)]
pub(crate) struct TextMeta {
    pub(crate) content: String,
    pub(crate) context: CosmicTextContext,
}

impl Text {
    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        let content = content.into();
        let metrics = Metrics {
            font_size: 24.0,
            line_height: 48.0,
        };
        let mut font_system = FontSystem::new();
        let inter_variable = include_bytes!("../inter.ttf");
        font_system.db_mut().load_font_data(inter_variable.to_vec());
        
        let attrs = Attrs::new();
        let spans: &[(&str, Attrs)] = &[(
            "生活,삶,जिंदगी 😀\n",
            attrs.clone().color(cosmic_text::Color::rgb(0xFF, 0x00, 0x00)),
        )];

        let context = CosmicTextContext::new(metrics, &spans, attrs, &mut font_system);

        Self {
            meta: TextMeta { content, context },
            style: Style::default(),
        }
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Text(self.meta.to_owned()), self.style.to_owned())
    }
}
