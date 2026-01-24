use serde::{Deserialize, Serialize};

pub const DEFAULT_PAGE: u64 = 1;
pub const DEFAULT_PAGE_SIZE: u64 = 20;
pub const MAX_PAGE_SIZE: u64 = 100;

fn default_page() -> u64 {
    DEFAULT_PAGE
}

fn default_page_size() -> u64 {
    DEFAULT_PAGE_SIZE
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

impl Pagination {
    pub fn normalize(mut self) -> Self {
        self.page = self.page.max(1);
        self.page_size = self.page_size.clamp(1, MAX_PAGE_SIZE);
        self
    }

    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.page_size
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub count: u64,
    pub total: u64,
    pub page: u64,
    pub page_count: u64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, _count: u64, total: u64, page: u64, page_size: u64) -> Self {
        let count = data.len() as u64;
        let page_count = total / page_size + u64::from(!total.is_multiple_of(page_size));

        Self {
            data,
            count,
            total,
            page,
            page_count,
        }
    }

    pub fn map<U, F>(self, f: F) -> PaginatedResponse<U>
    where
        F: Fn(T) -> U,
    {
        PaginatedResponse::new(
            self.data.into_iter().map(f).collect(),
            self.count,
            self.total,
            self.page,
            self.page_count,
        )
    }
}
