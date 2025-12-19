use crate::primitives::Resource;
use hashbrown::HashMap;

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

    pub(crate) fn get_resources(&self) -> &Vec<Resource> {
        self.resources.as_ref()
    }
}
