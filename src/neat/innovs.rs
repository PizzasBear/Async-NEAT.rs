pub struct Innovs {
    pub(super) offset: usize,
    pub(super) from_innovs: Vec<usize>,
    pub(super) to_innovs: Vec<usize>,
}

impl Innovs {
    pub fn len(&self) -> usize {
        self.from_innovs.len()
    }
}
