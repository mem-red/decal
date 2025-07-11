use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::collections::HashMap;

#[derive(Debug)]
pub struct IdentGen {
    counters: HashMap<String, usize>,
}

impl IdentGen {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    pub fn uniq(&mut self, base: &str) -> Ident {
        let count = self.counters.entry(base.to_string()).or_insert(0);
        *count += 1;
        format_ident!("{}_{}", base, count, span = Span::call_site())
    }
}
