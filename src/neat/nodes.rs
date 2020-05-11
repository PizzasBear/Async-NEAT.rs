// use crate::neat::innovs::Innovs;

pub(super) struct Nodes {
    pub input_links: Vec<Vec<usize>>,
    pub indices: Vec<usize>,
}

impl Clone for Nodes {
    fn clone(&self) -> Self {
        Self {
            input_links: self.input_links.clone(),
            indices: self.indices.clone(),
        }
    }
}
