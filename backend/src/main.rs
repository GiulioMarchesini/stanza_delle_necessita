// #![windows_subsystem = "windows"] // per non far apparire la console su windows
mod handlers;
mod models;

use crate::handlers::{free_room, leaderboard, occupy_room, room_status};
use crate::models::{AppState, RoomState, User};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::sync::Mutex;
use std::thread;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ottieni data di oggi
    let today = Local::now().date_naive();
    // crea una stringa partendo dalla data con numero del mese e anno
    let today = today.format("%m-%Y").to_string();
    // prova ad aprire un file csv con il nome della data, se non esiste crea un nuovo file
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join(format!("{}.csv", today));
    println!("{:?}", file_path);

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
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

    // Avvia un thread separato per controllare periodicamente lo stato della stanza
    let app_state_clone = app_state.clone();
    std::thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_secs(60));
        let mut room = app_state_clone.room.lock().unwrap();
        if !room.is_free {
            let now = Local::now().timestamp();
            let now = now as u64; // in secondi
            if let Some(start_time) = room.start_time {
                if now - start_time > 1800 {
                    room.is_free = true;
                    room.current_user = None;
                    room.start_time = None;
                    room.end_time = None;
                    println!("Stanza liberata automaticamente");
                }
            }
        }
    });

    let server_ip = "192.168.0.61:9696";
    println!("Server running at http://{}", server_ip);

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
    .bind(server_ip)?
    .run()
    .await
}
