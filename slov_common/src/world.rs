use crate::*;

#[derive(Clone, Debug)]
pub struct MyWorld {
    pub voxeltile_grid: RTree<Voxel>,
    pub entity_tree: RTree<MyEntity>,
    pub server_stuff: ServerStuff,
    
    pub world_seed: u32,
    pub entity_counter: u64,
    pub ent_loc_index: HashMap<EntityID,MyPoint>
}

impl Default for MyWorld {
    fn default() -> Self {
        let rngik: u32 = 87243563;

        Self {
            voxeltile_grid: MyWorld::generate_test(rngik),
            entity_tree: RTree::new(),
            server_stuff: ServerStuff::default(),
   
            world_seed: rngik.clone(),
            entity_counter: 1,
            ent_loc_index: HashMap::new()
        }
    }
}

impl MyWorld {
    pub fn receive(&mut self, input_pair: (ActionType, EntityID)) {
        self.server_stuff
            .input_queue
            .insert(input_pair.1, input_pair.0);
        //   println!("inserted");
    }

    pub fn interpret_and_execute(&mut self) {
        let my_clone = self.server_stuff.input_queue.clone();
        self.server_stuff.input_queue.clear();
        self.server_stuff.output_queue = RTree::new();

        for (eid, action) in &my_clone {
            if let Some(ent_loc) = self.ent_loc_index.get(&eid) {
                let caller_loc = ent_loc.clone();

                let success_type = match action {
                    ActionType::Go(loc) => Action::go(self, &eid, loc),
                    ActionType::Wait => SuccessType::Success,

                    _ => panic!("not implemented"),
                };
                self.server_stuff.output_queue.insert(ActionPacket {
                    action: action.clone(),
                    success: success_type,
                    action_location: caller_loc,
                    action_subject: eid.clone(),
                })
            }
        }

        // Extend with actual game logic
    }
    //z must be above 0 for movement
    pub fn make_account(&mut self) -> (EntityID, MyPoint) {

        let pp = (80, 80);
        let eid = self.new_entity(&pp, &EntityType::Player(Player::default()));
        self.server_stuff.account_counter += 1;

        self.server_stuff
            .entity_accid_map
            .insert(eid.clone(), self.server_stuff.account_counter.clone());

     

        (eid, pp)
    }

    pub fn new_entity(&mut self, point: &MyPoint, spawn_type: &EntityType) -> EntityID {
        self.entity_counter += 1;

        //GET ENTITY ID AND START ADDING COMPONENTS AFTER IT
        let eid = self.entity_counter.clone();

      

        let my_ent = MyEntity {
            entity_id: eid.clone(),
            
            entity_type: spawn_type.clone(),
            entity_pos: point.clone()
        };

        self.ent_loc_index.insert(eid.clone(), point.clone());

    

      self.entity_tree.insert(my_ent);




        //END ADDING COMPONENTS HERE EXTRA INCREMENT CAUSE WHY NOT
        self.entity_counter += 1;

        return eid;
    }

 

    // World initialization function.
    pub fn init_world(&mut self) -> RTree<Voxel> {
        let rngik = self.world_seed.clone();
        MyWorld::generate_test(rngik)
    }

    pub fn generate_test(seed: u32) -> RTree<Voxel> {
        
        let hasher = noise::permutationtable::PermutationTable::new(seed);
        let boop = noise::utils::PlaneMapBuilder::new_fn(|point| noise::core::open_simplex::open_simplex_2d(point.into(), &hasher))
            .set_size(300, 300)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();

        let mut batchvec = Vec::new();
        for x in 0..300 {
            for y in 0..300 {
                let val = boop.get_value(x as usize, y as usize);
                let floor = if val > 0.4 { Floor::DarkGrass } else if val > -0.1 { Floor::LightGrass } else if val > -0.2 { Floor::Dirt } else if val > -0.3 { Floor::Sand }  else  { Floor::Water };
                let furniture_type = if val > 0.8 {FurnitureType::Air} else if val > 0.5 {FurnitureType::Drěvo(WoodType::Jablanj)} else if val > 0.0 {FurnitureType::Råstlina(PlantType::Kanabis)} else {FurnitureType::Air};
                batchvec.push(Voxel {
                    floor: floor,
                    furniture: Furniture {
                        furniture_type
                    },
                   
                    roof: Roof::Air,
                    voxel_pos: (x, y),
                });
            }
        }
        let newtree = RTree::bulk_load(batchvec);

        newtree
    }

