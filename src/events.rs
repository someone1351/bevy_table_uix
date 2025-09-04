use bevy::prelude::{Entity, Event};

#[derive(Debug,Event,Clone)]
pub struct UixUserEvent {
    pub entity : Entity,
    pub event : script_lang::StringT,
    pub params : Vec<script_lang::Value>,
}

