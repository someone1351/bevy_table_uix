
use bevy::prelude::*;
use super::{resources::*, mapping::*,assets::*};
use bevy_axis_input as axis_input;

pub fn init(
    asset_server: Res<AssetServer>,
    mut input_config: ResMut<InputConfig>,
) {
    input_config.default_config=asset_server.load("default.input_conf");
}

pub fn on_modified(
    mut input_config: ResMut<InputConfig>,
    mut asset_events: EventReader<AssetEvent<InputAsset>>,

) {
    for ev in asset_events.read() {
        match ev {
            AssetEvent::Modified { id, } =>  {
                if input_config.default_config.id()==*id {
                    input_config.default_config_loaded=false;
                    println!("asset modified {:?}", id);
                }
            }
            _ =>{}
        }
    }
}

pub fn load(
    mut input_config: ResMut<InputConfig>,
    input_assets: ResMut<Assets<InputAsset>>,
    mut axis_input: ResMut<axis_input::InputMap<Mapping>>,
) {
    if !input_config.default_config_loaded {
        if let Some(InputAsset{conf})=input_assets.get(&input_config.default_config) {
            input_config.default_config_loaded=true;

            conf.root().walk(|walk|{
                // walk.skip_exit();

                match walk.record().node_label().unwrap_or("") {
                    "binding" => {
                        let profile=walk.ancestor(1).record().first().str();
                        let mapping=walk.ancestor(0).record().first().get_parsed::<Mapping>().unwrap();
                        // let bindings=walk.record().param_group("bindings").values().parsed::<axis_input::Binding>().collect::<Vec<_>>();
                        let bindings=walk.record().param_group("bindings").values().map(|x|x.get_parsed().unwrap()).collect::<Vec<_>>();
                        let scale=walk.record().param_group("scale").first().get_parsed::<f32>().unwrap_or(1.0);
                        let primary_dead=walk.record().param_group("primary_dead").first().get_parsed::<f32>().unwrap_or(0.0);
                        let modifier_dead=walk.record().param_group("modifier_dead").first().get_parsed::<f32>().unwrap_or(0.0);

                        // println!("== {profile:?} :: {mapping:?} :: {bindings:?} :: {scale} :: {primary_dead} :: {modifier_dead}");
                        input_config.set_binding(profile, mapping, bindings, scale, primary_dead, modifier_dead);
                    }
                    "invert" => {
                        let profile=walk.parent().record().first().str();
                        let mapping=walk.record().value(0).get_parsed::<Mapping>().unwrap();
                        let invert= walk.record().value(1).get_parsed::<bool>().unwrap();
                        input_config.set_mapping_invert(profile, mapping, invert);

                        // println!("invert {:?} {:?} {:?}",profile,mapping,invert);
                    }
                    "scale" => {
                        let profile=walk.parent().record().first().str();
                        let mapping=walk.record().value(0).get_parsed::<Mapping>().unwrap();
                        let scale= walk.record().value(1).get_parsed::<f32>().unwrap();

                        input_config.set_mapping_scale(profile, mapping, scale);
                        // println!("sensitivity {:?} {:?} {:?}",profile,mapping,scale);
                    }
                    "repeat" => {
                        let mapping= walk.record().value(0).get_parsed::<Mapping>().unwrap();
                        let repeat= walk.record().value(1).get_parsed::<f32>().unwrap();
                        input_config.set_mapping_repeat(mapping, repeat);
                        // println!("repeat {:?} {:?}",mapping,repeat);
                    }
                    "exclude" => {
                        let binding= walk.record().value(0).get_parsed::<axis_input::Binding>().unwrap();
                        input_config.set_bind_mode_exclude(binding);
                        // println!("exclude {:?}",binding);
                    }
                    _ =>{}
                }
            }).unwrap();

            input_config.apply(&mut axis_input);
        }
    }
}