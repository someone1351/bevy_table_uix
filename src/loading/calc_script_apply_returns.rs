

use super::vals::*;



#[derive(Clone)]
struct Work {
    element_ind:usize,
    parent:Option<usize>,
}

pub fn calc_script_returns(elements:&mut Vec<Element>,only_script:bool) {
    let only_script=true;

    let mut work_stk=vec![Work{ element_ind: 0, parent:None, }];

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=&elements[cur_work.element_ind];

        //push children on work_stk
        if let ElementType::Node{..}|ElementType::Stub{..}|ElementType::TemplateDecl{..}|ElementType::Apply{..}=&cur_element.element_type {
            work_stk.extend(cur_element.children.iter().rev().map(|&child|Work {
                element_ind: child,
                parent:Some(cur_work.element_ind),
            }));
        }

        //
        if cur_work.parent.is_none() || (only_script && !cur_element.has_script) {
            continue;
        }

        //
        if let ElementType::Node{..}|ElementType::Apply{..}|ElementType::TemplateDecl{..}=&cur_element.element_type {
        } else {
            continue;
        }

        //handle node/apply/template_decl returns

        //
        let mut return_items: Vec<(Option<ScriptSyntaxNode>,ScriptSyntaxTemplateUseOrApplyDecl)> = Vec::new();

        //apply/template uses returned by cur element's descendents
        for &child_element_ind in cur_element.children.iter() {
            let mut tmp_stk=vec![child_element_ind];

            while let Some(tmp_element_ind)=tmp_stk.pop() {
                let tmp_element=elements.get(tmp_element_ind).unwrap();

                match &tmp_element.element_type {
                    ElementType::Node{..}=>{
                        tmp_stk.extend(tmp_element.children.iter());

                        for &apply_element_ind in tmp_element.applies.iter() {
                            let apply_element=elements.get(apply_element_ind).unwrap();

                            if only_script && !apply_element.has_script {
                                continue;
                            }

                            return_items.push((Some(ScriptSyntaxNode(child_element_ind)),ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(apply_element_ind)));
                        }
                    }
                    &ElementType::TemplateUse{ template_decl_element_ind }=>{
                        let template_decl_element=elements.get(template_decl_element_ind).unwrap();

                        if only_script && !template_decl_element.has_apply_script {
                            continue;
                        }
                        if only_script && !template_decl_element.has_script {
                            continue;
                        }

                        let node=(tmp_element_ind!=child_element_ind).then_some(ScriptSyntaxNode(child_element_ind));
                        return_items.push((node,ScriptSyntaxTemplateUseOrApplyDecl::TemplateUse(tmp_element_ind)));
                    }
                    _=>{}
                }
            }
        }

        //apply uses returned from cur element
        for &apply_element_ind in cur_element.applies.iter() {
            let apply_element=elements.get(apply_element_ind).unwrap();

            if !only_script || apply_element.has_script {
                return_items.push((None,ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(apply_element_ind)));
            }
        }

        //
        elements[cur_work.element_ind].rets=return_items;
    } //end while

}
