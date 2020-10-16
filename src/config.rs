pub struct format {
    pub delimiter: String,
}

impl format {
    pub fn default() -> Self {
        Self {
            delimiter: "(;".to_string(),
        }
    }
    pub fn set(s: String) -> Self {
        Self {
            delimiter: s,
        }
    }
    pub fn get(&self) -> String {
        let r = &self.delimiter;
        r.to_string()
    }
}

