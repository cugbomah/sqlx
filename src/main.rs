use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Setup database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Create a warp filter for the /user/{id} route
    let user_route = warp::path!("user" / Uuid)
        .and(with_db(pool.clone()))
        .and_then(handle_get_user);

    warp::serve(user_route).run(([0, 0, 0, 0], 3030)).await;
}

fn with_db(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn handle_get_user(id: Uuid, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    match get_user_by_id(&id, &pool).await {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn get_user_by_id(id: &Uuid, pool: &PgPool) -> Result<User, sqlx::Error> {
    // let user = sqlx::query_as(
    //     User,
    //     r#"
    //     SELECT id, role_id, created_by, updated_by, deleted_by, first_name, last_name, email, password, status
    //     --, created_at, updated_at, deleted_at
    //     FROM core_user
    //     WHERE id = $1
    //     "#,
    //     id
    // )
    // .fetch_one(pool)
    // .await?;

    let user: User = sqlx::query_as(r#"
    SELECT id, role_id, created_by, updated_by, deleted_by, first_name, last_name, email, password, status
    --, created_at, updated_at, deleted_at 
    FROM core_user 
    WHERE id = $1
"#)
.bind(id)
.fetch_one(pool)
.await?;
    Ok(user)
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
struct User {
    id: Uuid,
    role_id: Uuid,
    created_by: Option<Uuid>,
    updated_by: Option<Uuid>,
    deleted_by: Option<Uuid>,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    status: Option<bool>,
    // created_at: Option<chrono::NaiveDateTime>,
    // updated_at: Option<chrono::NaiveDateTime>,
    // deleted_at: Option<chrono::NaiveDateTime>,
}
