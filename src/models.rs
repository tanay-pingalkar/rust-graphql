use crate::schema::*;
use juniper::GraphQLObject;

#[derive(Debug, Queryable, GraphQLObject)]
pub struct Post {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
}
