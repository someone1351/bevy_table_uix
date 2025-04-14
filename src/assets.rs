

use std::{collections::HashSet, path::PathBuf};

use bevy::{asset::{io::Reader, AssetLoader, Handle, LoadContext}, color::Color, prelude::Asset, reflect::TypePath};
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

#[derive(Clone)]
pub struct HexColor(pub Color);

pub fn parse_col_hex(s:&str) -> Option<HexColor> {
    let mut cs=s.chars();

    let Some('#')=cs.next() else {
        return None;
    };

    let len=cs.clone().count();

    if len!=6 && len!=8 {
        return None;
    }

    let mut col: [u8; 4] = [0,0,0,255];

    for i in 0 .. len/2 {
        let mut x= String::new();
        x.push(cs.next().unwrap());
        x.push(cs.next().unwrap());

        let Ok(y)=u8::from_str_radix(x.as_str(),16)
            // .map(|x|x.to_be_bytes())
            else {
            return None;
        };

        col[i]=y;
    }

    Some(HexColor(Color::srgba_u8(col[0],col[1],col[2],col[3])))
}

#[derive(Clone)]
pub struct NamedColor(pub Color);

pub fn parse_col_name(s:&str) -> Option<NamedColor> {
    match s.replace("_", " ").to_lowercase().as_str() {
        "maroon" => Some(Color::srgb_u8(128,0,0)),
        "dark red" => Some(Color::srgb_u8(139,0,0)),
        "brown" => Some(Color::srgb_u8(165,42,42)),
        "firebrick" => Some(Color::srgb_u8(178,34,34)),
        "crimson" => Some(Color::srgb_u8(220,20,60)),
        "red" => Some(Color::srgb_u8(255,0,0)),
        "tomato" => Some(Color::srgb_u8(255,99,71)),
        "coral" => Some(Color::srgb_u8(255,127,80)),
        "indian red" => Some(Color::srgb_u8(205,92,92)),
        "light coral" => Some(Color::srgb_u8(240,128,128)),
        "dark salmon" => Some(Color::srgb_u8(233,150,122)),
        "salmon" => Some(Color::srgb_u8(250,128,114)),
        "light salmon" => Some(Color::srgb_u8(255,160,122)),
        "orange red" => Some(Color::srgb_u8(255,69,0)),
        "dark orange" => Some(Color::srgb_u8(255,140,0)),
        "orange" => Some(Color::srgb_u8(255,165,0)),
        "gold" => Some(Color::srgb_u8(255,215,0)),
        "dark golden rod" => Some(Color::srgb_u8(184,134,11)),
        "golden rod" => Some(Color::srgb_u8(218,165,32)),
        "pale golden rod" => Some(Color::srgb_u8(238,232,170)),
        "dark khaki" => Some(Color::srgb_u8(189,183,107)),
        "khaki" => Some(Color::srgb_u8(240,230,140)),
        "olive" => Some(Color::srgb_u8(128,128,0)),
        "yellow" => Some(Color::srgb_u8(255,255,0)),
        "yellow green" => Some(Color::srgb_u8(154,205,50)),
        "dark olive green" => Some(Color::srgb_u8(85,107,47)),
        "olive drab" => Some(Color::srgb_u8(107,142,35)),
        "lawn green" => Some(Color::srgb_u8(124,252,0)),
        "chartreuse" => Some(Color::srgb_u8(127,255,0)),
        "green yellow" => Some(Color::srgb_u8(173,255,47)),
        "dark green" => Some(Color::srgb_u8(0,100,0)),
        "green" => Some(Color::srgb_u8(0,128,0)),
        "forest green" => Some(Color::srgb_u8(34,139,34)),
        "lime" => Some(Color::srgb_u8(0,255,0)),
        "lime green" => Some(Color::srgb_u8(50,205,50)),
        "light green" => Some(Color::srgb_u8(144,238,144)),
        "pale green" => Some(Color::srgb_u8(152,251,152)),
        "dark sea green" => Some(Color::srgb_u8(143,188,143)),
        "medium spring green" => Some(Color::srgb_u8(0,250,154)),
        "spring green" => Some(Color::srgb_u8(0,255,127)),
        "sea green" => Some(Color::srgb_u8(46,139,87)),
        "medium aqua marine" => Some(Color::srgb_u8(102,205,170)),
        "medium sea green" => Some(Color::srgb_u8(60,179,113)),
        "light sea green" => Some(Color::srgb_u8(32,178,170)),
        "dark slate gray" => Some(Color::srgb_u8(47,79,79)),
        "teal" => Some(Color::srgb_u8(0,128,128)),
        "dark cyan" => Some(Color::srgb_u8(0,139,139)),
        "aqua" => Some(Color::srgb_u8(0,255,255)),
        "cyan" => Some(Color::srgb_u8(0,255,255)),
        "light cyan" => Some(Color::srgb_u8(224,255,255)),
        "dark turquoise" => Some(Color::srgb_u8(0,206,209)),
        "turquoise" => Some(Color::srgb_u8(64,224,208)),
        "medium turquoise" => Some(Color::srgb_u8(72,209,204)),
        "pale turquoise" => Some(Color::srgb_u8(175,238,238)),
        "aqua marine" => Some(Color::srgb_u8(127,255,212)),
        "powder blue" => Some(Color::srgb_u8(176,224,230)),
        "cadet blue" => Some(Color::srgb_u8(95,158,160)),
        "steel blue" => Some(Color::srgb_u8(70,130,180)),
        "corn flower blue" => Some(Color::srgb_u8(100,149,237)),
        "deep sky blue" => Some(Color::srgb_u8(0,191,255)),
        "dodger blue" => Some(Color::srgb_u8(30,144,255)),
        "light blue" => Some(Color::srgb_u8(173,216,230)),
        "sky blue" => Some(Color::srgb_u8(135,206,235)),
        "light sky blue" => Some(Color::srgb_u8(135,206,250)),
        "midnight blue" => Some(Color::srgb_u8(25,25,112)),
        "navy" => Some(Color::srgb_u8(0,0,128)),
        "dark blue" => Some(Color::srgb_u8(0,0,139)),
        "medium blue" => Some(Color::srgb_u8(0,0,205)),
        "blue" => Some(Color::srgb_u8(0,0,255)),
        "royal blue" => Some(Color::srgb_u8(65,105,225)),
        "blue violet" => Some(Color::srgb_u8(138,43,226)),
        "indigo" => Some(Color::srgb_u8(75,0,130)),
        "dark slate blue" => Some(Color::srgb_u8(72,61,139)),
        "slate blue" => Some(Color::srgb_u8(106,90,205)),
        "medium slate blue" => Some(Color::srgb_u8(123,104,238)),
        "medium purple" => Some(Color::srgb_u8(147,112,219)),
        "dark magenta" => Some(Color::srgb_u8(139,0,139)),
        "dark violet" => Some(Color::srgb_u8(148,0,211)),
        "dark orchid" => Some(Color::srgb_u8(153,50,204)),
        "medium orchid" => Some(Color::srgb_u8(186,85,211)),
        "purple" => Some(Color::srgb_u8(128,0,128)),
        "thistle" => Some(Color::srgb_u8(216,191,216)),
        "plum" => Some(Color::srgb_u8(221,160,221)),
        "violet" => Some(Color::srgb_u8(238,130,238)),
        "magenta" => Some(Color::srgb_u8(255,0,255)),
        "fuchsia" => Some(Color::srgb_u8(255,0,255)),
        "orchid" => Some(Color::srgb_u8(218,112,214)),
        "medium violet red" => Some(Color::srgb_u8(199,21,133)),
        "pale violet red" => Some(Color::srgb_u8(219,112,147)),
        "deep pink" => Some(Color::srgb_u8(255,20,147)),
        "hot pink" => Some(Color::srgb_u8(255,105,180)),
        "light pink" => Some(Color::srgb_u8(255,182,193)),
        "pink" => Some(Color::srgb_u8(255,192,203)),
        "antique white" => Some(Color::srgb_u8(250,235,215)),
        "beige" => Some(Color::srgb_u8(245,245,220)),
        "bisque" => Some(Color::srgb_u8(255,228,196)),
        "blanched almond" => Some(Color::srgb_u8(255,235,205)),
        "wheat" => Some(Color::srgb_u8(245,222,179)),
        "corn silk" => Some(Color::srgb_u8(255,248,220)),
        "lemon chiffon" => Some(Color::srgb_u8(255,250,205)),
        "light golden rod yellow" => Some(Color::srgb_u8(250,250,210)),
        "light yellow" => Some(Color::srgb_u8(255,255,224)),
        "saddle brown" => Some(Color::srgb_u8(139,69,19)),
        "sienna" => Some(Color::srgb_u8(160,82,45)),
        "chocolate" => Some(Color::srgb_u8(210,105,30)),
        "peru" => Some(Color::srgb_u8(205,133,63)),
        "sandy brown" => Some(Color::srgb_u8(244,164,96)),
        "burly wood" => Some(Color::srgb_u8(222,184,135)),
        "tan" => Some(Color::srgb_u8(210,180,140)),
        "rosy brown" => Some(Color::srgb_u8(188,143,143)),
        "moccasin" => Some(Color::srgb_u8(255,228,181)),
        "navajo white" => Some(Color::srgb_u8(255,222,173)),
        "peach puff" => Some(Color::srgb_u8(255,218,185)),
        "misty rose" => Some(Color::srgb_u8(255,228,225)),
        "lavender blush" => Some(Color::srgb_u8(255,240,245)),
        "linen" => Some(Color::srgb_u8(250,240,230)),
        "old lace" => Some(Color::srgb_u8(253,245,230)),
        "papaya whip" => Some(Color::srgb_u8(255,239,213)),
        "sea shell" => Some(Color::srgb_u8(255,245,238)),
        "mint cream" => Some(Color::srgb_u8(245,255,250)),
        "slate gray" => Some(Color::srgb_u8(112,128,144)),
        "light slate gray" => Some(Color::srgb_u8(119,136,153)),
        "light steel blue" => Some(Color::srgb_u8(176,196,222)),
        "lavender" => Some(Color::srgb_u8(230,230,250)),
        "floral white" => Some(Color::srgb_u8(255,250,240)),
        "alice blue" => Some(Color::srgb_u8(240,248,255)),
        "ghost white" => Some(Color::srgb_u8(248,248,255)),
        "honeydew" => Some(Color::srgb_u8(240,255,240)),
        "ivory" => Some(Color::srgb_u8(255,255,240)),
        "azure" => Some(Color::srgb_u8(240,255,255)),
        "snow" => Some(Color::srgb_u8(255,250,250)),
        "black" => Some(Color::srgb_u8(0,0,0)),
        "dim gray" => Some(Color::srgb_u8(105,105,105)),
        "dim grey" => Some(Color::srgb_u8(105,105,105)),
        "gray" => Some(Color::srgb_u8(128,128,128)),
        "grey" => Some(Color::srgb_u8(128,128,128)),
        "dark gray" => Some(Color::srgb_u8(169,169,169)),
        "dark grey" => Some(Color::srgb_u8(169,169,169)),
        "silver" => Some(Color::srgb_u8(192,192,192)),
        "light gray" => Some(Color::srgb_u8(211,211,211)),
        "light grey" => Some(Color::srgb_u8(211,211,211)),
        "gainsboro" => Some(Color::srgb_u8(220,220,220)),
        "white smoke" => Some(Color::srgb_u8(245,245,245)),
        "white" => Some(Color::srgb_u8(255,255,255)),
        "none" => Some(Color::NONE),
        _ => None,
    }.map(|x|NamedColor(x))
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

    #[error("Could not parse ui conf: {0}")]
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
        .branch("root_branch").include(["nodes_branch"]) //.include(["apply_branch","nodes_branch"])
            .tags(["template"])
                .entry_children("node_branch").elabel("template_decl")
                    .param_any()
            .tags(["include"])
                .entry()
                    .param_any()
            // .tags(["script"])
            //     .entry_text() //.elabel("is_root")

        // .branch("apply_branch")
        //     .tags(["apply"])
        //         .entry_children("node_branch")
        //             .grepeat()
        //             .param_any()

        .branch("nodes_branch") //.include(["nodes_branch2"])
            // .tags(["stub"])
            //     .entry_children("nodes_branch2")
            //         .param_func(parse_identity)
            .tags(["stub"])
                .entry_children("nodes_branch")
                    .param_func(parse_identity)
            .tags(["node"])
                .entry_children("node_branch")
                    .grepeat()
                    .param_optional()
                    .param_any()
            .tags(["script"])
                .entry_text() //.elabel("not_root")
            .tags(["apply"])
                .entry_children("node_branch")
                    .grepeat()
                    .param_any()
        // .branch("nodes_branch2")
        //     .tags(["node"])
        //         .entry_children("node_branch")
        //             .grepeat()
        //             // .goptional(Some(0))
        //             .param_optional()
        //             .param_any()
        //     .tags(["script"])
        //         .entry_text() //.elabel("not_root")
        .branch("node_branch").include(["nodes_branch","attribs_branch"]) //.include(["nodes_branch","apply_branch","attribs_branch"])  //.include(["node2_branch"])
            .tags(["template"])
                .entry().elabel("template_use")
                    .grepeat()
                    .param_any()
            // .tags(["script"])
            //     .entry_text() //.elabel("not_root")

        .branch("attribs_branch").include(["attribs2_branch"])
            .tags(["on"])
                .entry_children("attribs2_branch")
                    // .goptional(Some(1))
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
                    // .goptional(Some(0))
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
                "hpadding","vpadding","hborder","vborder","hmargin","vmargin",
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
                    // .goptional(Some(1))
                    .param_func(parse_uival)
                    .param_optional()
                    .param_func(parse_uival)
            .tags(["border","padding","margin","gap"])
                .entry()
                    // .goptional(Some(1))
                    .param_func(parse_uival)
                    .param_optional()
                    .param_func(parse_uival)

        .branch("attribs2_branch")
            .tags([
                "color","border_color","padding_color",
                "margin_color","cell_color","text_color",
            ])
                .entry().elabel("colori")
                    // .goptional(Some(3))
                    .param_parse::<u8>()
                    .param_parse::<u8>()
                    .param_parse::<u8>()
                    .param_optional()
                    .param_parse::<u8>()
                .entry().elabel("colorf")
                    // .goptional(Some(3))
                    .param_parse::<f32>()
                    .param_parse::<f32>()
                    .param_parse::<f32>()
                    .param_optional()
                    .param_parse::<f32>()
                .entry().elabel("colorh")
                    .param_func(parse_col_hex)
                .entry().elabel("colorn")
                    .param_func(parse_col_name)

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