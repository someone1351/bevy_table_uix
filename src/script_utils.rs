use bevy::ecs::{entity::Entity, world::World};
use script_lang::Value;

use super::components::*;

pub fn self_entity_from_world(world : &mut World,entity:Entity) -> Value {
    world.entity_mut(entity).entry::<UixSelf>().or_insert_with(||UixSelf::new(entity)).get().entity.to_weak().unwrap()
}

