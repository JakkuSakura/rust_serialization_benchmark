mod minecraft_savedata_capnp;
mod minecraft_savedata_generated;

use core::pin::Pin;
use minecraft_savedata_capnp as cp;
use crate::{Generate, bench_capnp, bench_flatbuffers, generate_vec};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
pub use minecraft_savedata_generated::minecraft_savedata as fb;
use rand::Rng;
use rkyv::Archived;

#[derive(
    Clone, Copy,
    abomonation_derive::Abomonation,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Serialize, serde::Deserialize,
)]
#[archive(copy)]
#[repr(u8)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Generate for GameType {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        match rand.gen_range(0..4) {
            0 => GameType::Survival,
            1 => GameType::Creative,
            2 => GameType::Adventure,
            3 => GameType::Spectator,
            _ => unreachable!(),
        }
    }
}

impl Into<fb::GameType> for GameType {
    fn into(self) -> fb::GameType {
        match self {
            GameType::Survival => fb::GameType::Survival,
            GameType::Creative => fb::GameType::Creative,
            GameType::Adventure => fb::GameType::Adventure,
            GameType::Spectator => fb::GameType::Spectator,
        }
    }
}

impl Into<cp::GameType> for GameType {
    fn into(self) -> cp::GameType {
        match self {
            GameType::Survival => cp::GameType::Survival,
            GameType::Creative => cp::GameType::Creative,
            GameType::Adventure => cp::GameType::Adventure,
            GameType::Spectator => cp::GameType::Spectator,
        }
    }
}

#[derive(
    abomonation_derive::Abomonation,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct Item {
    pub count: i8,
    pub slot: u8,
    pub id: String,
}

impl Generate for Item {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const IDS: [&'static str; 8] = [
            "dirt",
            "stone",
            "pickaxe",
            "sand",
            "gravel",
            "shovel",
            "chestplate",
            "steak",
        ];
        Self {
            count: rng.gen(),
            slot: rng.gen(),
            id: IDS[rng.gen_range(0..IDS.len())].to_string(),
        }
    }
}

impl<'a> bench_flatbuffers::Serialize<'a> for Item {
    type Target = fb::Item<'a>;

    fn serialize_fb<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let id = Some(builder.create_string(&self.id));
        Self::Target::create(builder, &fb::ItemArgs {
            count: self.count,
            slot: self.slot,
            id,
        })
    }
}

impl<'a> bench_capnp::Serialize<'a> for Item {
    type Reader = cp::item::Reader<'a>;
    type Builder = cp::item::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_count(self.count);
        builder.set_slot(self.slot);
        builder.set_id(&self.id);
    }
}

#[derive(
    Clone, Copy,
    abomonation_derive::Abomonation,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Serialize, serde::Deserialize,
)]
#[archive(copy)]
pub struct Abilities {
    pub walk_speed: f32,
    pub fly_speed: f32,
    pub may_fly: bool,
    pub flying: bool,
    pub invulnerable: bool,
    pub may_build: bool,
    pub instabuild: bool,
}

impl Generate for Abilities {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            walk_speed: rng.gen(),
            fly_speed: rng.gen(),
            may_fly: rng.gen_bool(0.5),
            flying: rng.gen_bool(0.5),
            invulnerable: rng.gen_bool(0.5),
            may_build: rng.gen_bool(0.5),
            instabuild: rng.gen_bool(0.5),
        }
    }
}

impl Into<fb::Abilities> for Abilities {
    fn into(self) -> fb::Abilities {
        fb::Abilities::new(
            self.walk_speed,
            self.fly_speed,
            self.may_fly,
            self.flying,
            self.invulnerable,
            self.may_build,
            self.instabuild,
        )
    }
}

impl<'a> bench_capnp::Serialize<'a> for Abilities {
    type Reader = cp::abilities::Reader<'a>;
    type Builder = cp::abilities::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_walk_speed(self.walk_speed);
        builder.set_fly_speed(self.fly_speed);
        builder.set_may_fly(self.may_fly);
        builder.set_flying(self.flying);
        builder.set_invulnerable(self.invulnerable);
        builder.set_may_build(self.may_build);
        builder.set_instabuild(self.instabuild);
    }
}

