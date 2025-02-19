
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use bevy::asset::Handle;
use bevy::ecs::prelude::*;
use super::mapping::*;

use bevy_chair_input_map::{self as input_map, SetMappingBind, SetMappingRepeat};
use super::assets::*;

// #[derive(Default)]
pub struct InputConfigMapping {
    invert:bool,
    scale:f32,
    
    bindings:HashMap<Vec<input_map::Binding>,(f32,f32,f32)>,
}
impl Default for InputConfigMapping {
    fn default() -> Self {
        Self { invert: false, scale: 1.0, bindings: HashMap::new() }
    }
}
#[derive(Default)]
pub struct InputConfigProfile {
    mappings:HashMap<Mapping,InputConfigMapping>,
}

#[derive(Resource,Default)]
pub struct InputConfig {
    pub default_config:Handle<InputAsset>,
    pub default_config_loaded:bool,
    
    binding_excludes:HashSet<input_map::Binding>,
    mapping_repeats:HashMap<Mapping,f32>,
    profiles:HashMap<String,InputConfigProfile>,
    device_deadzones:HashMap<input_map::Device,HashMap<input_map::Binding,(f32,f32)>>,
}

impl InputConfig {
    pub fn clear_binding<B:AsRef<[input_map::Binding]>>(&mut self, profile:&str, mapping:Mapping, bindings:B) {
        if let Some(profile)=self.profiles.get_mut(profile) {
            if let Some(mapping_config)=profile.mappings.get_mut(&mapping) {
                mapping_config.bindings.remove(&bindings.as_ref().to_vec());
            }
        }
    }
    pub fn clear_profile_mapping(&mut self, profile:&str, mapping:Mapping) {
        if let Some(profile)=self.profiles.get_mut(profile) {
            profile.mappings.remove(&mapping);
        }
    }
    pub fn get_mapping_bindings(&mut self, profile:&str) -> Vec<(Mapping,Vec<Vec<input_map::Binding>>,Option<f32>,Option<bool>)> {
        let Some(profile_config)=self.profiles.get(profile) else {return Default::default()};

        profile_config.mappings.iter().map(|(mapping,mapping_config)|{
            let b=mapping_config.bindings.iter().map(|(binding_group,..)|binding_group.clone()).collect();
            (mapping.clone(),b,None,None)
        }).collect()
    }

    pub fn set_mapping_invert(&mut self, profile:&str, mapping:Mapping, invert:bool) {
        self.profiles.entry(profile.to_string()).or_default()
            .mappings.entry(mapping).or_default().invert=invert;
    }
    pub fn set_mapping_scale(&mut self, profile:&str, mapping:Mapping, scale:f32) {
        self.profiles.entry(profile.to_string()).or_default()
            .mappings.entry(mapping).or_default().scale=scale;
    }
    pub fn set_binding<B:AsRef<[input_map::Binding]>>(&mut self, profile:&str, mapping:Mapping, bindings:B,scale:f32, primary_dead:f32, modifier_dead:f32) {
        self.profiles.entry(profile.to_string()).or_default()
            .mappings.entry(mapping).or_default()
            .bindings.insert(bindings.as_ref().to_vec(), (scale,primary_dead,modifier_dead));
    }
    
    pub fn set_mapping_repeat(&mut self, mapping:Mapping,repeat:f32) {
        self.mapping_repeats.insert(mapping,repeat);
    }
    pub fn set_bind_mode_exclude(&mut self, binding:input_map::Binding) {
        self.binding_excludes.insert(binding);
    }

    pub fn clear_profiles(&mut self) {
        self.profiles.clear();
    }

    pub fn clear_profile(&mut self,profile:&str) {
        self.profiles.remove(profile);
    }

    pub fn set_device_dead_zones(&mut self, device:input_map::Device,binding:input_map::Binding,neg:f32,pos:f32) {
        self.device_deadzones.entry(device).or_default().insert(binding, (neg,pos));
    }

    pub fn clear_mapping_repeats(&mut self) {
        self.mapping_repeats.clear();
    }
    pub fn clear_bind_mode_excludes(&mut self) {
        self.binding_excludes.clear();
    }
    pub fn clear_device_dead_zones(&mut self, device:input_map::Device) {
        if let Some(x)=self.device_deadzones.get_mut(&device) {
            x.clear();
        }
    }

    pub fn clear(&mut self) {
        self.binding_excludes.clear();
        self.mapping_repeats.clear();
        self.profiles.clear();
        self.device_deadzones.clear();
    }


    pub fn apply(&self, input_map:&mut input_map::InputMap<Mapping>) {

        input_map.set_bind_mode_excludes(self.binding_excludes.iter().cloned());
        input_map.set_mapping_repeats(self.mapping_repeats.iter().map(|(k,&v)|SetMappingRepeat{mapping:k.clone(),delay:v})); //(k.clone(),v)
        input_map.set_player_mapping_binds(0, self.profiles.iter().map(|(_,profile)|{
            profile.mappings.iter().map(|(mapping,mapping_config)|{
                mapping_config.bindings.iter().map(|(bindings,&(scale,primary_dead,modifier_dead))|{
                    let scale=scale*mapping_config.scale*mapping_config.invert.then_some(-1.0).unwrap_or(1.0);
                    // (mapping.clone(),bindings.clone(),scale,primary_dead,modifier_dead)
                    SetMappingBind{mapping:mapping.clone(),bindings:bindings.clone(),scale,primary_dead,modifier_dead}
                })
            }).flatten()
        }).flatten());
    }
}


