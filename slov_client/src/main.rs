use bevy::{prelude::*, input::keyboard::Key};

use bevy_egui::{
    egui::{self, Frame},
    EguiContexts, EguiPlugin,
};
use ratatui::{
    layout::Rect,
    prelude::{Line, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap , *},
};
use ratframe::RataguiBackend;
use slov_common::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BevyTerminal<RataguiBackend>>()
        //Initialize the ratatui terminal
        .init_resource::<Masterik>()
        .init_resource::<UIState>()
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
fn ui_example_system(
    mut contexts: EguiContexts,
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    mut masterok: ResMut<Masterik>,
    ui_status: Res<UIState>,
) {
    draw_ascii_game(
        &mut termres.terminal_game,
        &masterok.client_world,
        &masterok.player_id,
    );
    
    draw_ascii_info(&mut termres.terminal_info, &masterok);

    if ui_status.menu_open == MenuOpen::Take {
        draw_take_menu(
            &mut termres.terminal_menu,
            &mut masterok,
         
        );
    }
  

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(contexts.ctx_mut(), |ui| {
            let av_height = ui.available_height().clamp(100., 2000.);
            let av_width = ui.available_width().clamp(100., 2000.);

            egui::SidePanel::right("containeeee")
            .min_width(av_width / (5.))
            .max_width(av_width / (5.))
            .frame(Frame::none())
            .show_inside(ui, |ui| {
               
                if ui_status.menu_open == MenuOpen::Take {
                    egui::TopBottomPanel::top("c")
                    .min_height(200.)
                    .max_height(200.)
                    .frame(Frame::none())
       
                .show_inside(ui, |ui| {
                    ui.add(termres.terminal_menu.backend_mut());
                });
                }
                egui::TopBottomPanel::bottom("b")
                .min_height(ui.available_height().clamp(100., 2000.))
                .max_height(ui.available_height().clamp(100., 2000.))
                .frame(Frame::none())
                .show_inside(ui, |ui| {
                    ui.add(termres.terminal_info.backend_mut());
                });
            });

          

                let remain_height = ui.available_height().clamp(100., 2000.);
            egui::TopBottomPanel::top("a")
                .min_height(remain_height)
                .max_height(remain_height)
                .frame(Frame::none())
                .show_inside(ui, |ui| {
                    ui.add(termres.terminal_game.backend_mut());
                });

               
        });
}

fn draw_ascii_game(
    terminal: &mut Terminal<RataguiBackend>,
    client_world: &MyWorld,
    client_id: &EntityID,
) {
    terminal
        .draw(|frame| {
            let area = frame.size();
            let client_render =
                client_world.create_client_render_packet_for_entity(client_id, &area);
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
                Paragraph::new(Text::from(render_lines)).on_black()
                    .block(Block::new().title("game").borders(Borders::ALL)),
                area,
            );
        })
        .expect("epic fail");
}

fn draw_ascii_info(terminal: &mut Terminal<RataguiBackend>, masterok: &Masterik) {
    let mut messages_clone = masterok.messages.clone();
    messages_clone.reverse();

    let mut messages_to_show = Vec::new();

    for massage in messages_clone {
        messages_to_show.push(Line::from(massage));
    }

    terminal
        .draw(|frame| {
            let area = frame.size();

            //neccesary beccause drawing is from the top

            frame.render_widget(
                Paragraph::new(messages_to_show).on_black()
                    .block(Block::new().title("log").borders(Borders::ALL)),
                area,
            );
        })
        .expect("epic fail");
}


fn draw_take_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &mut Masterik) {
    let ent_loc = masterok.client_world.ent_loc_index.get(&masterok.player_id).unwrap_or(&(0,0));
   let mut items = masterok.client_world.get_items_at_point(ent_loc);
   let mut listitemvec = Vec::new();

   for numb in 1..9 {

        let meowmeow = items.pop();
        let meownyaa: (u64, Item) = meowmeow.unwrap_or((0,Item{item_type:ItemType::None}));
     

        if meownyaa.1.item_type != ItemType::None {
            masterok.item_button_map.insert(numb.clone(), meownyaa.0.clone());

            let item_str = format!("{}    {}", numb, &meownyaa.1.to_title()) ;
            let litem: ListItem = item_str.into();
            listitemvec.push(litem);



        }





   }



   

    terminal
        .draw(|frame| {
            let area = frame.size();

            //neccesary beccause drawing is from the top

            frame.render_widget(
                List::new(listitemvec).on_gray()
                    .block(Block::new().title("press number to choose item to pick up").borders(Borders::ALL)),
                area,
            );
        })
        .expect("epic fail");
}

// Create resource to hold the ratatui terminal
#[derive(Resource)]
struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    terminal_game: Terminal<RataguiBackend>,
    terminal_info: Terminal<RataguiBackend>,
    terminal_menu: Terminal<RataguiBackend>,
}