#[derive(
    abomonation_derive::Abomonation,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct Entity {
    pub id: String,
    pub pos: (f64, f64, f64),
    pub motion: (f64, f64, f64),
    pub rotation: (f32, f32),
    pub fall_distance: f32,
    pub fire: u16,
    pub air: u16,
    pub on_ground: bool,
    pub no_gravity: bool,
    pub invulnerable: bool,
    pub portal_cooldown: i32,
    pub uuid: [u32; 4],
    pub custom_name: Option<String>,
    pub custom_name_visible: bool,
    pub silent: bool,
    pub glowing: bool,
}

impl Generate for Entity {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const IDS: [&'static str; 8] = [
            "cow", "sheep", "zombie", "skeleton", "spider", "creeper", "parrot", "bee",
        ];
        const CUSTOM_NAMES: [&'static str; 8] = [
            "rainbow", "princess", "steve", "johnny", "missy", "coward", "fairy", "howard",
        ];

        Self {
            id: IDS[rng.gen_range(0..IDS.len())].to_string(),
            pos: <(f64, f64, f64) as Generate>::generate(rng),
            motion: <(f64, f64, f64) as Generate>::generate(rng),
            rotation: <(f32, f32) as Generate>::generate(rng),
            fall_distance: rng.gen(),
            fire: rng.gen(),
            air: rng.gen(),
            on_ground: rng.gen_bool(0.5),
            no_gravity: rng.gen_bool(0.5),
            invulnerable: rng.gen_bool(0.5),
            portal_cooldown: rng.gen(),
            uuid: <[u32; 4] as Generate>::generate(rng),
            custom_name: <Option<()> as Generate>::generate(rng)
                .map(|_| CUSTOM_NAMES[rng.gen_range(0..CUSTOM_NAMES.len())].to_string()),
            custom_name_visible: rng.gen_bool(0.5),
            silent: rng.gen_bool(0.5),
            glowing: rng.gen_bool(0.5),
        }
    }
}

impl<'a> bench_flatbuffers::Serialize<'a> for Entity {
    type Target = fb::Entity<'a>;

    fn serialize_fb<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
    'a: 'b,
    {
        let id = Some(builder.create_string(&self.id));
        let custom_name = self.custom_name.as_ref().map(|name| builder.create_string(&name));
        Self::Target::create(builder, &fb::EntityArgs {
            id,
            pos: Some(&fb::Vector3d::new(self.pos.0, self.pos.1, self.pos.2)),
            motion: Some(&fb::Vector3d::new(self.motion.0, self.motion.1, self.motion.2)),
            rotation: Some(&fb::Vector2f::new(self.rotation.0, self.rotation.1)),
            fall_distance: self.fall_distance,
            fire: self.fire,
            air: self.air,
            on_ground: self.on_ground,
            no_gravity: self.no_gravity,
            invulnerable: self.invulnerable,
            portal_cooldown: self.portal_cooldown,
            uuid: Some(&fb::Uuid::new(self.uuid[0], self.uuid[1], self.uuid[2], self.uuid[3])),
            custom_name,
            custom_name_visible: self.custom_name_visible,
            silent: self.silent,
            glowing: self.glowing,
        })
    }
}

impl<'a> bench_capnp::Serialize<'a> for Entity {
    type Reader = cp::entity::Reader<'a>;
    type Builder = cp::entity::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_id(&self.id);
        let mut pos = builder.reborrow().init_pos();
        pos.set_x(self.pos.0);
        pos.set_y(self.pos.1);
        pos.set_z(self.pos.2);
        let mut motion = builder.reborrow().init_motion();
        motion.set_x(self.motion.0);
        motion.set_y(self.motion.1);
        motion.set_z(self.motion.2);
        let mut rotation = builder.reborrow().init_rotation();
        rotation.set_x(self.rotation.0);
        rotation.set_y(self.rotation.1);
        builder.set_fall_distance(self.fall_distance);
        builder.set_fire(self.fire);
        builder.set_air(self.air);
        builder.set_on_ground(self.on_ground);
        builder.set_no_gravity(self.no_gravity);
        builder.set_invulnerable(self.invulnerable);
        builder.set_portal_cooldown(self.portal_cooldown);
        let mut uuid = builder.reborrow().init_uuid();
        uuid.set_x0(self.uuid[0]);
        uuid.set_x1(self.uuid[1]);
        uuid.set_x2(self.uuid[2]);
        uuid.set_x3(self.uuid[3]);
        if let Some(ref custom_name) = self.custom_name {
            builder.set_custom_name(custom_name);
        }
        builder.set_custom_name_visible(self.custom_name_visible);
        builder.set_silent(self.silent);
        builder.set_glowing(self.glowing);
    }
}

