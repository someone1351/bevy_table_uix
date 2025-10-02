use bevy::prelude::{Entity, Message};

#[derive(Debug,Message,Clone)]
pub struct UixUserMessage {
    pub entity : Entity,
    pub event : script_lang::StringT,
    pub params : Vec<script_lang::Value>,
}

