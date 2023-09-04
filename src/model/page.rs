use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T> {
    content: Vec<T>,
    total: u64,
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

impl<T> Page<T> {
    pub fn new(content: Vec<T>, total: u64, page: &PageRequest) -> Self {
        Self {
            content,
            total,
            number: page.page,
            size: page.size,
        }
    }

    pub fn map<B, F>(&self, mapper: F) -> Page<B>
    where
        F: Fn(&T) -> B,
    {
        Page {
            content: self.content.iter().map(mapper).collect(),
            total: self.total,
            number: self.number,
            size: self.size,
        }
    }
}