#[derive(
    abomonation_derive::Abomonation,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct RecipeBook {
    pub recipes: Vec<String>,
    pub to_be_displayed: Vec<String>,
    pub is_filtering_craftable: bool,
    pub is_gui_open: bool,
    pub is_furnace_filtering_craftable: bool,
    pub is_furnace_gui_open: bool,
    pub is_blasting_furnace_filtering_craftable: bool,
    pub is_blasting_furnace_gui_open: bool,
    pub is_smoker_filtering_craftable: bool,
    pub is_smoker_gui_open: bool,
}

impl Generate for RecipeBook {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const RECIPES: [&'static str; 8] = [
            "pickaxe",
            "torch",
            "bow",
            "crafting table",
            "furnace",
            "shears",
            "arrow",
            "tnt",
        ];
        const MAX_RECIPES: usize = 30;
        const MAX_DISPLAYED_RECIPES: usize = 10;
        Self {
            recipes: generate_vec::<_, ()>(rng, 0..MAX_RECIPES)
                .iter()
                .map(|_| RECIPES[rng.gen_range(0..RECIPES.len())].to_string())
                .collect(),
            to_be_displayed: generate_vec::<_, ()>(rng, 0..MAX_DISPLAYED_RECIPES)
                .iter()
                .map(|_| RECIPES[rng.gen_range(0..RECIPES.len())].to_string())
                .collect(),
            is_filtering_craftable: rng.gen_bool(0.5),
            is_gui_open: rng.gen_bool(0.5),
            is_furnace_filtering_craftable: rng.gen_bool(0.5),
            is_furnace_gui_open: rng.gen_bool(0.5),
            is_blasting_furnace_filtering_craftable: rng.gen_bool(0.5),
            is_blasting_furnace_gui_open: rng.gen_bool(0.5),
            is_smoker_filtering_craftable: rng.gen_bool(0.5),
            is_smoker_gui_open: rng.gen_bool(0.5),
        }
    }
}

impl<'a> bench_flatbuffers::Serialize<'a> for RecipeBook {
    type Target = fb::RecipeBook<'a>;

    fn serialize_fb<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut recipes = Vec::new();
        for recipe in self.recipes.iter() {
            recipes.push(builder.create_string(recipe));
        }
        let recipes = Some(builder.create_vector(&recipes));

        let mut to_be_displayed = Vec::new();
        for name in self.to_be_displayed.iter() {
            to_be_displayed.push(builder.create_string(name));
        }
        let to_be_displayed = Some(builder.create_vector(&to_be_displayed));

        Self::Target::create(builder, &fb::RecipeBookArgs {
            recipes,
            to_be_displayed,
            is_filtering_craftable: self.is_filtering_craftable,
            is_gui_open: self.is_gui_open,
            is_furnace_filtering_craftable: self.is_furnace_filtering_craftable,
            is_furnace_gui_open: self.is_furnace_gui_open,
            is_blasting_furnace_filtering_craftable: self.is_blasting_furnace_filtering_craftable,
            is_blasting_furnace_gui_open: self.is_blasting_furnace_gui_open,
            is_smoker_filtering_craftable: self.is_smoker_filtering_craftable,
            is_smoker_gui_open: self.is_smoker_gui_open,
        })
    }
}

