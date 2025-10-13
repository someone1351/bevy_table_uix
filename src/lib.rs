/*
TODO
* could provide variadic params to stub func eg fn stub_thing(self params ...){}, so when calling eg call stub_thing self 1 2 3

*/

mod assets;
mod components;
mod plugin;
mod systems;
mod resources;
mod script_libs;
mod messages;
// mod loading_utils;
// mod loading_vals;
mod loading;
mod script_vals;

mod script_utils;

pub use plugin::*;
pub use assets::*;
pub use components::*;
pub use messages::*;