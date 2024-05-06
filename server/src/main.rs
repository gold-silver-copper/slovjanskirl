use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use ratatui::{
    prelude::{Stylize, Terminal,Line},
    widgets::{Block, Borders, Paragraph, Wrap},
    text::Text
};
use ratframe::RataguiBackend;
use slov_common::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BevyTerminal<RataguiBackend>>()
        //Initialize the ratatui terminal
        .init_resource::<Masterik>()
    
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .add_systems(PreUpdate, local_world_process)
        .add_systems(PostUpdate, keyboard_input_system)
        .add_systems(Startup, create_local_account)
        .run();
}
// Render to the terminal and to egui , both are immediate mode
fn ui_example_system(mut contexts: EguiContexts, mut termres: ResMut<BevyTerminal<RataguiBackend>>, masterok: Res<Masterik>) {
   draw_ascii_game( &mut termres.terminal_game, &masterok.client_world, &masterok.player_id);
      

    egui::Window::new("game").show(contexts.ctx_mut(), |ui| {
        ui.add(termres.terminal_game.backend_mut());
    });
    egui::Window::new("info").show(contexts.ctx_mut(), |ui| {
        ui.add(termres.terminal_info.backend_mut());
    });
}

fn draw_ascii_game ( terminal: &mut Terminal<RataguiBackend> , client_world: &MyWorld , client_id : &EntityID) {

    
    terminal.draw(|frame| {
        let area = frame.size();
        let client_render = client_world.create_client_render_packet_for_entity(client_id, &area);
        let client_graphics = client_render.spans_to_render;
        let mut render_lines = Vec::new();
        let needed_height = area.height as i16;

        if client_graphics.len() > 0 {
            for y in (0..needed_height) {
                let myspanvec: Vec<_> = client_graphics[y as usize]
                    .iter()
                    .map(|x| Span::from(&x.0).fg(x.1).bg(x.2))
                    .collect();

                let myline = Line::from(myspanvec);

                render_lines.push(myline);
            }
        }

        //neccesary beccause drawing is from the top
        render_lines.reverse();
        frame.render_widget(
            Paragraph::new(Text::from(render_lines))
                .block(Block::new().title("game").borders(Borders::ALL)),
            area,
        );
    })
    .expect("epic fail");
}

fn draw_ascii_info () {}




// Create resource to hold the ratatui terminal
#[derive(Resource)]
struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    terminal_game: Terminal<RataguiBackend>,
    terminal_info:Terminal<RataguiBackend>,
}

// Implement default on the resource to initialize it
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let backend1 = RataguiBackend::new(100, 50);
        let mut terminal1 = Terminal::new(backend1).unwrap();
        let backend2 = RataguiBackend::new(100, 50);
        let mut terminal2 = Terminal::new(backend2).unwrap();
        BevyTerminal { terminal_game:terminal1 , terminal_info:terminal2 }
    }
}


#[derive(Resource)]
struct Masterik {
    player_id: EntityID,
    location: MyPoint,
    messages: Vec<String>,
    client_world: MyWorld,

}

impl Default for Masterik {
    fn default() -> Self {
        Self { player_id: 0,
            location: (0, 0),
            messages: Vec::new(),
            client_world: MyWorld::default(), }
    }
}

fn local_world_process(mut masterok : ResMut<Masterik>){

    masterok.client_world.interpret_and_execute();
}

fn create_local_account(mut masterok : ResMut<Masterik>){

    let local_info = masterok.client_world.make_account();
    masterok.player_id = local_info.0;
    masterok.location = local_info.1;
}

fn keyboard_input_system(input: Res<ButtonInput<KeyCode>> ,mut masterok : ResMut<Masterik>)  {
    let char_up = input.any_just_pressed([KeyCode::KeyW]);
    let char_down = input.any_just_pressed([KeyCode::KeyS]);
    let char_left = input.any_just_pressed([KeyCode::KeyA]);
    let char_right = input.any_just_pressed([KeyCode::KeyD]);
    let char_quit = input.any_just_pressed([KeyCode::KeyQ]);

    let mut client_action = ActionType::Wait;
    let client_id = masterok.player_id.clone();

    if char_up {
       client_action =  ActionType::Go(LocativeID::Cardinal(CardinalDirection::North));
    }
    if char_down {
        client_action =  ActionType::Go(LocativeID::Cardinal(CardinalDirection::South));
     }
     if char_left {
        client_action =  ActionType::Go(LocativeID::Cardinal(CardinalDirection::West));
     }
     if char_right {
        client_action =  ActionType::Go(LocativeID::Cardinal(CardinalDirection::East));
     }
     if char_quit {
       panic!("BYE");
     }

     if client_action != ActionType::Wait {
        masterok.client_world.receive((client_action,client_id));
        println!("{:#?}",masterok.client_world.components);


     }

  

}