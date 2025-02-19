use std::collections::HashSet;
use bevy::asset::prelude::*;
use bevy::ecs::prelude::*;
use super::assets::*;


#[derive(Resource,Debug, Default)]
pub struct UiModifiedAssets {
    pub modified_handles : HashSet<AssetId<UiAsset>>,
}
