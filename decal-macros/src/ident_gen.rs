use hashbrown::HashMap;
use proc_macro2::{Ident, Span};
use quote::format_ident;

#[derive(Debug)]
pub(crate) struct IdentGen {
    counters: HashMap<String, usize>,
}

impl IdentGen {
    pub(crate) fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    pub(crate) fn uniq(&mut self, base: &str) -> Ident {
        let count = self.counters.entry(base.to_string()).or_insert(0);
        *count += 1;
        format_ident!("{}_{}", base, count, span = Span::call_site())
    }
}
