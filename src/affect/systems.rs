
use std::collections::{HashMap, HashSet};

use bevy::ecs::prelude::*;
use bevy_table_ui::{UiInteractEvent, UiInteractMessageType};
use super::components::*;
use super::values::*;


pub fn on_affects<'a>(
    mut affect_query: Query<(Entity,&UixAffect,&mut UixAffectComputed)>,
    mut commands: Commands,
    mut interact_event_reader: MessageReader<UiInteractEvent>,
) {
    let mut new_states: HashMap<Entity,HashSet<UixAffectState>>=Default::default(); //[entity][state]
    //
    for ev in interact_event_reader.read() {
        println!("e {ev}");
        let Ok((_,_, mut affect_computed))=affect_query.get_mut(ev.entity) else {continue;};

        let Some((state,device,is_end))=(match ev.event_type {
            UiInteractMessageType::FocusBegin {device, .. } => Some((UixAffectState::Focus,DeviceType::Focus(device),false)),
            UiInteractMessageType::FocusEnd { device,.. } => Some((UixAffectState::Focus,DeviceType::Focus(device),true)),
            UiInteractMessageType::CursorPressBegin{device,button,..} => Some((UixAffectState::Press(button),DeviceType::Cursor(device),false)),
            UiInteractMessageType::CursorPressEnd{device,button,..} => Some((UixAffectState::Press(button),DeviceType::Cursor(device),true)),
            UiInteractMessageType::FocusPressBegin { device, button } => Some((UixAffectState::Press(button),DeviceType::Focus(device),false)),
            UiInteractMessageType::FocusPressEnd { device, button } => Some((UixAffectState::Press(button),DeviceType::Focus(device),true)),
            UiInteractMessageType::SelectBegin => Some((UixAffectState::Select,DeviceType::None,false)),
            UiInteractMessageType::SelectEnd => Some((UixAffectState::Select,DeviceType::None,true)),
            UiInteractMessageType::CursorHoverBegin{device,..} => Some((UixAffectState::Hover,DeviceType::Cursor(device),false)),
            UiInteractMessageType::CursorHoverEnd{device,..} => Some((UixAffectState::Hover,DeviceType::Cursor(device),true)),
            UiInteractMessageType::CursorDragBegin { device, .. } => Some((UixAffectState::Drag,DeviceType::Cursor(device),false)),
            UiInteractMessageType::CursorDragEnd { device, .. } => Some((UixAffectState::Drag,DeviceType::Cursor(device),true)),
            UiInteractMessageType::CursorClick{..}=> None,
            UiInteractMessageType::FocusClick { .. } => None,
            UiInteractMessageType::CursorDragX{..} => None,
            UiInteractMessageType::CursorDragY{..} => None,
            UiInteractMessageType::CursorScroll { .. } => None,
        }) else {continue;};

        if !is_end {
            affect_computed.states.entry(state).or_default().insert(device);
            new_states.entry(ev.entity).or_default().insert(state);
        } else {
            affect_computed.states.get_mut(&state).map(|devices|devices.remove(&device));
        }

    }

    //
    for (entity, affect,mut affect_computed) in affect_query.iter_mut() {
        let new_states=new_states.get(&entity);

        for (attrib_ind,attrib) in affect.0.iter().enumerate() {

            let val_ind=attrib.states.iter().filter_map(|(k,&v)|{
                let b=affect_computed.states.get(k).map(|devices|!devices.is_empty()).unwrap_or_default();
                let b=b || new_states.map(|x|x.contains(k)).unwrap_or_default();
                b.then_some(v)
            }).fold(0, |x,y|x.max(y));

            let func=attrib.funcs.get(val_ind).unwrap().clone();
            if affect_computed.cur_attrib_inds.get(&attrib_ind).map(|&last_val_ind|last_val_ind==val_ind).unwrap_or_default() {
                continue;
            }

            affect_computed.cur_attrib_inds.insert(attrib_ind,val_ind);

            // println!("attrib {ind} : {:?} {states:?}",attrib.states);
            commands.queue(move|world:&mut World|func(entity,world,));
        }
    }
}
