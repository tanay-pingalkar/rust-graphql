use crate::resolver::mutations::MutationRoot;
use crate::resolver::query::QueryRoot;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use juniper::{EmptySubscription, RootNode};

pub struct Context {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
