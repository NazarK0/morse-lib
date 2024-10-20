#[derive(Debug, PartialEq, Clone)]
pub struct Sound {
    pub frequency: f32,
    pub speed: f32,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            frequency: 450.0,
            speed: 1.0,
        }
    }
}