    pub fn set_voxel_at(&mut self, vox: &Voxel) {
        if let Some(boop) = self
         
            .voxeltile_grid
            .locate_at_point_mut(&vox.voxel_pos)
        {
            *boop = vox.clone();
        } else {
            self.voxeltile_grid.insert(vox.clone())
        }
    }

    pub fn get_voxel_at(&self, point: &MyPoint) -> Option<Voxel> {
        if let Some(boop) = self.voxeltile_grid.locate_at_point(point) {
            Some(boop.clone())
        } else {
            None
        }
    }

    pub fn voxel_blocks_movement_at(&self, point: &MyPoint) -> bool {
        if let Some(got_point) = self.get_voxel_at(point) {
            //todo!("implement voxel blocking movements");
            return false;
        } else {
            if point.0 > 0 && point.1 > 0 {
                return false;
            } else {
                return true;
            }
        }
    }

    pub fn entity_blocks_movement_at(&self, point: &MyPoint) -> bool {
       

        if point.0 > 0 && point.1 > 0 {
            return false;
        } else {
            return true;
        }
    }

    pub fn generate_isv_message(
        &self,
        act_packet: &ActionPacket,
        local_player_id: &EntityID,
    ) -> String {
        /*
            let msg_person = if local_player_id == &act_packet.action_subject {
                Person::Second
            } else {
                Person::Third
            };
        */
        let abc = format!("HELLOᐂᐂᐂ𓀀𓀀𓀀 {:#?}", &act_packet.action_location);
        abc
    }

