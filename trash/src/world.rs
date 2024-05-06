use crate::*;

use ratatui::{layout::Rect, text::Line};
use socketioxide::{
    extract::{Data, SocketRef},
    socket::Sid,
    SocketIo,
};
pub struct MyWorld {
    pub entities: HashSet<EntityID>,
    pub positions: RTree<PositionComponent>,
    pub ent_loc_index: HashMap<EntityID, MyPoint>, //xyz
    pub healths: HashMap<EntityID, HealthComponent>,
    pub entity_types: HashMap<EntityID, EntityType>,
    pub voxeltile_grid: RTree<Voxel>,
    pub voxeltile_diffs: RTree<Voxel>,
    pub entity_counter: u64,
    pub input_queue: HashMap<EntityID, ActionType>,
    pub output_queue: RTree<ActionPacket>,

    pub sid_eid_map: HashMap<Sid, EntityID>,
    pub entity_socketid_map: HashMap<EntityID, Sid>,
    pub server_seed: u64,
}

impl MyWorld {
    pub fn new_world() -> Self {
        MyWorld {
            entities: HashSet::new(),
            positions: RTree::new(),
            ent_loc_index: HashMap::new(),
            entity_types: HashMap::new(),
            healths: HashMap::new(),
            voxeltile_grid: MyWorld::init_world(),
            entity_counter: 0,
            input_queue: HashMap::new(),
            output_queue: RTree::new(),
            voxeltile_diffs: RTree::new(),

            sid_eid_map: HashMap::new(),
            entity_socketid_map: HashMap::new(),
            server_seed: 0,
        }
    }

    pub fn receive(&mut self, input_pair: (ActionType, EntityID)) {
        self.input_queue.insert(input_pair.1, input_pair.0);
        //   println!("inserted");
    }

