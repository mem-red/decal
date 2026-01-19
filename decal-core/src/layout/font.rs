use cosmic_text::{
    FontSystem,
    SwashCache,
    fontdb::{
        ID,
        Source,
    },
};
use hashbrown::HashMap;
use smart_default::SmartDefault;
use std::sync::Arc;

pub(crate) const DEFAULT_FONT_FAMILY: &'static str = "sans-serif";
pub(crate) const BASE_FONT_SIZE: f32 = 16.0;
pub(crate) const BASE_LINE_HEIGHT: f32 = 22.0;

#[derive(Debug, SmartDefault)]
pub struct FontRegistry {
    #[default(FontSystem::new())]
    pub(crate) system: FontSystem,
    pub(crate) aliases: HashMap<String, Vec<ID>>,
    pub(crate) alias_cache: HashMap<String, String>,
    #[default(SwashCache::new())]
    pub(crate) swash_cache: SwashCache,
    #[default(DEFAULT_FONT_FAMILY)]
    pub(crate) default_family: &'static str,
}

impl FontRegistry {
    pub fn new() -> Self {
        Self::default()
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

    pub(crate) fn get_default_family(&mut self) -> String {
        self.resolve_family_name(self.default_family)
            .unwrap_or(DEFAULT_FONT_FAMILY.to_string())
    }

    pub(crate) fn resolve_family_name(&mut self, alias: &str) -> Option<String> {
        let alias = alias.trim();
        let lower = alias.to_ascii_lowercase();

        if matches!(
            lower.as_str(),
            "sans-serif" | "serif" | "mono" | "monospace"
        ) {
            return Some(lower);
        }

        if let Some(name) = self.alias_cache.get(alias) {
            return Some(name.clone());
        }

        let id = self.aliases.get(alias)?.first()?;
        let face = self.system.db().face(*id)?;
        let (name, _) = face.families.first()?;
        self.alias_cache.insert(alias.to_string(), name.clone());

        Some(name.to_owned())
    }
}