    pub fn create_client_render_packet_for_entity(
        &self,
        ent: &EntityID,
        render_rect: &Rect,
    ) -> RenderPacket {
        if let Some(e_pos) = self.ent_loc_index.get(ent) {
            let render_width = render_rect.width;
            let render_height = render_rect.height;
            let w_radius = render_width / 2;
            let h_radius = render_height / 2;
            let same_z = locate_square(e_pos, w_radius as i64, h_radius as i64);

            let local_ents = self.components.positions.locate_in_envelope(&same_z);
            let local_voxels = self.terrain.voxeltile_grid.locate_in_envelope(&same_z);

            let local_voxel_diffs = self.terrain.voxeltile_diffs.locate_in_envelope(&same_z);

            let local_actions = self
                .server_stuff
                .output_queue
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE);

            let bottom_left_of_game_screen = (e_pos.0 - w_radius as i64, e_pos.1 - h_radius as i64);

            // THIS GRID IS INDEXD Y FIRST
            let mut voxel_grid = create_2d_array(render_width.into(), render_height.into());

            let mut ent_vec = Vec::new();

            for pc in local_ents {
                let relative_point_x = pc.point.0 - bottom_left_of_game_screen.0;
                let relative_point_y = pc.point.1 - bottom_left_of_game_screen.1;
                if let Some(enttt) = self.components.entities.get(&pc.entity_id) {
                    let et = enttt.entity_type.clone();
                    ent_vec.push(((relative_point_x, relative_point_y), et.to_graphictriple()))
                } else {
                    ent_vec.push((
                        (relative_point_x, relative_point_y),
                        ("?".into(), Color::LightCyan, Color::LightCyan),
                    ))
                }
            }

            for lv in local_voxels {
                let relative_point_x = lv.voxel_pos.0 - bottom_left_of_game_screen.0;
                let relative_point_y = lv.voxel_pos.1 - bottom_left_of_game_screen.1;

                if (0 < relative_point_y)
                    && (relative_point_y < render_height as i64)
                    && (0 < relative_point_x)
                    && (relative_point_x < render_width as i64)
                {
                    let boop = lv.to_graphic();
                    voxel_grid[relative_point_y as usize][relative_point_x as usize] = boop;
                }
            }
            for vd in local_voxel_diffs {
                let relative_point_x = vd.voxel_pos.0 - bottom_left_of_game_screen.0;
                let relative_point_y = vd.voxel_pos.1 - bottom_left_of_game_screen.1;
                if (0 < relative_point_y)
                    && (relative_point_y < render_height as i64)
                    && (0 < relative_point_x)
                    && (relative_point_x < render_width as i64)
                {
                    let boop2 = vd.to_graphic();
                    voxel_grid[relative_point_y as usize][relative_point_x as usize] = boop2;
                }
            }

            //merge grids

            for (ent_relative, ent_graphic) in ent_vec {
                if (0 < ent_relative.1)
                    && (ent_relative.1 < render_height as i64)
                    && (0 < ent_relative.0)
                    && (ent_relative.0 < render_width as i64)
                {
                    voxel_grid[ent_relative.1 as usize][ent_relative.0 as usize].0 = ent_graphic.0;
                    voxel_grid[ent_relative.1 as usize][ent_relative.0 as usize].1 = ent_graphic.1;
                }
            }

            for la in local_actions {
                //   actions.push(la.clone());
            }

            RenderPacket {
                spans_to_render: voxel_grid,

                messages_to_render: Vec::new(),
            }
        } else {
            // println!("DESSSSSS");
            RenderPacket::new()
        }
    }

    pub fn create_game_data_packet_for_entity(&self, ent: &EntityID) -> Option<GameDataPacket> {
        if let Some(e_pos) = self.components.ent_loc_index.get(ent) {
            let local_ents = self
                .components
                .positions
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE * 2);
            let local_voxels = self
                .terrain
                .voxeltile_diffs
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE / 2);
            let local_actions = self
                .server_stuff
                .output_queue
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE / 8);

            let mut e_info = Vec::new();
            let mut v_diffs = Vec::new();
            let mut actions = Vec::new();

            for pc in local_ents {
                e_info.push(EntityPacket {
                    entity_pos: pc.point.clone(),
                    entity_id: pc.entity_id.clone(),
                    entity_type: self
                        .components
                        .entities
                        .get(ent)
                        .unwrap()
                        .entity_type
                        .clone(),
                })
            }

            for vd in local_voxels {
                v_diffs.push(vd.clone());
            }

            for la in local_actions {
                actions.push(la.clone());
            }

            Some(GameDataPacket {
                entity_info: e_info,
                voxel_diffs: v_diffs,
                action_info: actions,
            })
        } else {
            None
        }
    }

    pub fn voxel_blocks_vision_at(&self, point: &MyPoint) -> bool {
        if let Some(got_point) = MyWorld::get_voxel_at(&self, point) {
            return false;
        } else {
            return false;
        }
    }

    pub fn set_ent_loc(&mut self, ent: &EntityID, destination: &MyPoint) {
        if let Some(xyz) = self.components.ent_loc_index.get(ent) {
            self.components.positions.remove(&PositionComponent {
                entity_id: ent.clone(),
                point: xyz.clone(),
            });
            self.components.positions.insert(PositionComponent {
                entity_id: ent.clone(),
                point: destination.clone(),
            });
            self.components
                .ent_loc_index
                .insert(ent.clone(), destination.clone());
        }
    }

    pub fn move_entity_in_direction(
        &mut self,
        ent: &EntityID,
        cd: &CardinalDirection,
    ) -> SuccessType {
        if let Some(xyz) = self.components.ent_loc_index.get(ent) {
            println!("GOT ENT LOC INDEX FOR MOVEMENT");

            let dir_point = cd.to_xyz();
            let goal = add_two_points(&xyz, &dir_point);

            if !self.voxel_blocks_movement_at(&goal) {
                if !self.entity_blocks_movement_at(&goal) {
                    self.set_ent_loc(ent, &goal);
                    return SuccessType::Success;
                } else {
                    SuccessType::Failure
                }
            } else {
                SuccessType::Failure
            }
        } else {
            SuccessType::Failure
        }
    }
}
