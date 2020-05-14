pub(super) struct Links {
    pub innovs: Vec<usize>,
    pub enabled: Vec<bool>,
    pub weights: Vec<f64>,
}

impl Links {
    pub fn len(&self) -> usize {
        self.innovs.len()
    }
}

impl Clone for Links {
    fn clone(&self) -> Self {
        Self {
            innovs: self.innovs.clone(),
            enabled: self.enabled.clone(),
            weights: self.weights.clone(),
        }
    }
}