    pub fn interpret_and_execute(&mut self) {
        let my_clone = self.input_queue.clone();
        self.input_queue.clear();
        self.output_queue = RTree::new();

        for (eid, action) in &my_clone {
            if let Some(ent_loc) = self.ent_loc_index.get(&eid) {
                let caller_loc = ent_loc.clone();

                let success_type = match action {
                    ActionType::Go(loc) => Action::go(self, &eid, loc),
                    ActionType::Wait => SuccessType::Success,

                    _ => panic!("not implemented"),
                };
                self.output_queue.insert(ActionPacket {
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
    pub fn make_account(&mut self) -> EntityID {
        let eid = self.new_entity(&(9, 9, 1), &EntityType::Player);

        eid
    }

    pub fn new_entity(&mut self, point: &MyPoint, spawn_type: &EntityType) -> EntityID {
        self.entity_counter += 1;

        //GET ENTITY ID AND START ADDING COMPONENTS AFTER IT
        let eid = self.entity_counter.clone();

        self.entities.insert(eid.clone());

        self.positions.insert(PositionComponent {
            entity_id: eid.clone(),
            point: point.clone(),
        });
        self.ent_loc_index.insert(eid.clone(), point.clone());

        self.entity_types.insert(eid.clone(), spawn_type.clone());

        self.healths.insert(
            eid.clone(),
            HealthComponent {
                cur_health: 100,
                max_health: 100,
            },
        );

        //END ADDING COMPONENTS HERE EXTRA INCREMENT CAUSE WHY NOT
        self.entity_counter += 1;

        return eid;
    }

    pub fn get_ents_at_point(&self, point: &MyPoint) -> Vec<EntityID> {
        let mut mvec = Vec::new();
        let boop = self.positions.locate_all_at_point(point);

        for player in boop {
            mvec.push(player.entity_id.clone());
        }
        mvec
    }

    pub fn get_ents_in_aabb(&self, p1: &(i64, i64, i64), p2: &(i64, i64, i64)) -> Vec<EntityID> {
        let mut mvec = Vec::new();

        let unit_square = AABB::from_corners(p1.clone(), p2.clone());
        for player in self.positions.locate_in_envelope(&unit_square) {
            mvec.push(player.entity_id.clone());
        }
        mvec
    }

    // World initialization function.
    pub fn init_world() -> RTree<Voxel> {
        let mut batchvec = Vec::new();
        for x in 0..100 {
            for y in 0..100 {
                for z in 0..=1 {
                    batchvec.push(Voxel {
                        voxel_type: VoxelType::Gas(GasType::Air),
                        floor_type: FloorType::Raw(SolidType::Stone(StoneType::Marble)),
                        voxel_pos: (x, y, z),
                    });
                }
            }
        }
        let newtree = RTree::bulk_load(batchvec);

        newtree
    }

    pub fn set_voxel_at(&mut self, point: &MyPoint, vox: VoxelType) {
        if let Some(boop) = self.voxeltile_grid.locate_at_point_mut(point) {
            boop.voxel_type = vox;
        } else {
            self.voxeltile_grid.insert(Voxel {
                floor_type: FloorType::Raw(SolidType::Granular(GranularType::Dirt)),
                voxel_pos: point.clone(),
                voxel_type: vox.clone(),
            })
        }
    }

    pub fn get_voxel_at(&self, point: &MyPoint) -> Option<VoxelType> {
        if let Some(boop) = self.voxeltile_grid.locate_at_point(point) {
            Some(boop.voxel_type.clone())
        } else {
            None
        }
    }

    pub fn voxel_blocks_movement_at(&self, point: &MyPoint) -> bool {
        if let Some(got_point) = self.get_voxel_at(point) {
            match got_point {
                VoxelType::Solid(_) => {
                    return true;
                }
                VoxelType::Gas(_)
                | VoxelType::Liquid(_)
                | VoxelType::Plant(_)
                | VoxelType::Furniture(_) => {
                    return false;
                }
            }
        } else {
            if point.0 > 0 && point.1 > 0 && point.2 > 0 {
                return false;
            } else {
                return true;
            }
        }
    }

    pub fn entity_blocks_movement_at(&self, point: &MyPoint) -> bool {
        let ents_at = self.get_ents_at_point(point);

        for ent in ents_at {
            if let Some(etype) = self.entity_types.get(&ent) {
                match etype {
                    EntityType::Item => {}
                    EntityType::Player => {
                        return true;
                    }
                    EntityType::Monster => {
                        return true;
                    }
                }
            }

            // let obj = self.en
        }

        if point.0 > 0 && point.1 > 0 && point.2 > 0 {
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
            let same_z = locate_square(e_pos, w_radius as i64, h_radius as i64, 0);

            let local_ents = self.positions.locate_in_envelope(&same_z);
            let local_voxels = self.voxeltile_grid.locate_in_envelope(&same_z);

            let local_voxel_diffs = self.voxeltile_diffs.locate_in_envelope(&same_z);

            let local_actions = self
                .output_queue
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE);

            let bottom_left_of_game_screen = (
                e_pos.0 - w_radius as i64,
                e_pos.1 - h_radius as i64,
                e_pos.2,
            );

            // THIS GRID IS INDEXD Y FIRST
            let mut voxel_grid = create_2d_array(render_width.into(), render_height.into());

            let mut ent_vec = Vec::new();

            for pc in local_ents {
                let relative_point_x = pc.point.0 - bottom_left_of_game_screen.0;
                let relative_point_y = pc.point.1 - bottom_left_of_game_screen.1;
                if let Some(ent_type) = self.entity_types.get(&pc.entity_id) {
                    let et = ent_type.clone();
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
            //   println!("DESSSSSS");
            RenderPacket::new()
        }
    }

    pub fn create_game_data_packet_for_entity(&self, ent: &EntityID) -> Option<GameDataPacket> {
        if let Some(e_pos) = self.ent_loc_index.get(ent) {
            let local_ents = self
                .positions
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE * 2);
            let local_voxels = self
                .voxeltile_diffs
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE / 2);
            let local_actions = self
                .output_queue
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE / 8);

            let mut e_info = Vec::new();
            let mut v_diffs = Vec::new();
            let mut actions = Vec::new();

            for pc in local_ents {
                e_info.push(EntityPacket {
                    entity_pos: pc.point.clone(),
                    entity_id: pc.entity_id.clone(),
                    entity_type: self.entity_types.get(ent).unwrap().clone(),
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
            match got_point {
                VoxelType::Solid(_) => {
                    return true;
                }
                VoxelType::Gas(_)
                | VoxelType::Liquid(_)
                | VoxelType::Plant(_)
                | VoxelType::Furniture(_) => {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    pub fn set_ent_loc(&mut self, ent: &EntityID, destination: &MyPoint) {
        if let Some(xyz) = self.ent_loc_index.get(ent) {
            self.positions.remove(&PositionComponent {
                entity_id: ent.clone(),
                point: xyz.clone(),
            });
            self.positions.insert(PositionComponent {
                entity_id: ent.clone(),
                point: destination.clone(),
            });
            self.ent_loc_index.insert(ent.clone(), destination.clone());
        }
    }

    pub fn move_entity_in_direction(
        &mut self,
        ent: &EntityID,
        cd: &CardinalDirection,
    ) -> SuccessType {
        if let Some(xyz) = self.ent_loc_index.get(ent) {
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
