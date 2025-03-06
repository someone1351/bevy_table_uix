

use std::{collections::HashSet, path::PathBuf};

use bevy::{asset::{io::Reader, AssetLoader, Handle, LoadContext}, prelude::Asset, reflect::TypePath};
use conf_lang::Conf;

use bevy_table_ui as table_ui;
use table_ui::{UiAffectState, UiTextHAlign, UiTextVAlign, UiVal};

// use crate::libs::conf_lang;
// use crate::{input::Mapping, libs::input_map};
// use crate::input;

pub fn parse_scale(x:&str) -> Option<f32> {
    // let x = x.trim();
    if x.ends_with("%") {
        x[0..x.len()-1].parse::<f32>().ok().map(|x|x*0.01)
    } else if x.contains(".") {
        x.parse::<f32>().ok()
    } else {
        None

    }
}

pub fn parse_uival(s:&str) -> Option<UiVal> {
    let s = s.trim();

    if s.is_empty() || s.eq("none") {
        Some(UiVal::None)
    } else if s.ends_with("%") {
        s[0..s.len()-1].parse::<f32>().ok().map(|x|UiVal::Scale(x*0.01))
    } else if let Ok(x)=s.parse::<i32>() {
        Some(UiVal::Px(x as f32))
    } else if let Ok(x)=s.parse::<f32>() {
        Some(UiVal::Scale(x))
    } else {
        None
    }
}
pub fn parse_uival_bool(s:&str) -> Option<UiVal> {
    let s = s.trim();

    s.parse::<bool>().ok()
        .map(|x|UiVal::Scale(x.then_some(1.0).unwrap_or_default()))
        .or_else(||parse_uival(s))
}

pub fn parse_halign(s:&str) -> Option<UiVal> {
    let s = s.trim();

    parse_uival(s).or_else(||match s {
        "left" => Some(UiVal::Scale(0.0)),
        "center" => Some(UiVal::Scale(0.5)),
        "right" => Some(UiVal::Scale(0.0)),
        _ => None,
    })
}

pub fn parse_valign(s:&str) -> Option<UiVal> {
    let s = s.trim();

    parse_uival(s).or_else(||match s {
        "top" => Some(UiVal::Scale(0.0)),
        "center" => Some(UiVal::Scale(0.5)),
        "bottom" => Some(UiVal::Scale(0.0)),
        _ => None,
    })
}

pub fn parse_affect_state(s:&str) -> Option<UiAffectState> {
    match s {
        "select" => Some(UiAffectState::Select),
        "hover" => Some(UiAffectState::Hover),
        "focus" => Some(UiAffectState::Focus),
        "drag" => Some(UiAffectState::Drag),
        "press" => Some(UiAffectState::Press),
        _ => None
    }
}

pub fn parse_identity(s:&str) -> Option<()> {
    for (i,c) in s.chars().enumerate() {
        match c {
            'a'..='z'|'A'..='Z'|'_' => {}
            '0'..='9' if i!=0 => {}
            _ => {
                return None;
            }
        }
    }

    Some(())
}

#[derive(Asset, Debug, TypePath,)]
pub struct UiAsset {
    // pub src : String,
    pub conf : conf_lang::Conf,
    pub dependencies :Vec<Handle<UiAsset>>,
}

#[derive(Debug, thiserror::Error)]
pub enum UiAssetLoaderError {
    // #[error("Could load input bindings: {0}")]
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Could not parse input bindings: {0}")]
    Parse(#[from] conf_lang::ParseError),

    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),

    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

#[derive(Default)]
pub struct UiAssetLoader;

impl AssetLoader for UiAssetLoader {
    type Asset = UiAsset;
    type Settings = ();
    type Error = UiAssetLoaderError;

    // async fn load<'a>(
    //     &'a self,
    //     reader: &'a mut Reader<'_>,
    //     _settings: &'a Self::Settings,
    //     load_context: &'a mut LoadContext<'_>,
    // )
    //  -> Result<Self::Asset, Self::Error>

     async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error>
     {

        //
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        //
        let src = std::str::from_utf8(&bytes).unwrap();
        let path=load_context.path();

        //
        let def = input_def();

        //
        match def.get_root_branch().parse(src,true,Some(path)) {
            Ok(conf) => {
                let mut custom_asset = UiAsset {conf,dependencies:Default::default()}; //
                let includes=walk_includes(&custom_asset.conf);

                for include in includes {
                    let x:Handle<UiAsset>=load_context.load(include);
                    custom_asset.dependencies.push(x);

                }
                // load_context.ass
                // load_context.set_default_asset(LoadedAsset::new(custom_asset));

                // let x=load_context.labeled_asset_scope("label".to_string(), |ctx|custom_asset.clone());
                // let loaded = load_context
                //     .loader()
                //     .direct()
                //     .load::<UiAsset>("&embedded")
                //     .await?
                //     ;
        // load_context.
                Ok(custom_asset)
            },
            Err(e) => {
                // eprintln!("{}",e.msg(src));
                Err(UiAssetLoaderError::Parse(e))
            }
        }

    }

