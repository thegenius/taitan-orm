use crate::paged_info::PagedInfo;
use crate::SelectedEntity;
use sqlx::Database;
use std::marker::PhantomData;
use crate::pagination::Pagination;

#[derive(Clone, Debug)]
pub struct PagedList<DB, T>
where
    DB: Database,
    T: SelectedEntity<DB>,
{
    pub data: Vec<T>,
    pub page: PagedInfo,
    pub _phantom: PhantomData<DB>,
}

impl<DB, T> PagedList<DB, T>
where
    DB: Database,
    T: SelectedEntity<DB>,
{
    pub fn empty(page_size: u64, page_num: u64) -> Self {
        Self {
            page: PagedInfo::empty(page_size, page_num),
            data: Vec::new(),
            _phantom: PhantomData,
        }
    }
}

pub fn build_paged_list<DB: Database, SE>(
    data: Vec<SE>,
    record_count: u64,
    page: &Pagination,
) -> PagedList<DB, SE>
where
    SE: SelectedEntity<DB> + Send + Unpin,
{
    let page_info = PagedInfo {
        page_size: page.page_size,
        page_num: page.page_num,
        page_total: (record_count + page.page_size - 1) / page.page_size, // ceil
        total: record_count,
    };

    PagedList {
        data,
        page: page_info,
        _phantom: PhantomData,
    }
}