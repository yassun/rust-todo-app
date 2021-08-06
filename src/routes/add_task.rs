use actix_web::{web, HttpResponse};
use crate::domain::{Todo};
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn add_task(
    task: web::Json<Todo>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse> {
    println!("add_task #{:?}", task);
    insert_task(&pool, &task)
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn insert_task(pool: &PgPool, task: &Todo) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO tasks (id, content, done)
    VALUES ($1, $2, $3)
            "#,
        Uuid::new_v4(),
        task.content,
        task.done
    )
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {:?}", e);
    })?;
    Ok(())
}
