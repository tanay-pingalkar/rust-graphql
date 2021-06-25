# rust-graphql
a crud graphql actix diesel server made in rust

## setup
```
!                   rust/cargo/postgres should have installed                     !
===================================================================================
~ cargo install
===================================================================================
~ cargo install diesel_cli --no-default-features --features postgres 
===================================================================================
~ psql
===================================================================================
~ CREATE DATABASE rust_graphql
===================================================================================
~ echo DATABASE_URL=postgresql://postgres:password@localhost/rust_graphql > .env
===================================================================================
~ diesel migration run
===================================================================================
~ cargo run
===================================================================================
```

open `http://localhost:5000/graphql`


## graphql api
``` graphql
# fetch all posts
{
  posts {
    id
    title
  }
}

# create a post
mutation {
  post(title:"this is cool"){
    id
    title
  }
}

# delete post
mutation {
  delete(id:1){
    id
    title
  }
}

# edit post
mutation {
 edit(postInfo:{
  id:1, 
  newTitle:"new title"
 }){
  id 
  title
 }
}


# get one post
{
  post(postId:3){
    id
    title
  }
}
```
