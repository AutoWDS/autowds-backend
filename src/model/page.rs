use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T> {
    content: Vec<T>,
    total: u32,
    number: u32,
    size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    ASC,
    DESC,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sort {
    property: String,
    direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageRequest {
    page: u32,
    size: u32,
    sort: Vec<Sort>,
}

impl PageRequest {
    pub fn offset(&self) -> usize {
        (self.page as usize) * (self.size as usize)
    }
    pub fn limit(&self) -> usize {
        self.size as usize
    }
}