impl<'a> bench_capnp::Serialize<'a> for RecipeBook {
    type Reader = cp::recipe_book::Reader<'a>;
    type Builder = cp::recipe_book::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut recipes = builder.reborrow().init_recipes(self.recipes.len() as u32);
        for (i, recipe) in self.recipes.iter().enumerate() {
            recipes.set(i as u32, recipe);
        }
        let mut to_be_displayed = builder.reborrow().init_to_be_displayed(self.to_be_displayed.len() as u32);
        for (i, name) in self.to_be_displayed.iter().enumerate() {
            to_be_displayed.set(i as u32, name);
        }
        builder.set_is_filtering_craftable(self.is_filtering_craftable);
        builder.set_is_gui_open(self.is_gui_open);
        builder.set_is_furnace_filtering_craftable(self.is_furnace_filtering_craftable);
        builder.set_is_furnace_gui_open(self.is_furnace_gui_open);
        builder.set_is_blasting_furnace_filtering_craftable(self.is_blasting_furnace_filtering_craftable);
        builder.set_is_blasting_furnace_gui_open(self.is_blasting_furnace_gui_open);
        builder.set_is_smoker_filtering_craftable(self.is_smoker_filtering_craftable);
        builder.set_is_smoker_gui_open(self.is_smoker_gui_open);
    }
}

#[derive(
    abomonation_derive::Abomonation,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct Player {
    pub game_type: GameType,
    pub previous_game_type: GameType,
    pub score: u64,
    pub dimension: String,
    pub selected_item_slot: u32,
    pub selected_item: Item,
    pub spawn_dimension: Option<String>,
    pub spawn_x: i64,
    pub spawn_y: i64,
    pub spawn_z: i64,
    pub spawn_forced: Option<bool>,
    pub sleep_timer: u16,
    pub food_exhaustion_level: f32,
    pub food_saturation_level: f32,
    pub food_tick_timer: u32,
    pub xp_level: u32,
    pub xp_p: f32,
    pub xp_total: i32,
    pub xp_seed: i32,
    pub inventory: Vec<Item>,
    pub ender_items: Vec<Item>,
    pub abilities: Abilities,
    pub entered_nether_position: Option<(f64, f64, f64)>,
    pub root_vehicle: Option<([u32; 4], Entity)>,
    pub shoulder_entity_left: Option<Entity>,
    pub shoulder_entity_right: Option<Entity>,
    pub seen_credits: bool,
    pub recipe_book: RecipeBook,
}

impl ArchivedPlayer {
    pub fn game_type_pin(self: Pin<&mut Self>) -> Pin<&mut GameType> {
        unsafe { self.map_unchecked_mut(|s| &mut s.game_type) }
    }

    pub fn spawn_x_pin(self: Pin<&mut Self>) -> Pin<&mut i64> {
        unsafe { self.map_unchecked_mut(|s| &mut s.spawn_x) }
    }

    pub fn spawn_y_pin(self: Pin<&mut Self>) -> Pin<&mut i64> {
        unsafe { self.map_unchecked_mut(|s| &mut s.spawn_y) }
    }

    pub fn spawn_z_pin(self: Pin<&mut Self>) -> Pin<&mut i64> {
        unsafe { self.map_unchecked_mut(|s| &mut s.spawn_z) }
    }
}

impl Generate for Player {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const DIMENSIONS: [&'static str; 3] = ["overworld", "nether", "end"];
        const MAX_ITEMS: usize = 40;
        const MAX_ENDER_ITEMS: usize = 27;
        Self {
            game_type: GameType::generate(rng),
            previous_game_type: GameType::generate(rng),
            score: rng.gen(),
            dimension: DIMENSIONS[rng.gen_range(0..DIMENSIONS.len())].to_string(),
            selected_item_slot: rng.gen(),
            selected_item: Item::generate(rng),
            spawn_dimension: <Option<()> as Generate>::generate(rng)
                .map(|_| DIMENSIONS[rng.gen_range(0..DIMENSIONS.len())].to_string()),
            spawn_x: rng.gen(),
            spawn_y: rng.gen(),
            spawn_z: rng.gen(),
            spawn_forced: <Option<bool> as Generate>::generate(rng),
            sleep_timer: rng.gen(),
            food_exhaustion_level: rng.gen(),
            food_saturation_level: rng.gen(),
            food_tick_timer: rng.gen(),
            xp_level: rng.gen(),
            xp_p: rng.gen(),
            xp_total: rng.gen(),
            xp_seed: rng.gen(),
            inventory: generate_vec(rng, 0..MAX_ITEMS),
            ender_items: generate_vec(rng, 0..MAX_ENDER_ITEMS),
            abilities: Abilities::generate(rng),
            entered_nether_position: <Option<(f64, f64, f64)> as Generate>::generate(rng),
            root_vehicle: <Option<([u32; 4], Entity)> as Generate>::generate(rng),
            shoulder_entity_left: <Option<Entity> as Generate>::generate(rng),
            shoulder_entity_right: <Option<Entity> as Generate>::generate(rng),
            seen_credits: rng.gen_bool(0.5),
            recipe_book: RecipeBook::generate(rng),
        }
    }
}

