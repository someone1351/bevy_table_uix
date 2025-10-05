

use super::vals::*;

struct Work {
    element_ind:usize,
    apply_in_template_use:bool, //for applies declared in template_use
}


pub fn mark_has_script(elements:&mut Vec<Element>,) {
    let mut work_stk=vec![Work{ element_ind: 0, apply_in_template_use: false }];

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();

        //
        if cur_element.calcd_from_element_ind.is_some() {
            panic!("mark_has_script, shouldn't be traversing generated elements");
        }

        //
        match &cur_element.element_type {
            &ElementType::Apply { used, .. }|&ElementType::TemplateDecl { used, .. } if !used => {}
            ElementType::ApplyUse {..} => {} //do children later in mark_has_script_rest
            ElementType::TemplateUse {..} => {
                //for applies declared in template_use
                work_stk.extend(cur_element.children.iter().rev().filter_map(|&child_element_ind|{
                    let child_element=&elements[child_element_ind];
                    child_element.element_type.is_apply().then_some(Work { element_ind: child_element_ind, apply_in_template_use: true })
                }));
            }
            _=>{
                work_stk.extend(cur_element.children.iter().rev().map(|&child_element_ind|Work{
                    element_ind: child_element_ind, apply_in_template_use: cur_work.apply_in_template_use,
                }));
            }
        }

        //
        let (has_script, has_self_script,has_env_script,has_apply_script)=match &cur_element.element_type {
            ElementType::Script { .. } => (true,true,true,false),
            &ElementType::TemplateUse { template_decl_element_ind  } => { //template_decl will always appear before the template_use,
                let decl_element=elements.get(template_decl_element_ind).unwrap();
                let &ElementType::TemplateDecl { used, .. }=&decl_element.element_type else {panic!("");};
                used.then_some((decl_element.has_script, decl_element.has_self_script,decl_element.has_env_script,decl_element.has_apply_script)).unwrap_or_default()
                // (used && decl_element.has_script, used && decl_element.has_self_script,used && decl_element.has_env_script)
            },
            _ => (false,false,false,false),
        };

        //set ancestors to has_script
        if has_script {
            let mut element_ind=Some(cur_work.element_ind);
            let mut has_apply_script=has_apply_script;

            while let Some(element_ind2)=element_ind {
                let element=elements.get_mut(element_ind2).unwrap();

                if element.element_type.is_apply() {
                    has_apply_script=true;
                }

                //
                if !element.element_type.is_template_use() || !cur_work.apply_in_template_use //for applies declared in template_use
                {

                    //
                    if has_apply_script
                        // && !element.element_type.is_template_use()
                    {
                        element.has_apply_script=true;
                    }

                    //

                    element.has_script=true;
                }

                //
                element_ind=element.parent;
            }
        }

        //set ancestors to has_self_script
        if has_self_script {
            let mut element_ind=Some(cur_work.element_ind);

            while let Some(element_ind2)=element_ind {
                let element=elements.get_mut(element_ind2).unwrap();
                element.has_self_script=true;

                match &element.element_type {
                    ElementType::TemplateUse {..}|ElementType::Script {..} => {
                    }
                    _ => {
                        break;
                    }
                }

                element_ind=element.parent;
            }
        }

        //
        if has_env_script {
            let cur_element=elements.get_mut(cur_work.element_ind).unwrap();

            if let ElementType::TemplateUse { .. }=&cur_element.element_type {
                cur_element.has_env_script=true;
            } else { //parent = node/apply/template_decl/stub
                let parent_element_ind=cur_element.parent.unwrap();
                let parent_element=elements.get_mut(parent_element_ind).unwrap();
                parent_element.has_env_script=true;
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
            // let has_apply_script=element.has_apply_script;

            let cur_element=&mut elements[cur_element_ind];
            cur_element.has_script=has_script;
            cur_element.has_self_script=has_self_script;
            cur_element.has_env_script=has_env_script;
            // cur_element.has_apply_script=has_apply_script;
        }
    }
}