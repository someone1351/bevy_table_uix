
use std::collections::HashSet;

use bevy::{ecs::prelude::*, utils::HashMap};
use bevy::asset::prelude::*;
use bevy_table_ui::UiLayoutComputed;

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
pub struct UixEventListener {
    pub event_listeners : HashMap<script_lang::StringT,Vec<script_lang::Value>>,
}

#[derive(Component,Default)]
pub struct UixName {
    pub names : HashSet<script_lang::StringT>,
}

#[derive(Component,Default)]
pub struct UixEnv {
    pub env : HashMap<script_lang::StringT,Vec<Entity>>,
}

#[derive(Component)]
#[require(UiLayoutComputed)]
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

