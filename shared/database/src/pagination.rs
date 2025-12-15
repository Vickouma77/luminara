use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page number (1-indexed)
    pub page: u32,

    /// Number of Items per page
    pub per_page: u32,
}

impl Pagination {
    /// Default items per page
    pub const DEFAULT_PER_PAGE: u32 = 20;

    /// Maximum items per page
    pub const MAX_PER_PAGE: u32 = 100;

    /// Create new pagination with validation values
    #[must_use]
    pub fn new(page: u32, per_page: u32) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.clamp(1, Self::MAX_PER_PAGE),
        }
    }

    /// Calculate the SQL offset value
    #[must_use]
    pub fn offset(&self) -> i64 {
        i64::from(self.per_page)
    }

    #[must_use]
    pub fn limit(&self) -> i64 {
        i64::from(self.per_page)
    }

    /// Create pagination for the first page with default size
    #[must_use]
    pub fn first_page() -> Self {
        Self::new(1, Self::DEFAULT_PER_PAGE)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::first_page()
    }
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    /// The items of the current page
    pub items: Vec<T>,
    /// Pagination metadata
    pub metadata: PaginationMetadata,
}

/// Metadata about the paginated results
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PaginationMetadata {
    /// Current page number
    pub page: u32,
    /// Items per page
    pub per_page: u32,
    /// Total number of items across all pages
    pub total_items: u64,
    /// Total number of pages
    pub total_pages: u32,
    /// Whether there is a next page
    pub has_next: bool,
    /// Whether there is a previous page
    pub has_previous: bool,
}

impl<T> PaginatedResult<T> {
    /// Create new paginated results
    #[must_use]
    pub fn new(items: Vec<T>, pagination: Pagination, total_items: u64) -> Self {
        let per_page = u64::from(pagination.per_page);

        let total_pages = u32::try_from(total_items.div_ceil(per_page)).unwrap_or(u32::MAX);

        Self {
            items,
            metadata: PaginationMetadata {
                page: pagination.page,
                per_page: pagination.per_page,
                total_items,
                total_pages,
                has_next: pagination.page < total_pages,
                has_previous: pagination.page > 1,
            },
        }
    }

    /// Map the items to a different type
    pub fn map<F, U>(self, f: F) -> PaginatedResult<U>
    where
        F: FnMut(T) -> U,
    {
        PaginatedResult {
            items: self.items.into_iter().map(f).collect(),
            metadata: self.metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_new_clamps_values() {
        // Valid values pass through
        let p = Pagination::new(3, 10);
        assert_eq!(p.page, 3);
        assert_eq!(p.per_page, 10);

        // Page 0 becomes 1, per_page clamped to max
        let p = Pagination::new(0, 100);
        assert_eq!(p.page, 1);
        assert_eq!(p.per_page, Pagination::MAX_PER_PAGE);
    }

    #[test]
    fn test_pagination_defaults() {
        let p = Pagination::default();
        assert_eq!(p.page, 1);
        assert_eq!(p.per_page, Pagination::DEFAULT_PER_PAGE);
        assert_eq!(p.limit(), i64::from(Pagination::DEFAULT_PER_PAGE));
    }

    #[test]
    fn test_paginated_result_metadata() {
        let result = PaginatedResult::new(vec![1, 2, 3], Pagination::new(2, 5), 15);

        assert_eq!(result.metadata.page, 2);
        assert_eq!(result.metadata.total_pages, 3);
        assert!(result.metadata.has_next);
        assert!(result.metadata.has_previous);
    }

    #[test]
    fn test_paginated_result_map() {
        let result = PaginatedResult::new(vec![1, 2], Pagination::new(1, 10), 2);
        let mapped = result.map(|x| x.to_string());

        assert_eq!(mapped.items, vec!["1", "2"]);
        assert_eq!(mapped.metadata.total_items, 2);
    }
}
