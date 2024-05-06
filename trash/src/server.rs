use crate::*;
use axum::routing::get;
use serde::*;

use socketioxide::{
    extract::{Data, SocketRef},
    socket::Sid,
    SocketIo,
};

use tokio::time::{self, Duration};

pub struct GameServer {
    server_world: Arc<Mutex<MyWorld>>,
}

impl GameServer {
    pub fn new() -> Self {
        GameServer {
            // Wrap your game state with Arc<Mutex<T>> for shared access and mutation , necessary for async
            server_world: Arc::new(Mutex::new(MyWorld::new_world())),
        }
    }

    pub async fn run_server_loop(&self) {
        println!("STARTING SERVER LOOP");

        let (layer, io) = SocketIo::new_layer();

        let game_world_for_socket_io = self.server_world.clone();

        io.ns("/", move |s: SocketRef| {
            let game_world_upper = game_world_for_socket_io.clone();

            let game_world_for_input = game_world_upper.clone();

            s.on(
                "game_input",
                move |s: SocketRef, Data::<ActionType>(data)| {
                    let mut game_world = game_world_for_input.lock().unwrap();

                    if let Some(eid) = game_world.sid_eid_map.get(&s.id) {
                        let sender_entity_id = eid.clone();
                        game_world.receive((data, sender_entity_id));
                    } else {
                        println! {"SID has no corresponding pid"}
                    }
                },
            );

            let game_world_for_character_creation = game_world_upper.clone();

            s.on(
                "create_character",
                move |s: SocketRef, Data::<String>(data)| {
                    let mut game_world = game_world_for_character_creation.lock().unwrap();

                    let pid = game_world.make_account();
                    game_world.sid_eid_map.insert(s.id.clone(), pid.clone());

                    if let Some(player_loc) = game_world.ent_loc_index.get(&pid) {
                        s.emit(
                            "create_character_response",
                            CreateCharacterData {
                                player_id: pid.clone(),
                                player_position: player_loc.clone(),
                            },
                        )
                        .ok();
                    }
                },
            );
        });

        let app = axum::Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(layer);

        // Background task for game world updates
        let game_world_loop = self.server_world.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(200));
            loop {
                interval.tick().await;
                let mut world = game_world_loop.lock().unwrap();

                world.interpret_and_execute();

                let sockets = io.sockets().unwrap();

                for socket in sockets {
                    if let Some(eid) = world.sid_eid_map.get(&socket.id) {
                        socket
                            .emit(
                                "game_input_response",
                                serde_json::json!(world.create_game_data_packet_for_entity(eid)),
                            )
                            .ok();
                    }
                }
            }
        });

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
