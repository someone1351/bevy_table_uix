
use super::vals::*;

/*
TODO
* don't generate funcs for nodes/applies/templates that don't have any script tags

*/
pub fn mark_used(elements:&mut Vec<Element>) {
    for element in elements.iter_mut() {

        match &mut element.element_type {
            ElementType::Apply { used, .. } => {
                *used=false;
            }
            ElementType::TemplateDecl { used, .. } => {
                *used=false;
            }
            _ => {

            }
        }
    }

    //
    let mut work_stk=vec![0];

    while let Some(cur_element_ind)=work_stk.pop() {
        let cur_element=elements.get(cur_element_ind).unwrap();

        match &cur_element.element_type {
            ElementType::Apply { .. }|ElementType::TemplateDecl { .. } => {},
            _ => {
                work_stk.extend(cur_element.children.iter());
            }
        }

        match &cur_element.element_type {
            ElementType::ApplyUse { apply_decl_element_ind } => {
                let apply_decl_element_ind=*apply_decl_element_ind;
                let apply_element=elements.get_mut(apply_decl_element_ind).unwrap();
                let ElementType::Apply { used, .. }=&mut apply_element.element_type else {panic!("");};
                *used=true;
            },
            ElementType::TemplateUse { template_decl_element_ind } => {
                let template_decl_element_ind=*template_decl_element_ind;
                let tempalate_decl_element=elements.get_mut(template_decl_element_ind).unwrap();
                let ElementType::TemplateDecl { used, .. }=&mut tempalate_decl_element.element_type else {panic!("");};
                *used=true;
            },
            _ => {
            }
        }
    }
}
