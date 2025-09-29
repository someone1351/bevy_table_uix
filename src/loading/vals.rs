
// #![allow(unused_mut)]
// #![allow(unused_variables)]
#![allow(dead_code)]
use std::collections::BTreeSet;
// use std::collections::HashMap;
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
pub struct ElementApplyCall {
    pub parent_element_ind : usize,
    pub apply_use_element_ind : usize,
    pub func_froms : Option<(
        ScriptSyntaxNodeOrApplyUse, //node_element_ind or apply_use_element_ind
        Vec<ScriptSyntaxTemplateUse>, //template_use_element_inds
    )>,

}

#[derive(Debug,Clone)]
pub struct Element<'a> {
    pub element_type:ElementType<'a>,
    pub children : Vec<usize>,
    pub applies : Vec<usize>, //element_ind
    pub apply_after : usize, //parent_apply_ind
    pub calcd_from_element_ind : Option<usize>, //element_ind, mirrors nodes inside template_use/apply_use to their template_decl/apply_decl origin
    pub calcd_node_params:BTreeSet<usize>, //node element_ind
    pub calcd_env_params:BTreeSet<usize>, //element_ind
    pub calcd_created_from : usize, //same as calcd_original?? same as parent? no, something to do with applies, and their origin, which is sometimes parent, sometimes something else?
    // pub calcd_original : Option<usize>, //source element (Node/TemplateUse/Attrib) used to create this one, from an apply use
    pub has_self_script:bool, //or has_node_script? no it also includes apply and template own, ie has_decl_init_script
    // pub has_template_use_script:bool,
    pub has_script:bool,
    pub has_env_script:bool,
    // pub has_apply_decl_script:bool,
    // pub env : HashMap<String,Vec<usize>>, //env[name]=element_inds

    pub parent:Option<usize>,

    pub rets : Vec<(
        Option<ScriptSyntaxNode>, //node_element_ind
        ScriptSyntaxTemplateUseOrApplyDecl, //template_use_element_ind or apply_decl_element_ind
    )>, //for template decl, apply decl, node

    pub apply_calls : Vec<ElementApplyCall>,
    // apply_froms : Option<(
    //     ScriptSyntaxNodeOrApplyUse, //node_element_ind or apply_use_element_ind
    //     Vec<ScriptSyntaxTemplateUse>, //template_use_element_inds
    // )>, //for apply_use, only in nodes, and not descendant of apply_decl or template_decl
}


//////

#[derive(Debug,Clone)]
pub enum ScriptSyntaxTemplateUseOrApplyDecl {
    ApplyDecl(usize),
    TemplateUse(usize),
}



#[derive(Debug,Clone)]
pub enum ScriptSyntaxNodeOrApplyUse {
    Node(usize),
    ApplyUse(usize),
}
// pub enum ScriptSyntaxDecl {
//     Node,
//     Apply,
//     Template,
// }

#[derive(Debug,Copy,Clone)]
pub enum ScriptSyntaxNodeOrApplyOrTemplateDecl {
    Node(usize),
    ApplyDecl(usize),
    TemplateDecl(usize),
}
impl ScriptSyntaxNodeOrApplyOrTemplateDecl {
    pub fn element_ind(&self) -> usize {
        match *self {
            Self::Node(x) => x,
            Self::ApplyDecl(x) => x,
            Self::TemplateDecl(x) => x,
        }
    }
}
// #[derive(Debug,Copy,Clone)]
// pub enum usize {
//     Node(usize),
//     ApplyUse(usize),
//     TemplateUse(usize),
// }
// impl usize {
//     pub fn element_ind(&self) -> usize {
//         match *self {
//             Self::Node(x) => x,
//             Self::ApplyUse(x) => x,
//             Self::TemplateUse(x) => x,
//         }
//     }
// }

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct ScriptSyntaxNode(pub usize);
#[derive(Copy,Clone)]
pub struct ScriptSyntaxTemplateUse(pub usize);
#[derive(Copy,Clone)]
pub struct ScriptSyntaxTemplateDecl(pub usize);
#[derive(Copy,Clone)]
pub struct ScriptSyntaxApplyDecl(pub usize);
#[derive(Copy,Clone)]
pub struct ScriptSyntaxApplyUse(pub usize);


