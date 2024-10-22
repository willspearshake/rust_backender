mod mysql_backend;

pub use crate::mysql_backend::mysql_operations;
use crate::mysql_operations::Person;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/people")]
async fn get_people() -> HttpResponse {
    let all_people = mysql_operations::select_all_people();
    HttpResponse::Ok().json(all_people.unwrap()) 
}

#[post("/people")]
async fn post_people(people: web::Json<Vec<Person>>) -> impl Responder {
    // let persons = vec![
    //     Person { person_id: 1, first_name: Some("Matteo".into()), last_name: Some("Black".into()), address: Some("Strada del sole".into()), city: Some("Trana".to_string()) },
    //     Person { person_id: 2, first_name: Some("Secret".into()), last_name: Some("Agent".into()), address: None, city: None},
    //     Person { person_id: 3, first_name: Some("Marta".into()), last_name: Some("Rossi".into()), address: Some("Via Roma 7".into()), city: Some("Reano".to_string()) },
    //     Person { person_id: 4, first_name: Some("Fake".into()), last_name: Some("Fake".into()), address: Some("Secret street 7".into()), city: Some("Imaginary Town".to_string()) },
    //     Person { person_id: 5, first_name: None, last_name: None, address: None, city: None },
    // ];
    //let people_to_insert: Vec<Person> = people.into_inner();
    let result = mysql_operations::insert_people(people.into_inner());
    format!("{:?}", result)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(greet)
        .service(post_people)
        .service(get_people)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