    fn extensions(&self) -> &[&str] { &["ui_conf"] }
}

fn input_def() -> conf_lang::Def {
    conf_lang::Def::new()
        .branch("root_branch").include(["apply_branch","nodes_branch"])
            .tags(["template"])
                .entry_children("node_branch").label_entry("template_decl")
                    .param_any()
            .tags(["include"])
                .entry()
                    .param_any()
            .tags(["script"])
                .entry_text().label_entry("is_root")

        .branch("apply_branch")
            .tags(["apply"])
                .entry_children("node_branch")
                    .repeat_group()
                    .param_any()

        .branch("nodes_branch").include(["nodes_branch2"])
            .tags(["stub"])
                .entry_children("nodes_branch2")
                    .param_func(parse_identity)
        .branch("nodes_branch2")
            .tags(["node"])
                .entry_children("node_branch")
                    .repeat_group()
                    .param_optional()
                    .param_any()
        .branch("node_branch").include(["nodes_branch","apply_branch","attribs_branch"])  //.include(["node2_branch"])
            .tags(["template"])
                .entry().label_entry("template_use")
                    .repeat_group()
                    .param_any()
            .tags(["script"])
                .entry_text().label_entry("not_root")
            // .tags(["mut"])
            //     .entry_children(None, "attribs_branch")

        .branch("attribs_branch").include(["attribs2_branch"])
            .tags(["on"])
                .entry_children("attribs2_branch")
                    .param_func(parse_affect_state)
                    .param_optional()
                    .param_parse::<i32>()
            .tags([
                "hoverable","pressable","draggable","selectable","focusable",
                "press_always","press_physical",// "press_released",
                "disabled","hidden","floating","locked","selected",
                "focused",
                "focus_tab_exit","focus_hdir_exit","focus_vdir_exit",
                "focus_hdir_wrap","focus_vdir_wrap",
                "focus_hdir_press","focus_vdir_press",
            ])
                .entry()
                    .param_optional()
                    .param_parse::<bool>()
            .tags(["select_group"])
                .entry()
                    .param_any()
            .tags(["focus_group"])
                .entry()
                    .param_parse::<i32>()
            .tags([
                "width","height","hgap","vgap",
                //"hscroll","vscroll",
                "border_left","border_right","border_top","border_bottom",
                "padding_left","padding_right","padding_top","padding_bottom",
                "margin_left","margin_right","margin_top","margin_bottom",
            ])
                .entry()
                    .param_func(parse_uival)
            .tags(["hexpand","vexpand","hfill","vfill",])
                .entry()
                    .param_func(parse_uival_bool)
            .tags(["row_width_scale","col_height_scale"])
                .entry()
                    .param_func(parse_scale)
            .tags(["halign"])
                .entry()
                    .param_func(parse_halign)
            .tags(["valign"])
                .entry()
                    .param_func(parse_valign)
            .tags(["text_halign"])
                .entry()
                    .param_parse::<UiTextHAlign>()
            .tags(["text_valign"])
                .entry()
                    .param_parse::<UiTextVAlign>()
            .tags(["text","font","image"])
                .entry()
                    .param_any()
            .tags(["text_size"])
                .entry()
                    .param_parse::<f32>()
            .tags(["text_hlen","text_vlen"])
                .entry()
                    .param_parse::<u32>()
            .tags(["span"])
                .entry()
                    .param_parse::<u32>()
            .tags(["size"])
                .entry()
                    .param_func(parse_uival)
                    .param_func(parse_uival)
            .tags(["border","padding","margin","gap"])
                .entry()
                    .param_func(parse_uival)
                    .param_optional()
                    .param_func(parse_uival)

        .branch("attribs2_branch")
            .tags([
                "color","border_color","padding_color",
                "margin_color","cell_color","text_color",
            ])
                .entry().label_entry("colori")
                    .param_parse::<u8>()
                    .param_parse::<u8>()
                    .param_parse::<u8>()
                    .param_optional()
                    .param_parse::<u8>()
                .entry().label_entry("colorf")
                    .param_parse::<f32>()
                    .param_parse::<f32>()
                    .param_parse::<f32>()
                    .param_optional()
                    .param_parse::<f32>()

}

fn walk_includes(conf:&Conf) -> HashSet<PathBuf> {
    let mut includes=HashSet::<PathBuf>::new();

    conf.root().walk(|walk|{
        if walk.record().tag() == Some("include") && walk.is_enter() {
            includes.insert(PathBuf::from(walk.record().first().str()));
        }

    }).unwrap();

    includes
}