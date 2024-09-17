use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    total_time: u64, // in minuti
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RoomState {
    is_free: bool,
    current_user: Option<String>,
    start_time: Option<u64>, // timestamp in secondi
    end_time: Option<u64>,   // timestamp in secondi
}
#[derive(Debug)]
struct AppState {
    room: Mutex<RoomState>,
    leaderboard: Mutex<Vec<User>>,
}

#[post("/occupy_room")]
async fn occupy_room(data: web::Data<AppState>, new_room: web::Json<RoomState>) -> impl Responder {
    let mut room = data.room.lock().unwrap();
    let actix_web::web::Json(RoomState {
        current_user,
        start_time,
        ..
    }) = new_room;

    if room.is_free {
        // controlla che current_user e start_time siano presenti
        if current_user.is_none() || start_time.is_none() {
            return HttpResponse::BadRequest().json("current_user and start_time are required");
        }

        room.is_free = false;
        println!("stanza occupata da User: {:?}", current_user);
        room.current_user = current_user;
        room.start_time = start_time;
        HttpResponse::Ok().json("Room occupied")
    } else {
        HttpResponse::Ok().json("Room is currently occupied")
    }
}

#[post("/free_room")]
async fn free_room(data: web::Data<AppState>, new_room: web::Json<RoomState>) -> impl Responder {
    let mut room = data.room.lock().unwrap();

    let actix_web::web::Json(RoomState {
        current_user,
        end_time,
        ..
    }) = new_room;

    // controlla che current_user e end_time siano presenti
    if current_user.is_none() || end_time.is_none() {
        return HttpResponse::BadRequest().json("current_user and end_time are required");
    }

    if !room.is_free && room.current_user == current_user {
        let current_user = current_user.unwrap();
        println!("stanza liberata da User: {:?}", current_user);
        room.is_free = true;
        room.current_user = None;
        room.end_time = end_time;
        println!("{:?}", room);
        // il mutex di data è già lockato, non posso usare data direttamente
        let time = (room.end_time.unwrap() - room.start_time.unwrap()) / 60;
        let leaderboard_guard = data.leaderboard.lock().unwrap();
        update_leaderboard(current_user.as_ref(), time, leaderboard_guard);
        HttpResponse::Ok().json("Room freed")
    } else {
        HttpResponse::Ok().json("You cannot free the room")
    }
}

// get room status
#[get("/room_status")]
async fn room_status(data: web::Data<AppState>) -> impl Responder {
    let room = data.room.lock().unwrap();
    HttpResponse::Ok().json(&*room)
}

// get leaderboard
#[get("/leaderboard")]
async fn leaderboard(data: web::Data<AppState>) -> impl Responder {
    let leaderboard = data.leaderboard.lock().unwrap();
    HttpResponse::Ok().json(&*leaderboard)
}
// TODO salva leaderboard in un file csv locale
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        room: Mutex::new(RoomState {
            is_free: true,
            current_user: None,
            start_time: None,
            end_time: None,
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
            .service(room_status)
            .service(leaderboard)
            .service(occupy_room)
            .service(free_room)
            .service(Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind("192.168.0.61:8080")?
    .run()
    .await
}

fn update_leaderboard(username: &str, time: u64, mut leaderboard_guard: MutexGuard<Vec<User>>) {
    // let mut leaderboard_guard = data.leaderboard.lock().unwrap();
    println!("tempo: {:?}", time);
    if let Some(user) = leaderboard_guard
        .iter_mut()
        .find(|u| u.username == username)
    {
        user.total_time += time;
    } else {
        leaderboard_guard.push(User {
            username: username.to_string(),
            total_time: time,
        });
    }
    println!("{:?}", leaderboard_guard);
}

// TODO controlla ip per occupare e liberare stanza
// TODO salva classifica in un file csv locale
// TODO se stanza occupata per più di mezz'ora, libera automaticamente e non aggiornare leaderboard
