use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}
#[get("/Login")]
async fn login() -> &'static str {
    "Login"
}
#[get("/Register")]
async fn register() -> &'static str {
    "Register"
}
#[get("/Movie")]
async fn movie() -> &'static str {
    "Movie"
}
#[get("/Movie/{id}")]
async fn movie_id() -> &'static str {
    "Movie ID"
}
#[get("Movie/{id}/Review")]
async fn movie_review() -> &'static str {
    "Movie Review"
}
#[get("Movie/{id}/Review/{review_id}")]
async fn movie_review_id() -> &'static str {
    "Movie Review ID"
}
#[get("Movie/{id}/Review/{review_id}/Comment")]
async fn movie_review_comment() -> &'static str {
    "Movie Review Comment"
}
#[get("Movie/{id}/Review/{review_id}/Comment/{comment_id}")]
async fn movie_review_comment_id() -> &'static str {
    "Movie Review Comment ID"
}
#[get("Movie/{id}/Review/{review_id}/Comment/{comment_id}/Reply")]
async fn movie_review_comment_reply() -> &'static str {
    "Movie Review Comment Reply"
}
#[get("Movie/{id}/Review/{review_id}/Comment/{comment_id}/Reply/{reply_id}")]
async fn movie_review_comment_reply_id() -> &'static str {
    "Movie Review Comment Reply ID"
}
#[get("Movie/{id}/Review/{review_id}/Comment/{comment_id}/Reply/{reply_id}/Like")]
async fn movie_review_comment_reply_like() -> &'static str {
    "Movie Review Comment Reply Like"
}
#[get("Movie/{id}/Review/{review_id}/Comment/{comment_id}/Reply/{reply_id}/Like/{like_id}")]
async fn movie_review_comment_reply_like_id() -> &'static str {
    "Movie Review Comment Reply Like ID"
}
#[get("Movie/{id}/Review/{review_id}/Comment/{comment_id}/Reply/{reply_id}/Like/{like_id}/Dislike")]
async fn movie_review_comment_reply_like_dislike() -> &'static str {
    "Movie Review Comment Reply Like Dislike"
}
#[get("Movie/{id}/Review/{review_id}/Comment/{comment_id}/Reply/{reply_id}/Like/{like_id}/Dislike/{dislike_id}")]
async fn movie_review_comment_reply_like_dislike_id() -> &'static str {
    "Movie Review Comment Reply Like Dislike ID"
}



#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    pool.execute(include_str!("../../../api/db/schema.sql"))
    .await
    .map_err(CustomError::new)?;

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
            .service(login)
            .service(register)
            .service(movie)
            .service(movie_id)
            .service(movie_review)
            .service(movie_review_id)
            .service(movie_review_comment)
            .service(movie_review_comment_id)
            .service(movie_review_comment_reply)
            .service(movie_review_comment_reply_id)
            .service(movie_review_comment_reply_like)
            .service(movie_review_comment_reply_like_id)
            .service(movie_review_comment_reply_like_dislike)
            .service(movie_review_comment_reply_like_dislike_id);
    };

    Ok(config.into())
}






