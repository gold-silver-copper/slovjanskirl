use bevy::{log::LogPlugin, prelude::*};
use bevy_rtc::prelude::*;
use protocol::{PingPayload, PongPayload,LoginLoad, LoginAnswer};
use slov_common::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(RtcServerPlugin {
            port: 3536,
            // CAREFUL: This encoding MUST match the client encoding!

        })
        .init_resource::<Masterik>()
        .add_server_ro_protocol::<PingPayload>(1)
        .add_server_ro_protocol::<LoginLoad>(1)
        .add_server_wo_protocol::<PongPayload>()
        .add_systems(
            Update,
            network_handler,
        )
        .run();
}

pub fn network_handler(mut reader: RtcServer<PingPayload>, mut writer: RtcServer<PongPayload>){
    for (peer_id, _ping) in reader.read() {
        info!("Received ping! Sending pong...");
        writer.reliable_to_peer(peer_id, PongPayload);
    }
}
pub fn login_handler(mut reader: RtcServer<LoginLoad>, mut writer: RtcServer<LoginAnswer>,mut masterok : ResMut<Masterik>){
    for (peer_id, login) in reader.read() {
       
    let local_info = masterok.server_world.make_account();
    let player_id = local_info.0;
    let player_location = local_info.1;
        writer.reliable_to_peer(peer_id, LoginAnswer {player_id,player_location});
    }
}

#[derive(Resource)]
struct Masterik {

  
    server_world: MyWorld,
 

}

impl Default for Masterik {
    fn default() -> Self {
        Self {
            server_world: MyWorld::default(),   }
          
    }
}