impl<'a> bench_flatbuffers::Serialize<'a> for Player {
    type Target = fb::Player<'a>;

    fn serialize_fb<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let dimension = Some(builder.create_string(&self.dimension));
        let selected_item = Some(self.selected_item.serialize_fb(builder));
        let spawn_dimension = self.spawn_dimension.as_ref().map(|d| builder.create_string(d));

        let mut inventory = Vec::new();
        for inventory_item in self.inventory.iter() {
            inventory.push(inventory_item.serialize_fb(builder));
        }
        let inventory = Some(builder.create_vector(&inventory));

        let mut ender_items = Vec::new();
        for ender_item in self.ender_items.iter() {
            ender_items.push(ender_item.serialize_fb(builder));
        }
        let ender_items = Some(builder.create_vector(&ender_items));

        let entered_nether_position = self.entered_nether_position.map(|p| {
            fb::Vector3d::new(p.0, p.1, p.2)
        });
        let root_vehicle = self.root_vehicle.as_ref().map(|v| {
            let entity = Some(v.1.serialize_fb(builder));
            fb::Vehicle::create(builder, &fb::VehicleArgs {
                param_0: v.0[0],
                param_1: v.0[1],
                param_2: v.0[2],
                param_3: v.0[3],
                entity,
            })
        });
        let shoulder_entity_left = self.shoulder_entity_left.as_ref().map(|e| e.serialize_fb(builder));
        let shoulder_entity_right = self.shoulder_entity_right.as_ref().map(|e| e.serialize_fb(builder));
        let recipe_book = Some(self.recipe_book.serialize_fb(builder));

        Self::Target::create(builder, &fb::PlayerArgs {
            game_type: self.game_type.into(),
            previous_game_type: self.previous_game_type.into(),
            score: self.score,
            dimension,
            selected_item_slot: self.selected_item_slot,
            selected_item,
            spawn_dimension,
            spawn_x: self.spawn_x,
            spawn_y: self.spawn_y,
            spawn_z: self.spawn_z,
            spawn_forced: self.spawn_forced.unwrap_or(false),
            sleep_timer: self.sleep_timer,
            food_exhaustion_level: self.food_exhaustion_level,
            food_saturation_level: self.food_saturation_level,
            food_tick_timer: self.food_tick_timer,
            xp_level: self.xp_level,
            xp_p: self.xp_p,
            xp_total: self.xp_total,
            xp_seed: self.xp_seed,
            inventory,
            ender_items,
            abilities: Some(&self.abilities.into()),
            entered_nether_position: entered_nether_position.as_ref(),
            root_vehicle,
            shoulder_entity_left,
            shoulder_entity_right,
            seen_credits: self.seen_credits,
            recipe_book,
        })
    }
}

