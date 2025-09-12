
// #![allow(unused_mut)]
// #![allow(unused_variables)]
#![allow(dead_code)]
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
// use bevy::platform::collections
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]
use std::fmt::Debug;
use std::fmt::Display;
use std::path::PathBuf;

// use bevy::platform::collections::HashSet;
use conf_lang::RecordContainer;
use bevy_table_ui as table_ui;
// use ron::de;
use table_ui::*;
// use super::script_stuff::{AttribFunc, Stuff};
use super::super::script_vals::*;

// use super::assets::*;


//
#[derive(Debug,Clone,Default)]
pub struct ElementAttribCalc {
    pub in_template : Option<usize>, //template_use_id
    pub in_apply : Option<usize>, //apply_decl_id
    pub used:bool,
    pub ok:bool,
}

#[derive(Debug,Clone)]
pub enum ElementType<'a> {
    // Root,
    Node {
        names : HashSet<&'a str>,
        ignore_applies : HashSet<usize>, //apply_decl_id
    },
    Attrib {
        name : &'a str,
        on_state: Option<UiAffectState>,
        in_template: Option<usize>,
        func : AttribFunc,
        in_node : bool,
        calcd:ElementAttribCalc,
    },
    Script {
        record : RecordContainer<'a>,
    },
    Apply {
        name : &'a str, //text_ind
        owner_apply_decl_id : Option<usize>, //element_ind
        used:bool,
    },
    ApplyUse {
        apply_decl_element_ind:usize,
    },
    TemplateDecl {
        name : &'a str, //text_ind
        used:bool,
    },
    TemplateUse {
        template_decl_element_ind:usize,
    },
    Stub {
        name : &'a str,
    },
    // From {
    //     element_ind:usize,
    // }
    // CalcApplyUse {
    //     apply_element_ind:usize,
    // }
    //CalcNode? what to do with its applies?
}

// #[derive(Debug,Clone,Hash,PartialEq, Eq)]
// pub enum EnvIndex {
//     Index(usize),
//     Name(String),
// }

#[derive(Debug,Clone)]
pub struct Element<'a> {
    pub element_type:ElementType<'a>,
    pub children : Vec<usize>,
    pub applies : Vec<usize>, //element_ind
    pub apply_after : usize, //parent_apply_ind
    pub calcd_from_element_ind : Option<usize>, //element_ind
    pub calcd_node_params:BTreeSet<usize>, //element_ind
    pub calcd_created_from : usize,
    pub calcd_original : Option<usize>, //source element (Node/TemplateUse/Attrib) used to create this one, from an apply use
    pub has_own_script:bool,
    pub has_template_use_script:bool,
    pub has_script:bool,
    pub has_apply_decl_script:bool,
    pub env : HashMap<String,Vec<usize>>, //env[name]=element_inds
}


//////

#[derive(Debug)]
pub enum ScriptSyntaxTemplateUseOrApplyDecl {
    ApplyDecl(usize),
    TemplateUse(usize),
}



#[derive(Debug)]
pub enum ScriptSyntaxNodeOrApplyUse {
    Node(usize),
    ApplyUse(usize),
}
// pub enum ScriptSyntaxDecl {
//     Node,
//     Apply,
//     Template,
// }

#[derive(Debug)]
pub enum ScriptSyntaxNodeOrApplyOrTemplate {
    Node(usize),
    Apply(usize),
    Template(usize),
}

pub struct ScriptSyntaxNode(pub usize);
pub struct ScriptSyntaxTemplateUse(pub usize);
pub struct ScriptSyntaxTemplateDecl(pub usize);
pub struct ScriptSyntaxApplyDecl(pub usize);
pub struct ScriptSyntaxApplyUse(pub usize);


pub enum ScriptSyntax {
    Root {
        children:Vec<usize>,
        // has_script:bool,
    },
    // InitStub {
    //     name:String,
    //     children:Vec<usize>,
    // },
    // InitVar {name:String,},

    Insert {
        path:Option<PathBuf>,
        loc :conf_lang::Loc,
        insert : String,
    },



    Decl {
        // decl : ScriptSyntaxDecl,
        name : ScriptSyntaxNodeOrApplyOrTemplate, //element_ind
        params : Vec<ScriptSyntaxNode>, //node element_inds
        children:Vec<usize>, //syntax_inds
        returns : Vec<(
            Option<ScriptSyntaxNode>, //node_element_ind
            ScriptSyntaxTemplateUseOrApplyDecl, //template_use_element_ind or apply_decl_element_ind
        )>,
    },

    Stub {
        name : String,
        children:Vec<usize>, //syntax_inds
        // has_script:bool,
    },

    CallStub {
        is_root:bool,
        has_script:bool,
        stub : usize,//element_ind

    },
    CallTemplate {
        ret : Option<ScriptSyntaxTemplateUse>, //template_use_element_ind
        func : ScriptSyntaxTemplateDecl, //template_decl_element_ind
        params : Vec<ScriptSyntaxNode>, //node_element_inds
    },
    CallApply {
        ret : Option<ScriptSyntaxApplyUse>, //apply_use_element_ind
        func_froms : Option<(
            ScriptSyntaxNodeOrApplyUse, //node_element_ind or apply_use_element_ind
            Vec<ScriptSyntaxTemplateUse>, //template_use_element_inds
        )>,
        func_apply : ScriptSyntaxApplyDecl, //apply_decl_element_ind
        params : Vec<ScriptSyntaxNode>, //node_element_inds
    },
    CallNode {
        ret:bool,
        in_func:bool, //inside template_decl, apply_decl or node
        func : ScriptSyntaxNode, //node_element_ind
        params : Vec<ScriptSyntaxNode>, //node_element_inds
    },
}

impl ScriptSyntax {
    pub fn get_children(&self) -> Option<&Vec<usize>> {
        match self {
            ScriptSyntax::Root{children, ..}|ScriptSyntax::Decl{children,..}|ScriptSyntax::Stub{children,..}=>Some(children),
            _ =>None,
        }
    }
    pub fn get_children_mut(&mut self) -> Option<&mut Vec<usize>> {
        match self {
            ScriptSyntax::Root{children, ..}|ScriptSyntax::Decl{children,..}|ScriptSyntax::Stub{children,..}=>Some(children),
            _ =>None,
        }
    }
}


//////





impl Debug for ScriptSyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Node").field(&self.0).finish()
    }
}
impl Debug for ScriptSyntaxTemplateUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TemplateUse").field(&self.0).finish()
    }
}
impl Debug for ScriptSyntaxTemplateDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TemplateDecl").field(&self.0).finish()
    }
}
impl Debug for ScriptSyntaxApplyDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ApplyDecl").field(&self.0).finish()
    }
}
impl Debug for ScriptSyntaxApplyUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ApplyUse").field(&self.0).finish()
    }
}
impl Display for ScriptSyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl Display for ScriptSyntaxTemplateUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}
impl Display for ScriptSyntaxTemplateDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl Display for ScriptSyntaxApplyDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}
impl Display for ScriptSyntaxApplyUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}