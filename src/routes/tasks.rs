#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: Option<u32>,
    content: String,
    done: bool,
}

#[get("/tasks/{id}")]
pub async fn get_task(web::Path(id): web::Path<u32>) -> impl Responder {
    println!("get_tasks");
    let id_option: Option<u32> = Some(id);
    HttpResponse::Ok().json(Task {
        id: id_option,
        content: String::from("TODO"),
        done: false,
    })
}
