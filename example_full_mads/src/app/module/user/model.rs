use diesel;

use super::schema::users;

#[derive(Clone, Debug, PartialEq, serde::Serialize, diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
