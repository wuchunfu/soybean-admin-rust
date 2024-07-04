use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct PageRequest {
    #[serde(default = "default_current")]
    pub current: u32,
    #[serde(default = "default_size")]
    pub size: u32,
}

fn default_current() -> u32 {
    1
}

fn default_size() -> u32 {
    10
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            current: default_current(),
            size: default_size(),
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct PaginatedData<T> {
    pub current: u32,
    pub size: u32,
    pub total: u32,
    pub records: Vec<T>,
}
