use cosmic_text::FontSystem;
use cosmic_text::fontdb::{ID, Source};
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) const DEFAULT_FONT_FAMILY: &'static str = "sans-serif";
pub(crate) const BASE_FONT_SIZE: f32 = 16.0;
pub(crate) const BASE_LINE_HEIGHT: f32 = 22.0;

#[derive(Debug)]
pub struct FontRegistry {
    pub(crate) system: FontSystem,
    pub(crate) aliases: HashMap<String, Vec<ID>>,
    pub(crate) default_family: &'static str,
}

impl FontRegistry {
    pub fn new() -> Self {
        Self {
            system: FontSystem::new(),
            aliases: HashMap::new(),
            default_family: DEFAULT_FONT_FAMILY,
        }
    }

    pub fn load_font<T>(mut self, alias: &str, data: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        self.append_font(alias, data);
        self
    }

    pub fn load_system_fonts(mut self) -> Self {
        self.system.db_mut().load_system_fonts();
        self
    }

    pub fn default_family(mut self, alias: &'static str) -> Self {
        self.default_family = alias;
        self
    }

    pub(crate) fn append_font<T>(&mut self, alias: &str, data: T)
    where
        T: Into<Vec<u8>>,
    {
        let source = Source::Binary(Arc::new(data.into()));
        let ids = self.system.db_mut().load_font_source(source);
        self.aliases.insert(alias.to_string(), ids.to_vec());
    }

    pub(crate) fn get_default_family(&self) -> String {
        self.resolve_family_name(self.default_family)
            .unwrap_or(DEFAULT_FONT_FAMILY.to_string())
    }

    pub(crate) fn resolve_family_name(&self, alias: &str) -> Option<String> {
        let trimmed = alias.trim().to_ascii_lowercase();

        if matches!(
            trimmed.as_str(),
            "sans-serif" | "serif" | "mono" | "monospace"
        ) {
            return Some(trimmed);
        }

        let id = self.aliases.get(alias)?.first()?;
        let face = self.system.db().face(*id)?;
        let (name, _) = face.families.first()?;
        Some(name.to_owned())
    }
}
