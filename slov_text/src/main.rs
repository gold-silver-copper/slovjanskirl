use bevy::{input::keyboard::Key, prelude::*, render::view::visibility};


use bevy_egui::{
    egui::{self, Frame,FontData, FontDefinitions, FontFamily},
    EguiContexts, EguiPlugin,
};
use egui_ratatui::RataguiBackend;
use ratatui::{
    layout::Rect,
    prelude::{Line, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BevyTerminal<RataguiBackend>>()
        //Initialize the ratatui terminal
        .init_resource::<Masterik>()
        .add_plugins(EguiPlugin)
        .add_systems(PostStartup, set_custom_font)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
      
        .add_systems(PostUpdate, keyboard_input_system)
        .run();
}
// Render to the terminal and to egui , both are immediate mode
fn ui_example_system(
    mut contexts: EguiContexts,
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    mut masterok: ResMut<Masterik>,
) {
    draw_attack_menu(&mut termres.terminal_info, &mut masterok);

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(contexts.ctx_mut(), |ui| {
            let av_height = ui.available_height().clamp(100., 1500.);
            let av_width = ui.available_width().clamp(100., 1500.);

           
            let remain_height = ui.available_height().clamp(100., 1500.);
            egui::TopBottomPanel::top("game")
                .min_height(remain_height)
                .max_height(remain_height)
                .frame(Frame::none())
                .show_inside(ui, |ui| {
                    ui.add(termres.terminal_info.backend_mut());
                });
        });
}

fn draw_attack_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &mut Masterik) {
    terminal
        .draw(|frame| {
            let area = frame.size();

            //neccesary beccause drawing is from the top

            frame.render_widget(
                Paragraph::new("ASDASDASDSSSS").on_gray().block(
                    Block::new()
                        .title("press number to choose item to pick up")
                        .blue()
                        .borders(Borders::ALL),
                ),
                area,
            );
        })
        .expect("epic fail");
}
//create resource to hold the ratatui terminal
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
        backend1.set_font_size(20);
        let mut terminal1 = Terminal::new(backend1).unwrap();

        let mut backend2 = RataguiBackend::new(20, 20);
        backend2.set_font_size(12);
        let mut terminal2 = Terminal::new(backend2).unwrap();
        let mut backend3 = RataguiBackend::new(20, 20);
        backend3.set_font_size(12);
        let mut terminal3 = Terminal::new(backend3).unwrap();
        BevyTerminal {
            terminal_game: terminal1,
            terminal_info: terminal2,
            terminal_menu: terminal3,
        }
    }
}

#[derive(Resource)]
struct Masterik {
  

    messages: Vec<String>,

    is_logged_in: bool,
   
    list_cursor_index: usize,

    menu_open: MenuOpen,
}

impl Masterik {
    pub fn refresh_menus(&mut self) {
        self.list_cursor_index = 0;
      //  self.targeted_ent_id = 0;
    //    self.button_entityid_map.drain();
        //self.button_itemstruct_map.drain();
    }
}

impl Default for Masterik {
    fn default() -> Self {
        Self {
     

            messages: Vec::new(),
            
            is_logged_in: false,
            list_cursor_index: 0,
          
        
            menu_open: MenuOpen::None,
        }
    }
}

#[derive(PartialEq)]
pub enum MenuOpen {
    None,
    Take,
    Drop,
    Inventory,
    Stats,
    CursorInteract,
    PlayerLocationInteract,
    Attack,
}

fn set_custom_font(mut contexts: EguiContexts) {
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "my_font".to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/hiero_mono.ttf")),
    ); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "my_font".to_owned());

    contexts.ctx_mut().set_fonts(fonts);
}

fn keyboard_input_system(input: Res<ButtonInput<KeyCode>>, mut masterok: ResMut<Masterik>) {
    let char_up = input.any_pressed([KeyCode::KeyW]);
    let char_down = input.any_pressed([KeyCode::KeyS]);
    let char_left = input.any_pressed([KeyCode::KeyA]);
    let char_right = input.any_pressed([KeyCode::KeyD]);

    let cursor_up = input.any_just_pressed([KeyCode::KeyI]);
    let cursor_down = input.any_just_pressed([KeyCode::KeyK]);
    let cursor_left = input.any_just_pressed([KeyCode::KeyJ]);
    let cursor_right = input.any_just_pressed([KeyCode::KeyL]);

    let char_attack = input.any_just_pressed([KeyCode::KeyY]); // jęti (jme) / vzeti
    let char_take = input.any_just_pressed([KeyCode::KeyG]); // jęti (jme) / vzeti metnuti  imej target range do ktorogo mozno metati dla praktiki zeby povysati skil be ubijstva
    let char_drop = input.any_just_pressed([KeyCode::KeyB]); //izbaviti se
    let char_help = input.any_just_pressed([KeyCode::KeyT]); //pokazati pomoc ?

    let char_one = input.any_just_pressed([KeyCode::Digit1]);
    let char_two = input.any_just_pressed([KeyCode::Digit2]);
    let char_three = input.any_just_pressed([KeyCode::Digit3]);
    let char_four = input.any_just_pressed([KeyCode::Digit4]);
    let char_five = input.any_just_pressed([KeyCode::Digit5]);
    let char_six = input.any_just_pressed([KeyCode::Digit6]);
    let char_seven = input.any_just_pressed([KeyCode::Digit7]);
    let char_eight = input.any_just_pressed([KeyCode::Digit8]);
    let char_nine = input.any_just_pressed([KeyCode::Digit9]);
    let char_zero = input.any_just_pressed([KeyCode::Digit0]);

    let char_backspace = input.any_pressed([KeyCode::Backspace, KeyCode::Delete]);
    let char_quit = input.any_just_pressed([KeyCode::KeyQ]);

    if char_quit {
        panic!("BYE");
    }
}
