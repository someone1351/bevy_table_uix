

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
            .tag_nodes(["template"])
                .entry_children(Some("template_decl"),"node_branch")
                    .param_any()
            .tag_nodes(["include"])
                .entry(None)
                    .param_any()
            .tag_nodes(["script"])
                .entry_text(Some("is_root"))

        .branch("apply_branch")
            .tag_nodes(["apply"])
                .entry_children(None, "node_branch")
                    .group_repeat()
                    .param_any()

        .branch("nodes_branch").include(["nodes_branch2"])
            .tag_nodes(["stub"])
                .entry_children(None, "nodes_branch2")
                    .param_func(parse_identity)
        .branch("nodes_branch2")
            .tag_nodes(["node"])
                .entry_children(None, "node_branch")
                    .group_repeat()
                    .param_optional()
                    .param_any()
        .branch("node_branch").include(["nodes_branch","apply_branch","attribs_branch"])  //.include(["node2_branch"])
            .tag_nodes(["template"])
                .entry(Some("template_use"))
                    .group_repeat()
                    .param_any()
            .tag_nodes(["script"])
                .entry_text(Some("not_root"))
            // .tag_nodes(["mut"])
            //     .entry_children(None, "attribs_branch")

        .branch("attribs_branch").include(["attribs2_branch"])
            .tag_nodes(["on"])
                .entry_children(None, "attribs2_branch")
                    .param_func(parse_affect_state)
                    .param_optional()
                    .param_parse::<i32>()
            .tag_nodes([
                "hoverable","pressable","draggable","selectable","focusable",
                "press_always","press_physical",// "press_released",
                "disabled","hidden","floating","locked","selected",
                "focused",
                "focus_tab_exit","focus_hdir_exit","focus_vdir_exit",
                "focus_hdir_wrap","focus_vdir_wrap",
                "focus_hdir_press","focus_vdir_press",
            ])
                .entry(None)
                    .param_optional()
                    .param_parse::<bool>()
            .tag_nodes(["select_group"])
                .entry(None)
                    .param_any()
            .tag_nodes(["focus_group"])
                .entry(None)
                    .param_parse::<i32>()
            .tag_nodes([
                "width","height","hgap","vgap",
                //"hscroll","vscroll",
                "border_left","border_right","border_top","border_bottom",
                "padding_left","padding_right","padding_top","padding_bottom",
                "margin_left","margin_right","margin_top","margin_bottom",
            ])
                .entry(None)
                    .param_func(parse_uival)
            .tag_nodes(["hexpand","vexpand","hfill","vfill",])
                .entry(None)
                    .param_func(parse_uival_bool)
            .tag_nodes(["row_width_scale","col_height_scale"])
                .entry(None)
                    .param_func(parse_scale)
            .tag_nodes(["halign"])
                .entry(None)
                    .param_func(parse_halign)
            .tag_nodes(["valign"])
                .entry(None)
                    .param_func(parse_valign)
            .tag_nodes(["text_halign"])
                .entry(None)
                    .param_parse::<UiTextHAlign>()
            .tag_nodes(["text_valign"])
                .entry(None)
                    .param_parse::<UiTextVAlign>()
            .tag_nodes(["text","font","image"])
                .entry(None)
                    .param_any()
            .tag_nodes(["text_size"])
                .entry(None)
                    .param_parse::<f32>()
            .tag_nodes(["text_hlen","text_vlen"])
                .entry(None)
                    .param_parse::<u32>()
            .tag_nodes(["span"])
                .entry(None)
                    .param_parse::<u32>()
            .tag_nodes(["size"])
                .entry(None)
                    .param_func(parse_uival)
                    .param_func(parse_uival)
            .tag_nodes(["border","padding","margin","gap"])
                .entry(None)
                    .param_func(parse_uival)
                    .param_optional()
                    .param_func(parse_uival)

        .branch("attribs2_branch")
            .tag_nodes([
                "color","border_color","padding_color",
                "margin_color","cell_color","text_color",
            ])
                .entry(Some("colori"))
                    .param_parse::<u8>()
                    .param_parse::<u8>()
                    .param_parse::<u8>()
                    .param_optional()
                    .param_parse::<u8>()
                .entry(Some("colorf"))
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