impl<'a> bench_capnp::Serialize<'a> for Player {
    type Reader = cp::player::Reader<'a>;
    type Builder = cp::player::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_game_type(self.game_type.into());
        builder.set_previous_game_type(self.previous_game_type.into());
        builder.set_score(self.score);
        builder.set_dimension(&self.dimension);
        let mut selected_item = builder.reborrow().init_selected_item();
        self.selected_item.serialize_capnp(&mut selected_item);
        let mut spawn_dimension = builder.reborrow().init_spawn_dimension();
        if let Some(ref value) = self.spawn_dimension {
            spawn_dimension.set_some(value);
        } else {
            spawn_dimension.set_none(());
        }
        let mut spawn = builder.reborrow().init_spawn();
        spawn.set_x(self.spawn_x);
        spawn.set_y(self.spawn_y);
        spawn.set_z(self.spawn_z);
        let mut spawn_forced = builder.reborrow().init_spawn_forced();
        if let Some(ref value) = self.spawn_forced {
            spawn_forced.set_some(*value);
        } else {
            spawn_forced.set_none(());
        }
        builder.set_sleep_timer(self.sleep_timer);
        builder.set_food_exhaustion_level(self.food_exhaustion_level);
        builder.set_food_saturation_level(self.food_saturation_level);
        builder.set_food_tick_timer(self.food_tick_timer);
        builder.set_xp_level(self.xp_level);
        builder.set_xp_p(self.xp_p);
        builder.set_xp_total(self.xp_total);
        builder.set_xp_seed(self.xp_seed);
        let mut inventory = builder.reborrow().init_inventory(self.inventory.len() as u32);
        for (i, value) in self.inventory.iter().enumerate() {
            value.serialize_capnp(&mut inventory.reborrow().get(i as u32));
        }
        let mut ender_items = builder.reborrow().init_ender_items(self.ender_items.len() as u32);
        for (i, value) in self.ender_items.iter().enumerate() {
            value.serialize_capnp(&mut ender_items.reborrow().get(i as u32));
        }
        self.abilities.serialize_capnp(&mut builder.reborrow().init_abilities());
        let mut entered_nether_position = builder.reborrow().init_entered_nether_position();
        if let Some(ref value) = self.entered_nether_position {
            let mut builder = entered_nether_position.init_some();
            builder.set_x(value.0);
            builder.set_y(value.1);
            builder.set_z(value.2);
        } else {
            entered_nether_position.set_none(());
        }
        let mut root_vehicle = builder.reborrow().init_root_vehicle();
        if let Some(ref value) = self.root_vehicle {
            let mut builder = root_vehicle.init_some();
            let mut uuid = builder.reborrow().init_uuid();
            uuid.set_x0(value.0[0]);
            uuid.set_x1(value.0[1]);
            uuid.set_x2(value.0[2]);
            uuid.set_x3(value.0[3]);
            value.1.serialize_capnp(&mut builder.reborrow().init_entity());
        } else {
            root_vehicle.set_none(());
        }
        let mut shoulder_entity_left = builder.reborrow().init_shoulder_entity_left();
        if let Some(ref value) = self.shoulder_entity_left {
            value.serialize_capnp(&mut shoulder_entity_left.init_some());
        } else {
            shoulder_entity_left.set_none(());
        }
        let mut shoulder_entity_right = builder.reborrow().init_shoulder_entity_right();
        if let Some(ref value) = self.shoulder_entity_right {
            value.serialize_capnp(&mut shoulder_entity_right.init_some());
        } else {
            shoulder_entity_right.set_none(());
        }
        builder.set_seen_credits(self.seen_credits);
        self.recipe_book.serialize_capnp(&mut builder.reborrow().init_recipe_book());
    }
}

#[derive(
    abomonation_derive::Abomonation,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct Players {
    pub players: Vec<Player>,
}

impl ArchivedPlayers {
    pub fn players_pin(self: Pin<&mut Self>) -> Pin<&mut Archived<Vec<Player>>> {
        unsafe { self.map_unchecked_mut(|s| &mut s.players) }
    }
}

impl<'a> bench_flatbuffers::Serialize<'a> for Players {
    type Target = fb::Players<'a>;

    fn serialize_fb<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut players = Vec::new();
        for player in self.players.iter() {
            players.push(player.serialize_fb(builder));
        }
        let players = Some(builder.create_vector(&players));
        fb::Players::create(builder, &fb::PlayersArgs {
            players,
        })
    }
}

impl<'a> bench_capnp::Serialize<'a> for Players {
    type Reader = cp::players::Reader<'a>;
    type Builder = cp::players::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut players = builder.reborrow().init_players(self.players.len() as u32);
        for (i, value) in self.players.iter().enumerate() {
            value.serialize_capnp(&mut players.reborrow().get(i as u32));
        }
    }
}