use std::{collections::HashSet, sync::{Arc, Mutex}};

// use std::collections::HashSet;
use bevy::asset::prelude::*;
use bevy::ecs::prelude::*;
// use bevy::platform::collections::HashSet;
use script_lang::{GcScope, LibScope};
use crate::script_libs;

use super::assets::*;


#[derive(Resource,Debug, Default)]
pub struct UiModifiedAssets {
    pub modified_handles : HashSet<AssetId<UiAsset>>,
}

#[derive(Resource,)]
pub struct UixLibScope(pub Arc<LibScope<World>>);

impl Default for UixLibScope {
    fn default() -> Self {
        let mut lib_scope=LibScope::new_full();

        script_libs::register_attribs(&mut lib_scope);
        script_libs::register_ui_val(&mut lib_scope);
        script_libs::register_entity_attribs(&mut lib_scope);
        script_libs::register_rect(&mut lib_scope);


        script_libs::register_stuff(&mut lib_scope);
        script_libs::register_misc(&mut lib_scope);
        script_libs::register_events(&mut lib_scope);


        script_libs::register(&mut lib_scope);

        Self(Arc::new(lib_scope))
    }
}

#[derive(Resource,Default)]
pub struct UixGcScope(pub Arc<Mutex<GcScope>>);
