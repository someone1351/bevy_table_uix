
use std::collections::HashMap;

use super::vals::*;

// pub fn mark_all_has_script(elements:&mut Vec<Element>) {
//     let mut work_stk=vec![0];

//     while let Some(cur_element_ind)=work_stk.pop() {
//         let cur_element=elements.get(cur_element_ind).unwrap();

//         if cur_element.use
//         work_stk.extend(cur_element.children.iter().rev());


//             let cur_element=elements.get_mut(cur_element_ind).unwrap();
//             cur_element.has_script=true;
//             cur_element.has_own_script=true;
//             cur_element.has_apply_decl_script=true;
//             cur_element.has_template_use_script=true;
//     }
// }

pub fn mark_has_script(elements:&mut Vec<Element>,) {

    let mut element_parent_map: HashMap<usize, usize>=HashMap::new(); //[element]=parent
    let mut work_stk=vec![0];

    while let Some(cur_element_ind)=work_stk.pop() {
        let cur_element=elements.get(cur_element_ind).unwrap();

        //
        match &cur_element.element_type {
            &ElementType::Apply { used, .. }|&ElementType::TemplateDecl { used, .. } if !used => {}
            _=>{
                work_stk.extend(cur_element.children.iter().rev());
                element_parent_map.extend(cur_element.children.iter().map(|&child_element_ind|(child_element_ind, cur_element_ind)));
            }
        }

        //
        let has_script=match &cur_element.element_type {
            ElementType::Script { .. } => true,
            &ElementType::TemplateUse { template_decl_element_ind  } => {
                let element=elements.get(template_decl_element_ind).unwrap();
                let &ElementType::TemplateDecl { used, .. }=&element.element_type else {panic!("");};
                used && element.has_script
            },
            _ => false,
        };

        //set ancestors to has_script
        if has_script {
            let mut element_ind=Some(cur_element_ind);
            let mut bla=false;

            while let Some(element_ind2)=element_ind {
                let element=elements.get_mut(element_ind2).unwrap();

                element.has_script=true;

                if !bla {
                    element.has_self_script=true;
                }

                if let ElementType::Apply {..}=&element.element_type {
                    bla=true;
                }

                element_ind=element_parent_map.get(&element_ind2).cloned();

            }
        }

    }
}
