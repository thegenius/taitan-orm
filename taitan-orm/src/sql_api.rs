use crate::result::Result;
// use sqlx::{Connection, Database};
use sqlx::Database;
use taitan_orm_trait::paged_list::PagedList;
use taitan_orm_trait::pagination::Pagination;
use taitan_orm_trait::{
    Entity, Location, Mutation, OrderBy, SelectedEntity, TemplateRecord, Unique,
};
// use tracing::debug;

/**
查询语义设计：
返回值语义：
option: 返回至多1个元素
vec:    返回元素数组，数组可能为空
page:  返回元素分页数组

行级筛选语义：
select: 使用唯一键筛选，对应返回值option
search: 使用索引筛选，对应返回值vec/paged
devour: 不使用条件筛选，对应返回值vec/paged

字段级筛选语义：
selection具备返回all_fields的方法

最终查询api设计：
select(selection, unique) -> option<SE>

search(selection, location, order_by)
search_paged(selection, location, page, order_by)

devour(selection, order_by)
devour_paged(selection, page, order_by)
*/
pub trait SqlApi {
    // type G: SqlGenerator + Sync + Debug;
    // fn get_generator(&mut self) -> &Self::G;

    async fn insert(&mut self, entity: &dyn Entity) -> Result<bool>;
    async fn upsert(&mut self, entity: &dyn Entity) -> Result<bool>;
    async fn update<M: Mutation>(&mut self, mutation: &M, unique: &dyn Unique<Mutation = M>) -> Result<bool>;
    async fn change<M: Mutation>(&mut self, mutation: &M, location: &M::Location) -> Result<u64>;
    async fn delete<M: Mutation>(&mut self, unique: &dyn Unique<Mutation = M>) -> Result<bool>;
    async fn purify(&mut self, location: &dyn Location) -> Result<u64>;

    async fn select<DB: Database, SE, M>(
        &mut self,
        selection: &SE::Selection,
        unique: &dyn Unique<Mutation = M>,
    ) -> Result<Option<SE>>
    where
        M: Mutation,
        SE: SelectedEntity<DB> + Send + Unpin;

    async fn search<DB: Database, SE>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        order_by: &dyn OrderBy,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<DB> + Send + Unpin;

    async fn search_paged<DB: Database, SE>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<PagedList<DB, SE>>
    where
        SE: SelectedEntity<DB> + Send + Unpin;

    /**
    根据表中所有数据
    */
    async fn devour<DB: Database, SE>(
        &mut self,
        selection: &SE::Selection,
        order_by: &dyn OrderBy,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<DB> + Send + Unpin;

    async fn devour_paged<DB: Database, SE>(
        &mut self,
        selection: &SE::Selection,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<PagedList<DB, SE>>
    where
        SE: SelectedEntity<DB> + Send + Unpin;

    async fn count(&mut self, location: &dyn Location) -> Result<u64>;

    async fn count_table(&mut self, table_name: &str) -> Result<u64>;

    async fn execute_by_template(&mut self, template: &dyn TemplateRecord) -> Result<usize>;

    // async fn select_by_template<SE>(
    //     &mut self,
    //     template: &dyn TemplateRecord,
    // ) -> Result<Option<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin;

    // sqlx is not abstract enough

    // async fn _insert(&mut self, entity: &dyn Entity) -> Result<bool>;
    // async fn _upsert(&mut self, entity: &dyn Entity) -> Result<bool>;
    // async fn _update<M: Mutation>(&mut self, mutation: &M, unique: &M::Primary) -> Result<bool>;
    // async fn _change<M: Mutation>(&mut self, mutation: &M, location: &M::Location) -> Result<u64>;
    // async fn _delete(&mut self, unique: &dyn Unique) -> Result<bool>;
    // async fn _purify(&mut self, location: &dyn Location) -> Result<u64>;
    //
    //
    // async fn _select<SE>(
    //     &mut self,
    //     selection: &SE::Selection,
    //     unique: &dyn Unique,
    // ) -> Result<Option<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin;
    //
    // async fn _search<SE>(
    //     &mut self,
    //     selection: &SE::Selection,
    //     location: &dyn Location,
    //     order_by: &dyn OrderBy,
    // ) -> Result<Vec<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin;
    //
    // async fn _search_paged<SE>(
    //     &mut self,
    //     selection: &SE::Selection,
    //     location: &dyn Location,
    //     order_by: &dyn OrderBy,
    //     page: &Pagination,
    // ) -> Result<PagedList<Self::DB, SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin;
    //
    // /**
    // 根据表中所有数据
    // */
    // async fn _devour<SE>(
    //     &mut self,
    //     selection: &SE::Selection,
    //     order_by: &dyn OrderBy,
    // ) -> Result<Vec<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin;
    //
    // async fn _devour_paged<SE>(
    //     &mut self,
    //     selection: &SE::Selection,
    //     order_by: &dyn OrderBy,
    //     page: &Pagination,
    // ) -> Result<PagedList<Self::DB, SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin;
    //
    // async fn _count(&mut self, location: &dyn Location) -> Result<u64>;
    //
    // async fn _count_table(&mut self, table_name: &str) -> Result<u64>;
    //
    // async fn _execute_by_template(&mut self, template: &dyn TemplateRecord) -> Result<usize>;
}
