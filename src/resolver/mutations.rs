use crate::models::*;
use crate::resolver::root::*;
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLInputObject};

#[derive(GraphQLInputObject)]
struct PostUpdate {
    id: i32,
    new_title: String,
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn post(ctx: &Context, title: String) -> FieldResult<Post> {
        use crate::schema::posts;
        let conn = &mut ctx.pool.get().unwrap();
        let p = diesel::insert_into(posts::table)
            .values(NewPost { title })
            .get_result::<Post>(conn)
            .expect("error while creating post");
        Ok(p)
    }

    fn delete(ctx: &Context, post_id: i32) -> FieldResult<Post> {
        use crate::schema::posts::dsl::*;
        let conn = &mut ctx.pool.get().unwrap();
        let p = diesel::delete(posts.filter(id.eq(post_id)))
            .get_result::<Post>(conn)
            .expect("error while deleting post");
        Ok(p)
    }

    fn edit(ctx: &Context, post_info: PostUpdate) -> FieldResult<Post> {
        use crate::schema::posts::dsl::*;
        let conn = &mut ctx.pool.get().unwrap();
        let p = diesel::update(posts.filter(id.eq(post_info.id)))
            .set(title.eq(post_info.new_title))
            .get_result::<Post>(conn)
            .expect("error while updating post");
        Ok(p)
    }
}
