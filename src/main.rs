use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use bson::doc;

/// Represents a user with username and password.
#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

/// Handles user registration.
///
/// # Arguments
///
/// * `user` - User data to be registered.
/// * `collection` - MongoDB collection to store user data.
///
/// # Returns
///
/// Returns a HTTP response indicating the registration status.
async fn register(user: web::Json<User>, collection: web::Data<mongodb::Collection<User>>) -> impl Responder {
    let user = user.into_inner();
    let result = collection.insert_one(user, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
    }
}

/// Handles user login.
///
/// # Arguments
///
/// * `user` - User data for login attempt.
/// * `collection` - MongoDB collection to fetch user data.
///
/// # Returns
///
/// Returns a HTTP response indicating the login status.
async fn login(user: web::Json<User>, collection: web::Data<mongodb::Collection<User>>) -> impl Responder {
    let user = user.into_inner();
    let filter = doc! {"username": &user.username, "password": &user.password};
    let result = collection.find_one(filter, None).await;

    match result {
        Ok(Some(_)) => HttpResponse::Ok().body("Login successful"),
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to login"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create MongoDB client
    let client = Client::with_uri_str("mongodb+srv://oguzhancart1:0DEN82hoeglgUruP@cluster0.ur3pik1.mongodb.net/")
    .await
    .expect("Failed to initialize MongoDB client");


    // Select the 'rust_auth' database
    let db = client.database("rust_auth");

    // Select the 'users' collection
    let collection: mongodb::Collection<User> = db.collection("users");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(collection.clone()))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
