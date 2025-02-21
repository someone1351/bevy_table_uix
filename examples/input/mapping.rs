use serde::Deserialize;
// use std::str::FromStr;

#[derive(Clone,Debug,Deserialize,Hash,PartialEq,Eq,Ord,PartialOrd)]

pub enum GlobalMapping {
    Exit,
    Test(String),
    ScreenShot
}
#[derive(Default,Clone,Copy,Debug,serde::Deserialize,Hash,PartialEq,Eq,Ord,PartialOrd)]
pub enum UiMapping {
    #[default]
    None,

    CursorOk,
    CursorCancel,
    ScrollHori,
    ScrollVert,
    FocusUp,
    FocusRight,
    FocusNext(i32),
    FocusOk,
    FocusCancel,

    Test,
}
#[derive(Clone,Copy,Debug,Deserialize,Hash,PartialEq,Eq,Ord,PartialOrd)]
pub enum GameMapping {
    Forward,
    Right,
    Pitch,
    Yaw,
    MousePitch,
    MouseYaw,
    SelectColorUp,
    SelectColorRight,
    Fire,
    AltFire,
    Reload,
    Jump,
    Crouch,
    Sprint,
    SelectWeapon(usize),
    SelectWeaponNext,
    Menu,
}

#[derive(Clone,Debug,Deserialize,Hash,PartialEq,Eq,Ord,PartialOrd,Default)]
pub enum Mapping {
    #[default]
    None,

    Global(GlobalMapping),
    Game(GameMapping),
    Ui(UiMapping),
}

impl ToString for Mapping {
    fn to_string(&self) -> String { format!("{:?}",self) }
}

impl std::str::FromStr for Mapping {
    type Err = ron::de::SpannedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { ron::de::from_str::<Self>(s) }
}
