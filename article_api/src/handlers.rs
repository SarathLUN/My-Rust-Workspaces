use crate::models::{Article, CreateArticle, UpdateArticle};
use crate::schema::articles::dsl::*;
use actix_web::{web, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("/create_post", web::post().to(create_post))
            .route("/get_post/{uuid}", web::get().to(get_post))
            .route("/list_posts", web::get().to(list_posts))
            .route("/list_all_posts", web::get().to(list_all_posts))
            .route("/list_deleted_posts", web::get().to(list_deleted_posts))
            .route("/update_post/{uuid}", web::put().to(update_post))
            .route("/delete_post/{uuid}", web::delete().to(delete_post))
            .route("/remove_post/{uuid}", web::delete().to(remove_post)),
    );
}

async fn create_post(pool: web::Data<DbPool>, item: web::Json<CreateArticle>) -> HttpResponse {
    let new_article = Article {
        id: Uuid::new_v4(),
        title: item.title.clone(),
        content: item.content.clone(),
        published_at: Utc::now().naive_utc(),
        is_published: item.is_published,
        is_deleted: false,
        deleted_at: None,
    };

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::insert_into(articles)
        .values(&new_article)
        .execute(&mut conn)
        .expect("Error saving new post");

    HttpResponse::Ok().json(new_article.id)
}

async fn get_post(pool: web::Data<DbPool>, article_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let result = articles
        .filter(id.eq(article_id.into_inner()))
        .first::<Article>(&mut conn)
        .optional()
        .expect("Error loading post");

    match result {
        Some(article) => HttpResponse::Ok().json(article),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn list_posts(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let result = articles
        .filter(is_deleted.eq(false))
        .filter(is_published.eq(true))
        .load::<Article>(&mut conn)
        .expect("Error loading posts");

    HttpResponse::Ok().json(result)
}

async fn list_all_posts(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let result = articles
        .filter(is_deleted.eq(false))
        .load::<Article>(&mut conn)
        .expect("Error loading posts");

    HttpResponse::Ok().json(result)
}

async fn list_deleted_posts(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let result = articles
        .filter(is_deleted.eq(true))
        .load::<Article>(&mut conn)
        .expect("Error loading posts");

    HttpResponse::Ok().json(result)
}

async fn update_post(
    pool: web::Data<DbPool>,
    article_id: web::Path<Uuid>,
    item: web::Json<UpdateArticle>,
) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let updated = diesel::update(articles.filter(id.eq(article_id.into_inner())))
        .set(&*item)
        .execute(&mut conn)
        .expect("Error updating post");

    if updated > 0 {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_post(pool: web::Data<DbPool>, article_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let deleted = diesel::delete(articles.filter(id.eq(article_id.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting post");

    if deleted > 0 {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn remove_post(pool: web::Data<DbPool>, article_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let updated = diesel::update(articles.filter(id.eq(article_id.into_inner())))
        .set((
            is_deleted.eq(true),
            deleted_at.eq(Some(Utc::now().naive_utc())),
        ))
        .execute(&mut conn)
        .expect("Error marking post as deleted");

    if updated > 0 {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
