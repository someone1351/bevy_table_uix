
use std::collections::HashMap;

use super::vals::*;


pub fn mark_has_script(elements:&mut Vec<Element>) {
    let mut element_parent_map: HashMap<usize, usize>=HashMap::new(); //[element]=parent
    let mut work_stk=vec![0];

    while let Some(cur_element_ind)=work_stk.pop() {
        let cur_element=elements.get(cur_element_ind).unwrap();

        if cur_element.has_script {
            // break;
        }

        //
        match &cur_element.element_type {
            ElementType::Apply { used, .. }|ElementType::TemplateDecl { used, .. } if !(*used) => {
            }
            _=>{
                work_stk.extend(cur_element.children.iter().rev());
                element_parent_map.extend(cur_element.children.iter().map(|&child_element_ind|(child_element_ind, cur_element_ind)));
            }
        }



        let has_own_script=if let ElementType::Script { .. }=&cur_element.element_type {true}else{false};
        let has_template_use_script =if let &ElementType::TemplateUse { template_decl_element_ind  }=&cur_element.element_type {
            let template_decl_element=elements.get(template_decl_element_ind).unwrap();
            template_decl_element.has_own_script
        }else{false};
        //
        let has_script=match &cur_element.element_type {
            ElementType::Script { .. } => true,
            &ElementType::TemplateUse { template_decl_element_ind  } => {
                let element=elements.get(template_decl_element_ind).unwrap();
                let &ElementType::TemplateDecl {
                    // name,
                    used,
                    ..
                }=&element.element_type else {panic!("");};
                // println!("---- {name:?} used={used}, script={}",element.has_script);

                used && element.has_script
            },
            _ => false,
        };

        //for apply ret? in a template
        let has_apply_decl_ret_script=match cur_element.element_type {
            ElementType::TemplateUse { template_decl_element_ind  } => {
                let element=elements.get(template_decl_element_ind).unwrap();
                let ElementType::TemplateDecl { .. }=&element.element_type else {panic!("");};

                element.has_apply_decl_script
            },
            _ => false,
        };

        //has_apply_script means it returns an apply decl func or a template decl that has a descendent that is an apply decl func with script

        //
        if has_own_script || has_template_use_script
        {
            let parent_element_ind=element_parent_map.get(&cur_element_ind).cloned().unwrap();
            let parent_element=elements.get_mut(parent_element_ind).unwrap();
            parent_element.has_own_script|=has_own_script;
            parent_element.has_template_use_script|=has_template_use_script;
        }
        // cur_element

        //set ancestors to has_script
        if has_script {
            let mut has_apply_script=has_apply_decl_ret_script;
            let mut element_ind=Some(cur_element_ind);

            while let Some(element_ind2)=element_ind {
                let element=elements.get_mut(element_ind2).unwrap();


                // match &element.element_type {
                //     ElementType::Apply { used, .. }|ElementType::TemplateDecl { used, .. } if !(*used) => {
                //         break;
                //     }
                //     _ => {
                element.has_script=true;
                element.has_apply_decl_script=element.has_apply_decl_script||has_apply_script;
                element_ind=element_parent_map.get(&element_ind2).cloned();
                //     }
                // }

                //for apply ret
                if let ElementType::Apply{..} = &element.element_type {
                    has_apply_script=true;
                }

                // match &element.element_type {
                //     ElementType::Apply{..} => {
                //         has_apply_script=true;
                //     }
                //     _ => {
                //     }
                // }
            }
        }

    }
}
