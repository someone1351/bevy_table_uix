

use bevy::{asset::{AssetLoader, LoadContext, io::Reader}, reflect::TypePath, prelude::Asset};

// use crate::libs::conf_lang;
// use crate::{Mapping, libs::axis_input};
// use crate::input;

use super::mapping::*;
use bevy_axis_input as axis_input;


#[derive(Asset, Debug, TypePath)]
pub struct InputAsset {
    // pub src : String,
    pub conf : conf_lang::Conf,
}

#[derive(Debug, thiserror::Error)]
pub enum InputAssetLoaderError {
    #[error("Could load input bindings: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse input bindings: {0}")]
    Parse(#[from] conf_lang::ParseError),
}

#[derive(Default)]
pub struct InputAssetLoader;

impl AssetLoader for InputAssetLoader {
    type Asset = InputAsset;
    type Settings = ();
    type Error = InputAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,)
     -> Result<Self::Asset, Self::Error>
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
                let custom_asset = InputAsset {conf,};
                // load_context.set_default_asset(LoadedAsset::new(custom_asset));
                Ok(custom_asset)
            },
            Err(e) => {
                // eprintln!("{}",e.msg(src));
                Err(InputAssetLoaderError::Parse(e))
            }
        }

    }

    fn extensions(&self) -> &[&str] { &["input_conf"] }
}

fn input_def() -> conf_lang::Def {
    conf_lang::Def::new()
        .branch("root_branch")
            .tag_nodes(["repeat"])
                .entry_children(None,"repeat_branch")
            .tag_nodes(["excludes"])
                .entry_children(None,"exclude_branch")
            .tag_nodes(["scale"])
                .entry_children(None,"scale_branch")
                    .param_any()
            .tag_nodes(["invert"])
                .entry_children(None,"invert_branch")
                    .param_any()
            .tag_nodes(["bindings"])
                .entry_children(None,"bindings_branch")
                    .param_any()

        .branch("repeat_branch")
            .tagless_nodes()
                .entry(Some("repeat"))
                    .param_parse::<Mapping>()
                    .param_parse::<f32>()
        .branch("exclude_branch")
            .tagless_nodes()
                .entry(Some("exclude"))
                    .param_parse::<axis_input::Binding>()
        .branch("scale_branch")
            .tagless_nodes()
                .entry(Some("scale"))
                    .param_parse::<Mapping>()
                    .param_parse::<f32>()
        .branch("invert_branch")
            .tagless_nodes()
                .entry(Some("invert"))
                    .param_parse::<Mapping>()
                    .param_parse::<bool>()

        .branch("bindings_branch")
            .tagless_nodes()
                .entry_children(None,"binding_branch")
                    .param_parse::<Mapping>()

        .branch("binding_branch")
            .tagless_nodes()
                .entry(Some("binding"))
                    .group(Some("bindings"),false,true)
                        .param_parse::<axis_input::Binding>()
                    .group(Some("scale"),true,false)
                        .param_parse::<f32>()
                    .group(Some("primary_dead"),true,false)
                        .param_parse::<f32>()
                    .group(Some("modifier_dead"),true,false)
                        .param_parse::<f32>()

}