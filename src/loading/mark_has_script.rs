

use super::vals::*;


pub fn mark_has_script(elements:&mut Vec<Element>,) {
    let mut work_stk: Vec<usize>=vec![0];

    while let Some(cur_element_ind)=work_stk.pop() {
        let cur_element=elements.get(cur_element_ind).unwrap();

        //
        if cur_element.calcd_from_element_ind.is_some() {
            panic!("mark_has_script, shouldn't be traversing generated elements");
        }

        //
        match &cur_element.element_type {
            &ElementType::Apply { used, .. }|&ElementType::TemplateDecl { used, .. } if !used => {}
            ElementType::TemplateUse {..}|ElementType::ApplyUse {..} => {} //do children later
            _=>{
                work_stk.extend(cur_element.children.iter().rev());
            }
        }

        //
        let (has_script, has_self_script,has_env_script)=match &cur_element.element_type {
            ElementType::Script { .. } => (true,true,true),
            &ElementType::TemplateUse { template_decl_element_ind  } => { //template_decl will always be before template_use,
                let element=elements.get(template_decl_element_ind).unwrap();
                let &ElementType::TemplateDecl { used, .. }=&element.element_type else {panic!("");};
                (used && element.has_script, used && element.has_self_script,false)
            },
            _ => (false,false,false),
        };

        //set ancestors to has_script
        if has_script {
            let mut element_ind=Some(cur_element_ind);

            while let Some(element_ind2)=element_ind {
                let element=elements.get_mut(element_ind2).unwrap();
                element.has_script=true;
                element_ind=element.parent;
            }
        }

        //set ancestors to has_self_script
        if has_self_script {
            let mut element_ind=Some(cur_element_ind);

            while let Some(element_ind2)=element_ind {
                let element=elements.get_mut(element_ind2).unwrap();
                element.has_self_script=true;

                match &element.element_type {
                    ElementType::TemplateUse {..}|ElementType::Script {..} => {}
                    _ => {
                        break;
                    }
                }

                element_ind=element.parent;
            }
        }
    }
}

pub fn mark_has_script_rest(elements:&mut Vec<Element>,) {
    let mut work_stk: Vec<usize>=vec![0];

    while let Some(cur_element_ind)=work_stk.pop() {
        let cur_element=elements.get(cur_element_ind).unwrap();

        //
        work_stk.extend(cur_element.children.iter().rev());

        //
        if let Some(element_ind)=cur_element.calcd_from_element_ind {
            let element=&elements[element_ind];
            let has_script=element.has_script;
            let has_self_script=element.has_self_script;
            let has_env_script=element.has_env_script;

            let cur_element=&mut elements[cur_element_ind];
            cur_element.has_script=has_script;
            cur_element.has_self_script=has_self_script;
            cur_element.has_env_script=has_env_script;
        }
    }
}