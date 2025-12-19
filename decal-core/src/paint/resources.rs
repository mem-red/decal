use crate::primitives::Resource;
use hashbrown::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub(crate) struct Resources {
    index_map: HashMap<Resource, usize>,
    resources: Vec<Resource>,
}

impl Resources {
    pub(crate) fn get_or_add_resource(&mut self, resource: Resource) -> usize {
        if let Some(idx) = self.index_map.get(&resource) {
            return *idx;
        }

        let idx = self.resources.len();
        self.index_map.insert(resource, idx);
        self.resources.push(resource);

        idx
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
}

impl Display for Resources {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for resource in &self.resources {
            write!(f, "{resource}")?;
        }

        Ok(())
    }
}
