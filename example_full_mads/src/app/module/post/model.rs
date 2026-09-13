use diesel;

use super::schema::posts;

#[derive(Clone, Debug, serde::Serialize, diesel::Queryable, diesel::Selectable, PartialEq)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(diesel::AsChangeset)]
#[diesel(table_name = posts)]
pub struct PostChangeset {
    pub title: Option<String>,
    pub body: Option<String>,
}
