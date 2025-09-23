#![allow(dead_code)]

pub mod vals;
pub mod load_elements;
pub mod calc_node_apply_ignores;
pub mod expand_elements;
pub mod calc_node_params;
pub mod calc_env_params;
pub mod mark_used;
pub mod mark_has_script;
pub mod calc_envs;
pub mod gen_script_syntax_tree;
pub mod gen_script_src;
pub mod gen_stubs;


/*
* allow stubs' nodes to be added to anyone eg stub_a some_node
* add env to store named nodes
    eg
    env.abc.0
    env.0 ??
*/

// #![allow(unused_mut)]
// #![allow(unused_variables)]



use vals::*;


pub use load_elements::*;
pub use calc_node_apply_ignores::*;
pub use expand_elements::*;
pub use calc_node_params::*;
pub use calc_env_params::*;
pub use mark_used::*;
pub use mark_has_script::*;
pub use calc_envs::*;
pub use gen_script_syntax_tree::*;
pub use gen_script_src::*;
pub use gen_stubs::*;






                        // if let Some(loc)=record.text_values().next().map(|x|x.start_loc())
                        // if record.has_text()
                        // {
                        //     if let Some(p)=record.path() {
                        //         src+=&format!("{indent}#pragma_source {p:?}\n");
                        //     }

                        //     let loc=record.text_values().next().unwrap().start_loc();
                        //     src+=&format!("{indent}#pragma_loc {} {} {}\n",loc.pos,loc.row,loc.col);
                        // }
                        // if record.has_text() {
                        //     src+=&format!("{indent}#pragma_clear_loc\n");

                        //     if record.path().is_some() {
                        //         src+=&format!("{indent}#pragma_clear_source\n");
                        //     }
                        // }

pub fn debug_print_elements1(elements:&Vec<Element>) {
    println!("\n");

    {
        let mut stk=vec![(0,0)];

        while let Some((cur_element_ind,depth))=stk.pop() {
            let cur_element=elements.get(cur_element_ind).unwrap();
            stk.extend(cur_element.children.iter().rev().map(|&x|(x,depth+1)));
            let depth="    ".repeat(depth);
            println!("{depth}{:?}, applies={:?}, after={}",
                &cur_element.element_type,
                &cur_element.applies,
                // cur_element.applies.iter().map(|&i|{
                //     // let ElementType::Apply { .. }=&elements.get(i).unwrap().element_type else{panic!("")};
                //     // *apply_decl_id
                //     i
                // }).collect::<Vec<_>>(),
                cur_element.apply_after,
            );
        }
    }
    println!("\n");

}


pub fn debug_print_elements2(elements:&Vec<Element>) {
    println!("=====");
    //DebugPrintElements
    struct Work { element_ind:usize, depth:usize, }

    let mut work_stk=vec![Work{ element_ind: 0, depth: 0 }];

    while let Some(cur_work)=work_stk.pop() {
        let cur_element_ind=cur_work.element_ind;
        let cur_element=elements.get(cur_element_ind).unwrap();
        work_stk.extend(cur_element.children.iter().rev().map(|&child|Work { element_ind: child, depth: cur_work.depth+1 }));

        let indent="    ".repeat(cur_work.depth);
        let created_from=&cur_element.calcd_created_from;
        let calcd_from=&cur_element.calcd_from_element_ind;
        let params = &cur_element.calcd_node_params;
        let has_script=cur_element.has_script;
        // let has_apply_script=cur_element.has_apply_decl_script;
        let calcd_original=cur_element.calcd_original.map(|x|format!("{x}")).unwrap_or("_".to_string());
        let has_own_script=cur_element.has_self_script;
        // let has_template_use_script= cur_element.has_template_use_script;


        match &cur_element.element_type {
            ElementType::Node { names,ignore_applies,.. } => {
                println!("{indent}node {names:?}, e={cur_element_ind}, ignaps={ignore_applies:?}, created_from={created_from:?}, calcd_from={calcd_from:?}, params={params:?}, scr={has_script:?}, orig={calcd_original}, has_own_script={has_own_script}" , );
            }
            ElementType::TemplateUse { template_decl_element_ind, .. } => {
                let ElementType::TemplateDecl { name, .. }=elements.get(*template_decl_element_ind).unwrap().element_type else {panic!("");};

                println!("{indent}template use, e={cur_element_ind} : {name:?}, e2={template_decl_element_ind}, created_from={created_from:?}, calcd_from={calcd_from:?}, params={params:?}, scr={has_script:?}, orig={calcd_original}",);
            }
            ElementType::Apply { name,used,.. } => {
                println!("{indent}apply, e={cur_element_ind} : {name:?}, created_from={created_from:?}, calcd_from={calcd_from:?}, params={params:?}, used={used}, scr={has_script:?}, has_own_script={has_own_script}",);
            }
            ElementType::Attrib { name,in_node,calcd, ..  } => {
                println!("{indent}attrib {name:?}, e={cur_element_ind}, in_node={in_node}, calcd={calcd:?}, created_from={created_from:?}, calcd_from={calcd_from:?}, params={params:?}, scr={has_script:?}, orig={calcd_original}", );
            }
            ElementType::Script { .. } => {
                println!("{indent}script, e={cur_element_ind}, created_from={created_from:?}, calcd_from={calcd_from:?}, params={params:?}, scr={has_script:?}");
            }
            ElementType::TemplateDecl { name, used, .. } => {
                // let name=texts[*name];
                println!("{indent}template decl, e={cur_element_ind} : {name:?}, created_from={created_from:?}, calcd_from={calcd_from:?}, params={params:?}, used={used}, scr={has_script:?}, has_own_script={has_own_script}",);
            }
            ElementType::Stub { name } => {
                println!("{indent}stub {name:?}, e={cur_element_ind}, created_from={created_from:?}, calcd_from={calcd_from:?}, params={params:?}, scr={has_script:?}, orig={calcd_original}");
            }
            ElementType::ApplyUse { apply_decl_element_ind,   } => {
                // let ElementType::Apply { apply_decl_id, .. }=elements.get(*apply_decl_element_ind).unwrap().element_type else {panic!("");};
                println!("{indent}apply use, e={cur_element_ind} : e2={apply_decl_element_ind:?}, created_from={created_from:?}, calcd_from={calcd_from:?}, params={params:?}, scr={has_script:?}",);
            }
        }
        // println!("{indent}={:?}",cur_element.calcd_node_params);
    }
}
