
// use std::collections::HashSet;

use bevy::platform::collections::HashSet;
use bevy::{ecs::prelude::*, platform::collections::HashMap};
use bevy::asset::prelude::*;
use bevy_table_ui::{UiLayoutComputed, UiRoot, };

// use std::collections::{BTreeSet, HashMap};

// use crate::table_ui::{UiSize, UiVal};

use super::{
    // systems::*,
    assets::*,
    // resources::*,
    // systems::*,
};

// use script_lang as script;

#[derive(Component,Default)]
#[require(UiLayoutComputed)]
pub struct UixEventListener {
    pub event_listeners : HashMap<script_lang::StringT,Vec<script_lang::Value>>,
}

#[derive(Component,Default)]
#[require(UiLayoutComputed)]
pub struct UixName {
    pub names : HashSet<script_lang::StringT>,
}

// #[derive(Component,Default)]
// #[require(UiLayoutComputed)]
// pub struct UixEnv {
//     pub env : HashMap<script_lang::StringT,Vec<script_lang::Value>>,
// }

#[derive(Component,)]
#[require(UiLayoutComputed)]
pub struct UixSelf {
    pub entity : script_lang::Value,
}

impl UixSelf {
    pub fn new(entity:Entity) -> Self {
        Self { entity: script_lang::Value::custom_unmanaged(entity) }
    }
    // pub fn new_from_world(world : &mut World,entity:Entity) -> script_lang::Value {
    //     world.entity_mut(entity).entry::<Self>().or_insert_with(||Self::new(entity)).get_mut().entity.clone()
    // }
}

#[derive(Component)]
#[require(UiLayoutComputed,UiRoot)]
pub struct UixFromAsset {
    pub handle : Handle<UiAsset>,
    pub all_dep_ids : HashSet<AssetId<UiAsset>>,
    pub init : bool,
    pub loaded : bool,
    // pub build : Option<script::BuildT>,
    // pub build_init : bool,
    // // post_loaded : bool,
    // // deps_loaded : bool,
    // // deps : HashMap<Handle<ConfLangAsset>, bool>,
}

impl UixFromAsset {
    pub fn new(handle : Handle<UiAsset>) -> Self { //HandleUntyped
        Self {
            handle,
            init : false,
            loaded : false,
            all_dep_ids:Default::default(),
            // build:None,
            // build_init:false,
            // // post_loaded:true,
            // // deps_loaded:false,
            // // deps:HashMap::new(),
        }
    }
}

