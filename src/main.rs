use axum::routing::get;
use axum::Router;
use axum::response::IntoResponse;
use axum::response::Json;
use serde::Serialize;
use axum::extract::Path;
use sqlx::MySqlPool;
use axum::extract::State;
// use serde_json::json;

mod database;
use database::database_connection;

#[derive(Serialize, Clone)]
struct Student {
    name: String,
    age: i32
}


#[tokio::main]
async fn main()
{
    let db = database_connection().await.expect("Faild to database connection");

    let app = Router::new()
                        .route("/", get(greeting))
                        .route("/hadi", get(|| async {"my name is hadiuzzaman"}))
                        .route("/info", get(|| async { info().await}))
                        .route("/new-office", get(|| async {newOffice().await}))
                        .route("/number", get(|| async {number_return().await}))
                        .route("/res-test/{id}", get(res_test))
                        .route("/db", get(db_data))
                        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server is runing port: 3000");

    axum::serve(listener, app).await.unwrap();
}

async fn greeting()-> impl IntoResponse
{
    return "Hello world";
}


async fn info()->&'static str{
    "my name is hadiuzzaman hadi. i am a student . and i am working for my dream"
}

async fn newOffice()->String
{
    String::from("active it zone limited")
}

async fn number_return()-> impl IntoResponse
{
    let s1 =  Student{ name: "Babor".to_string(), age: 25};
    Json(s1)
}

async fn res_test(Path(user_id): Path<u32>) -> impl IntoResponse
{
    // format!("Requested user id: {}", user_id)

    let s2 = Student{name: "rayhan".to_string(), age: 20};

    let student = vec![
        Student{name: "Shaown".to_string(), age: 25},
        Student{name: "sohel".to_string(), age: 25}
    ];

    if(user_id > 2){
        return ("user not found").into_response();
    }

    return Json(student[user_id as usize].clone()).into_response();
}

async fn db_data(State(_db): State<MySqlPool>)-> impl IntoResponse
{
    "hello frox asxum server"
}