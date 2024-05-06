use crate::*;
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;

use std::{
    io::{self, Stdout, Write},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub struct LocalPlayerInfo {
    player_id: EntityID,
    location: MyPoint,
    messages: Vec<String>,
    client_world: MyWorld,
}

pub struct GameClient {
    pub local_player_info: Arc<Mutex<LocalPlayerInfo>>,
}

impl GameClient {
    pub fn new() -> Self {
        GameClient {
            local_player_info: Arc::new(Mutex::new(LocalPlayerInfo {
                player_id: 0,
                location: (0, 0, 0),
                messages: Vec::new(),
                client_world: MyWorld::new_world(),
            })),
        }
    }

    pub fn run_client_loop(&mut self) -> Result<()> {
        let lpinfo_for_loop = self.local_player_info.clone();
        let lpinfo_for_render = self.local_player_info.clone();
        let lpinfo_for_input = self.local_player_info.clone();
        let lpinfo_for_create = self.local_player_info.clone();

        // get a socket that is connected to the admin namespace
        let socket = ClientBuilder::new("http://0.0.0.0:3000")
            .namespace("/")
            .on("game_input_response", move |payload: Payload, _| {
                if let Payload::Text(text_data) = payload {
                    let mut lpinfo = lpinfo_for_input.lock().unwrap();

                    for beep in text_data {
                        let parsed: Result<GameDataPacket, serde_json::Error> =
                            serde_json::from_value(beep);
                        if let Ok(meow) = parsed {
                            //replace local ent data with this data, update local player position
                            let packet_ents = meow.entity_info;
                            //replace local diffs with these diffs
                            let packet_voxels = meow.voxel_diffs;
                            //generate text messages from these action packets, then push them to the player message viewer
                            let packet_actions = meow.action_info;
                            let player_loc = lpinfo.location.clone();

                            let local_positions = lpinfo.client_world.positions.drain();

                            let mut bulk_positions: Vec<PositionComponent> = Vec::new();

                            let packet_entity_ids: Vec<EntityID> =
                                packet_ents.iter().map(|x| x.entity_id.clone()).collect();

                            let packet_voxel_points: Vec<MyPoint> =
                                packet_voxels.iter().map(|x| x.voxel_pos.clone()).collect();

                            for lp in local_positions {
                                if !packet_entity_ids.contains(&lp.entity_id) {
                                    bulk_positions.push(lp)
                                }
                            }
                            for pent in &packet_ents {
                                if pent.entity_id == lpinfo.player_id {
                                    lpinfo.location = pent.entity_pos.clone();
                                }

                                lpinfo.client_world.entities.insert(pent.entity_id.clone());
                                lpinfo
                                    .client_world
                                    .entity_types
                                    .insert(pent.entity_id.clone(), pent.entity_type.clone());
                                bulk_positions.push(PositionComponent {
                                    entity_id: pent.entity_id.clone(),
                                    point: pent.entity_pos.clone(),
                                });
                                lpinfo
                                    .client_world
                                    .ent_loc_index
                                    .insert(pent.entity_id.clone(), pent.entity_pos.clone());
                            }

                            for bp in bulk_positions {
                                lpinfo.client_world.positions.insert(bp);
                            }

                            let mut bulk_voxels: Vec<Voxel> = Vec::new();

                            let local_diffs = lpinfo
                                .client_world
                                .voxeltile_diffs
                                .drain_within_distance(player_loc, LOCAL_RANGE + 5);

                            for vx in local_diffs {
                                if !packet_voxel_points.contains(&vx.voxel_pos) {
                                    bulk_voxels.push(vx);
                                }
                            }

                            for pv in packet_voxels {
                                bulk_voxels.push(pv);
                            }

                            for boop in bulk_voxels {
                                lpinfo.client_world.voxeltile_diffs.insert(boop);
                            }

                            for act in packet_actions {
                                let zzz = lpinfo
                                    .client_world
                                    .generate_isv_message(&act, &lpinfo.player_id);
                                lpinfo.messages.push(zzz);
                            }

                            //TODO HANDLE ACTION MESSAGES

                            //      println!("parsed - {:#?}", packet_actions);
                            //    println!("current loc is {:#?}", lppos);
                        }
                    }
                }
            })
            .on("create_character_response", move |payload: Payload, _| {
                if let Payload::Text(text_data) = payload {
                    for response in text_data {
                        //    println!("CREATE CHARACTER RESPONSE REC");
                        if let Ok(ccd) = serde_json::from_value::<CreateCharacterData>(response) {
                            let mut lpinfo = lpinfo_for_create.lock().unwrap();
                            lpinfo.player_id = ccd.player_id.clone();
                            lpinfo.location = ccd.player_position.clone();

                            //     println!("local player id updated to - {}", *id_lock);
                            //     println!("local player loc updated to - {:#?}", *pos_loc);
                        }
                    }
                }
            })
            .on("error", |err, _| eprintln!("Error: {:#?}", err))
            .connect()
            .expect("Connection failed");

        let terminal_closure = |frame: &mut Frame| {
            let lpinfo = lpinfo_for_loop.lock().unwrap();
            let wow = format!("{:#?}", &lpinfo.location);
            let mut messages_clone = lpinfo.messages.clone();
            messages_clone.reverse();

            let mut messages_to_show = Vec::new();

            for massage in messages_clone {
                messages_to_show.push(Line::from(massage));
            }

            let outer_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
                .split(frame.size());

            let inner_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(outer_layout[1]);

            let client_render = lpinfo
                .client_world
                .create_client_render_packet_for_entity(&lpinfo.player_id, &outer_layout[0]);
            let client_graphics = client_render.spans_to_render;
            let mut render_lines = Vec::new();
            let needed_height = outer_layout[0].height as i16;

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
                outer_layout[0],
            );
            frame.render_widget(
                Paragraph::new(wow).block(Block::new().title("info").borders(Borders::ALL)),
                inner_layout[0],
            );
            frame.render_widget(
                Paragraph::new(messages_to_show)
                    .block(Block::new().title("log").borders(Borders::ALL)),
                inner_layout[1],
            );
        };

        // emit to the "foo" event
        let json_payload = String::new();

        // emit with an ack
        socket
            .emit("create_character", json_payload)
            .expect("Server unreachable");

        //RENDER LOOP BEGINS HERE
        //
        //
        //
        //
        let mut terminal = GameClient::setup_terminal().context("setup failed")?;

        loop {
            println!("WAITING TO GET LOCAL PLAYER ID FROM SERVER");
            let lpinfo = lpinfo_for_render.lock().unwrap();
            if lpinfo.player_id != 0 {
                break;
            }
        }
        loop {
            terminal.draw(terminal_closure);

            let input_to_send = GameClient::poll_to_action();

            if input_to_send == ActionType::Quit {
                break;
            }
            if input_to_send != ActionType::Wait {
                socket
                    .emit("game_input", json!(input_to_send))
                    .expect("Server unreachable");
            }
        }
        socket.disconnect().expect("Disconnect failed");
        GameClient::restore_terminal(&mut terminal).context("restore terminal failed")?;

        Ok(())
    }

    pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
        let mut stdout = io::stdout();
        enable_raw_mode().context("failed to enable raw mode")?;
        execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
        Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
    }

    /// Restore the terminal. This is where you disable raw mode, leave the alternate screen, and show
    /// the cursor.
    pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        disable_raw_mode().context("failed to disable raw mode")?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)
            .context("unable to switch to main screen")?;
        terminal.show_cursor().context("unable to show cursor")
    }

    pub fn poll_to_char() -> Option<char> {
        if event::poll(Duration::from_millis(150)).expect("event poll failed") {
            if let Event::Key(key) = event::read().expect("event read failed") {
                if let KeyCode::Char(needed_char) = key.code {
                    return Some(needed_char);
                }
            }
        }
        None
    }

    pub fn char_to_action(got_char: String) -> ActionType {
        let processed_char = got_char.to_lowercase();

        match processed_char.as_str() {
            "w" => ActionType::Go(Locative::Cardinal(CardinalDirection::North)),
            "a" => ActionType::Go(Locative::Cardinal(CardinalDirection::West)),
            "s" => ActionType::Go(Locative::Cardinal(CardinalDirection::South)),
            "d" => ActionType::Go(Locative::Cardinal(CardinalDirection::East)),
            "q" => ActionType::Quit,
            _ => ActionType::Wait,
        }
    }

    pub fn poll_to_action() -> ActionType {
        let boop = GameClient::poll_to_char();

        match boop {
            Some(gotchar) => GameClient::char_to_action(gotchar.to_string()),
            None => ActionType::Wait,
        }
    }
}
