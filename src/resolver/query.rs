use crate::models::*;
use crate::resolver::root::Context;
use diesel::prelude::*;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    fn posts(ctx: &Context) -> FieldResult<Vec<Post>> {
        use crate::schema::posts::dsl::*;
        let conn = &mut ctx.pool.get().unwrap();
        let results: Vec<Post> = posts.load::<Post>(conn).expect("Error loading posts");

        Ok(results)
    }
    fn post(ctx: &Context, post_id: i32) -> FieldResult<Vec<Post>> {
        use crate::schema::posts::dsl::*;
        let conn = &mut ctx.pool.get().unwrap();
        let results: Vec<Post> = posts
            .filter(id.eq(post_id))
            .load::<Post>(conn)
            .expect("Error loading posts");

        Ok(results)
    }
}
