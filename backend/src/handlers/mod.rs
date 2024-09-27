use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::Local;
use std::fs::OpenOptions;
use std::net::SocketAddr;
use std::sync::MutexGuard;

use crate::models::{AppState, RoomState, User};

#[post("/occupy_room")]
async fn occupy_room(
    data: web::Data<AppState>,
    new_room: web::Json<RoomState>,
    req: HttpRequest,
) -> impl Responder {
    let mut room = data.room.lock().unwrap();
    let actix_web::web::Json(RoomState {
        current_user,
        start_time,
        ..
    }) = new_room;

    // Ottieni l'indirizzo IP del client
    let ip: Option<SocketAddr> = req.peer_addr();
    let ip = ip
        .unwrap()
        .to_string()
        .split(':')
        .next()
        .unwrap()
        .parse()
        .ok();
    // println!("IP del client che vuole OCCUPY: {:?}", ip);

    if room.is_free {
        // controlla che current_user e start_time siano presenti
        if current_user.is_none() || start_time.is_none() || ip.is_none() {
            return HttpResponse::BadRequest().json("current_user and start_time are required");
        }

        room.is_free = false;
        // println!("stanza occupata da User: {:?}", current_user);
        room.current_user = current_user;
        room.start_time = start_time;
        room.ip_user = ip;

        HttpResponse::Ok().json("stanca occupata, buon divertimento 😉")
    } else {
        HttpResponse::Ok().json("la stanza è già occupata 😢, trattieniti")
    }
}

#[post("/free_room")]
async fn free_room(
    data: web::Data<AppState>,
    new_room: web::Json<RoomState>,
    req: HttpRequest,
) -> impl Responder {
    let mut room = data.room.lock().unwrap();

    let actix_web::web::Json(RoomState {
        current_user,
        end_time,
        ..
    }) = new_room;

    // Ottieni l'indirizzo IP del client
    let ip: Option<SocketAddr> = req.peer_addr();
    // converto IP in Option<String> e tolgo quello che viene dopo i due punti. es 192.168.0.61:62604 -> 192.168.0.61
    let ip = ip
        .unwrap()
        .to_string()
        .split(':')
        .next()
        .unwrap()
        .parse()
        .ok();
    // println!("IP del client che vuole FREE: {:?}", ip);

    // controlla che current_user e end_time siano presenti
    if current_user.is_none() || end_time.is_none() || ip.is_none() {
        return HttpResponse::BadRequest().json("current_user and end_time are required");
    }

    if !room.is_free && room.current_user == current_user && room.ip_user == ip {
        let current_user = current_user.unwrap();
        // println!("stanza liberata da User: {:?}", current_user);
        room.is_free = true;
        room.current_user = None;
        room.end_time = end_time;
        // println!("{:?}", room);
        // il mutex di data è già lockato, non posso usare data direttamente
        let time = (room.end_time.unwrap() - room.start_time.unwrap()) / 60;
        let leaderboard_guard = data.leaderboard.lock().unwrap();
        update_leaderboard(current_user.as_ref(), time, leaderboard_guard);
        HttpResponse::Ok().json("stanza liberata, alla prossima 😊")
    } else {
        HttpResponse::Ok()
            .json("non puoi liberare la stanza, prova ad andare a bussare o aspetta il tuo turno")
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

fn update_leaderboard(username: &str, time: u64, mut leaderboard_guard: MutexGuard<Vec<User>>) {
    // ordinamento decrescente per tempo
    // println!("tempo: {:?}", time);
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
    // ordina la classifica in base al tempo totale
    leaderboard_guard.sort_by(|a, b| b.total_time.cmp(&a.total_time));
    // println!("{:?}", leaderboard_guard);
    // salva la classifica aggiornata nel file csv
    let today = Local::now().date_naive();
    let today = today.format("%m-%Y").to_string();
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("{}.csv", today))
        .unwrap();
    let mut writer = csv::Writer::from_writer(file);
    for user in leaderboard_guard.iter() {
        writer.serialize(user).unwrap();
    }
    writer.flush().unwrap();

    // println!("Leaderboard aggiornata");
}
