use actix_web::{web, HttpResponse};
use crate::domain::{Todo};

pub async fn tasks(id: web::Path<u32>) -> HttpResponse {
    println!("get_tasks id:{}", id);

    let id_option: Option<u32> = Some(id.into_inner());
    HttpResponse::Ok().json(Todo {
        id: id_option,
        content: String::from("TODO"),
        done: false,
    })
}
