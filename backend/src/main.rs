use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::net::SocketAddr;
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
    ip_user: Option<String>,
    start_time: Option<u64>, // timestamp in secondi
    end_time: Option<u64>,   // timestamp in secondi
}
#[derive(Debug)]
struct AppState {
    room: Mutex<RoomState>,
    leaderboard: Mutex<Vec<User>>,
}

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
    println!("IP del client che vuole OCCUPY: {:?}", ip);

    if room.is_free {
        // controlla che current_user e start_time siano presenti
        if current_user.is_none() || start_time.is_none() || ip.is_none() {
            return HttpResponse::BadRequest().json("current_user and start_time are required");
        }

        room.is_free = false;
        println!("stanza occupata da User: {:?}", current_user);
        room.current_user = current_user;
        room.start_time = start_time;
        room.ip_user = ip;
        HttpResponse::Ok().json("Room occupied")
    } else {
        HttpResponse::Ok().json("Room is currently occupied")
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
    println!("IP del client che vuole FREE: {:?}", ip);

    // controlla che current_user e end_time siano presenti
    if current_user.is_none() || end_time.is_none() || ip.is_none() {
        return HttpResponse::BadRequest().json("current_user and end_time are required");
    }

    if !room.is_free && room.current_user == current_user && room.ip_user == ip {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ottieni data di oggi
    let today = Local::now().date_naive();
    // crea una stringa partendo dalla data con numero del mese e anno
    let today = today.format("%m-%Y").to_string();
    // prova ad aprire un file csv con il nome della data, se non esiste crea un nuovo file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}.csv", today))
        .unwrap();

    // leggi il contenuto del file csv e salvalo in un vettore di tipo User
    let mut file_leaderboard = Vec::new();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let user: User = result.unwrap();
        file_leaderboard.push(user);
    }

    let app_state = web::Data::new(AppState {
        room: Mutex::new(RoomState {
            is_free: true,
            current_user: None,
            ip_user: None,
            start_time: None,
            end_time: None,
        }),
        leaderboard: Mutex::new(file_leaderboard),
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
    .bind("192.168.0.61:8081")?
    .run()
    .await
}

fn update_leaderboard(username: &str, time: u64, mut leaderboard_guard: MutexGuard<Vec<User>>) {
    // ordinamento decrescente per tempo
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
    // ordina la classifica in base al tempo totale
    leaderboard_guard.sort_by(|a, b| b.total_time.cmp(&a.total_time));
    println!("{:?}", leaderboard_guard);
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

    println!("Leaderboard aggiornata");
}

// TODO se stanza occupata per più di mezz'ora, libera automaticamente e non aggiornare leaderboard
