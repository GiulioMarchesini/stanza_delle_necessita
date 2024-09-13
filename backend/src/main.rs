use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    total_time: u64, // in minuti
}

#[derive(Serialize, Deserialize, Debug)]
struct RoomState {
    status: String,
    current_user: Option<String>,
}
#[derive(Debug)]
struct AppState {
    room: Mutex<RoomState>,
    leaderboard: Mutex<Vec<User>>,
}

#[post("/occupy_room")]
async fn occupy_room(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut room = data.room.lock().unwrap();

    if room.status == "libera" {
        room.status = "occupata".to_string();
        room.current_user = Some(user.username.clone());
        println!("stanza occupata da User: {:?}", user);
        HttpResponse::Ok().json("Room occupied")
    } else {
        HttpResponse::Ok().json("Room is currently occupied")
    }
}

#[post("/free_room")]
async fn free_room(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut room = data.room.lock().unwrap();

    if room.status == "occupata" && room.current_user.as_ref() == Some(&user.username) {
        room.status = "libera".to_string();
        room.current_user = None;
        println!("stanza liberata da User: {:?}", user);
        println!("{:?}", data);
        update_leaderboard(&user.username, 5, &data); // Aggiungi 5 minuti come esempio
        HttpResponse::Ok().json("Room freed")
    } else {
        HttpResponse::Ok().json("You cannot free the room")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        room: Mutex::new(RoomState {
            status: "libera".to_string(),
            current_user: None,
        }),
        leaderboard: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(app_state.clone())
            .service(occupy_room)
            .service(free_room)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn update_leaderboard(username: &str, time: u64, data: &web::Data<AppState>) {
    let mut leaderboard = data.leaderboard.lock().unwrap();
    if let Some(user) = leaderboard.iter_mut().find(|u| u.username == username) {
        user.total_time += time;
    } else {
        leaderboard.push(User {
            username: username.to_string(),
            total_time: time,
        });
    }
}