// Implement default on the resource to initialize it
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let mut backend1 = RataguiBackend::new(20, 20);
        backend1.set_font_size(15);
        let mut terminal1 = Terminal::new(backend1).unwrap();

        let mut backend2 = RataguiBackend::new(20, 20);
        backend2.set_font_size(11);
        let mut terminal2 = Terminal::new(backend2).unwrap();
        let mut backend3 = RataguiBackend::new(20, 20);
        backend3.set_font_size(11);
        let mut terminal3 = Terminal::new(backend3).unwrap();
        BevyTerminal {
            terminal_game: terminal1,
            terminal_info: terminal2,
            terminal_menu: terminal3
        }
    }
}

#[derive(Resource)]
struct Masterik {
    player_id: EntityID,
    location: MyPoint,
    messages: Vec<String>,
    client_world: MyWorld,
    is_logged_in: bool,
    item_button_map: HashMap<ItemKey,EntityID>
}

impl Default for Masterik {
    fn default() -> Self {
        Self {
            player_id: 1,
            location: (1, 1),
            messages: Vec::new(),
            client_world: MyWorld::new_test(),
            is_logged_in: false,
            item_button_map: HashMap::new(),
        }
    }
}


#[derive(PartialEq)]
pub enum MenuOpen {
    None,
    Take,
}


#[derive(Resource)]
struct UIState {
    menu_open: MenuOpen,
   
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            menu_open: MenuOpen::None,
            
        }
    }
}

fn local_world_process(mut masterok: ResMut<Masterik>) {
    masterok.client_world.interpret_and_execute();
    let boop = masterok
        .client_world
        .create_game_data_packet_for_entity(&masterok.player_id);
    
  
    if let Some(meow) = boop {
        //generate text messages from these action packets, then push them to the player message viewer
        let packet_actions = meow.action_info;
        let player_loc = masterok.location.clone();

        for act in packet_actions {
            let zzz = masterok
                .client_world
                .generate_isv_message(&act, &masterok.player_id);
            masterok.messages.push(zzz);
        }

        //TODO HANDLE ACTION MESSAGES

        //      println!("parsed - {:#?}", packet_actions);
        //    println!("current loc is {:#?}", lppos);
    }
}

fn create_local_account(mut masterok: ResMut<Masterik>) {
    let local_info = masterok.client_world.make_account();
    masterok.player_id = local_info.0;
    masterok.location = local_info.1;
}

fn keyboard_input_system(input: Res<ButtonInput<KeyCode>>, mut masterok: ResMut<Masterik>, mut ui_state: ResMut<UIState>) {
    let char_up = input.any_pressed([KeyCode::KeyW]);
    let char_down = input.any_pressed([KeyCode::KeyS]);
    let char_left = input.any_pressed([KeyCode::KeyA]);
    let char_right = input.any_pressed([KeyCode::KeyD]);
    let char_backspace = input.any_pressed([KeyCode::Backspace , KeyCode::Delete]);
    let char_quit = input.any_pressed([KeyCode::KeyQ]);

    let char_take = input.any_pressed([KeyCode::KeyV]);



    let char_one = input.any_pressed([KeyCode::Digit1]);
    let char_two = input.any_pressed([KeyCode::Digit2]);
    let char_three = input.any_pressed([KeyCode::Digit3]);
    let char_four = input.any_pressed([KeyCode::Digit4]);
    let char_five = input.any_pressed([KeyCode::Digit5]);
    let char_six = input.any_pressed([KeyCode::Digit6]);
    let char_seven = input.any_pressed([KeyCode::Digit7]);
    let char_eight = input.any_pressed([KeyCode::Digit8]);
    let char_nine = input.any_pressed([KeyCode::Digit9]);
    let char_zero = input.any_pressed([KeyCode::Digit0]);

    let mut client_action = ActionType::Wait;
    let client_id = masterok.player_id.clone();

    if ui_state.menu_open == MenuOpen::None {

        if char_up {
            client_action = ActionType::Go(LocativeID::Cardinal(CardinalDirection::North));
        }
        if char_down {
            client_action = ActionType::Go(LocativeID::Cardinal(CardinalDirection::South));
        }
        if char_left {
            client_action = ActionType::Go(LocativeID::Cardinal(CardinalDirection::West));
        }
        if char_right {
            client_action = ActionType::Go(LocativeID::Cardinal(CardinalDirection::East));
        }
        if char_take {
            ui_state.menu_open = MenuOpen::Take;
        }
       
    
      
    }

    if ui_state.menu_open == MenuOpen::Take {
        if char_take {
            ui_state.menu_open = MenuOpen::Take;
        }
        if char_backspace {
            ui_state.menu_open = MenuOpen::None;
        }

        if char_one {
            client_action = ActionType::Take(masterok.item_button_map.get(&1).unwrap_or(&0).clone());
        }



    }

    if char_quit {
        panic!("BYE");
    }
    if client_action != ActionType::Wait {
        masterok.client_world.receive((client_action, client_id));
        //  println!("{:#?}", masterok.client_world);
    }



  
}