pub enum ScriptSyntax {
    Root {
        children:Vec<usize>,
        // has_script:bool,
        has_env:bool,
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



    Decl { //name is element_ind
        // decl : ScriptSyntaxDecl,
        name : ScriptSyntaxNodeOrApplyOrTemplateDecl, //element_ind
        params : Vec<ScriptSyntaxNode>, //node element_inds, doesn't include self
        envs : Vec<usize>, //excludes self, node/apply_use/template_use element_ind
        children:Vec<usize>, //syntax_inds
        returns : Vec<(
            Option<ScriptSyntaxNode>, //node_element_ind
            ScriptSyntaxTemplateUseOrApplyDecl, //template_use_element_ind or apply_decl_element_ind
        )>,
        has_self:bool, //self param
        has_ret:bool,
        has_env:bool,
    },

    Stub { //needs stub_element_ind? no
        name : String,
        children:Vec<usize>, //syntax_inds
        // has_script:bool,
        element_ind:usize,
        has_env:bool,
    },

    // CallStubCreate {
    //     is_root:bool,
    //     // has_script:bool,
    //     stub : usize,//element_ind

    // },
    CallTemplate {

        ret : ScriptSyntaxTemplateUse, //template_use_element_ind
        func : ScriptSyntaxTemplateDecl, //template_decl_element_ind
        params : Vec<ScriptSyntaxNode>, //node_element_inds, doesn't include self, why not? doesn't need it since "self" is
        envs : Vec<usize>, //includes self env, node/apply_use/template_use element_ind
        // use_self : Option<usize>, // element_ind of self
        has_self : bool,
        has_ret:bool,
        // has_env:bool,
    },
    CallApply {
        ret : ScriptSyntaxApplyUse, //apply_use_element_ind
        func_froms : Option<(
            ScriptSyntaxNodeOrApplyUse, //node_element_ind or apply_use_element_ind
            Vec<ScriptSyntaxTemplateUse>, //template_use_element_inds
        )>,
        func_apply : ScriptSyntaxApplyDecl, //apply_decl_element_ind
        params : Vec<ScriptSyntaxNode>, //node_element_inds, includes self
        envs : Vec<usize>, //includes self, node/apply_use/template_use element_ind
        // not_has_self : Option<ScriptSyntaxNode>, //element_ind of self
        self_node:ScriptSyntaxNode, //
        has_self:bool, //not needed, can check if param[0]==self_node
        has_ret:bool,
    },
    CallNode {
        has_ret:bool,
        in_func:bool, //inside template_decl, apply_decl or node
        func : ScriptSyntaxNode, //node_element_ind
        params : Vec<ScriptSyntaxNode>, //node_element_inds, includes self
        envs : Vec<usize>, //includes self, node/apply_use/template_use element_ind
        // self_param:bool, //doesn't need, params will have it (or not)
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
    pub fn element_ind(&self) -> Option<usize> {
        match self {
            ScriptSyntax::Root { .. } => None,
            ScriptSyntax::Insert { .. } => None,
            ScriptSyntax::Decl { name, .. } => Some(name.element_ind()),
            ScriptSyntax::Stub { .. } => None,
            // ScriptSyntax::CallStubCreate { .. } => None,
            ScriptSyntax::CallTemplate { ret, .. } => Some(ret.0),
            ScriptSyntax::CallApply { ret, .. } => Some(ret.0),
            ScriptSyntax::CallNode {  func, .. } => Some(func.0),
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



impl Display for ScriptSyntaxNodeOrApplyOrTemplateDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.element_ind())
    }
}

// impl Display for usize {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f,"{}",self.element_ind())
//     }
// }
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