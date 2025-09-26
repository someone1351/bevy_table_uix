

use super::vals::*;



#[derive(Clone)]
struct Work {
    element_ind:usize,
    depth:usize,
    exit:bool,
    parent:Option<usize>,
    in_a_use:bool,
    inside:Option<usize>,
}

pub fn calc_script_returns(elements:&mut Vec<Element>) {
    let only_script=true;
    let mut syntax_stk: Vec<usize> = vec![0];//Vec::new(); //syntax_ind


    let mut work_stk=vec![Work{ element_ind: 0, depth: 0, exit:false,parent:None, in_a_use:false,inside:None}];


    while let Some(cur_work)=work_stk.pop() {
        let cur_element=&elements[cur_work.element_ind];

        //push children on work_stk
        if !cur_work.exit { //enter
            if let ElementType::Node{..}|ElementType::Stub{..}|ElementType::TemplateDecl{..}|ElementType::Apply{..}
                |ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type
            {
                let in_use = if let ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type {
                    true
                } else {
                    cur_work.in_a_use
                };

                let inside = if let ElementType::Node{..}|ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type {
                    Some(cur_work.element_ind)
                } else {
                    cur_work.inside
                };

                work_stk.push(Work{exit:true, ..cur_work.clone()});
                work_stk.extend(cur_element.children.iter().rev().map(|&child|Work {
                    element_ind: child, depth: cur_work.depth+1, exit:false,
                    parent:Some(cur_work.element_ind),
                    in_a_use: in_use,
                    inside,
                }));
            }
        }


        //handle node/apply/template_decl returns
        if !cur_work.in_a_use && cur_work.exit
            && (!only_script || cur_element.has_script)
            && match &cur_element.element_type
        {
            ElementType::Node{..} if cur_work.parent.is_some() => true,
            ElementType::Apply{..} => true,
            ElementType::TemplateDecl{..}=>true, //added
            _=>false,
        } {
            let mut return_items: Vec<(Option<ScriptSyntaxNode>,ScriptSyntaxTemplateUseOrApplyDecl)> = Vec::new();

            //apply/template uses returned by cur element's descendents
            for &child_element_ind in cur_element.children.iter() {

                //
                let mut tmp_stk=vec![child_element_ind];

                while let Some(tmp_element_ind)=tmp_stk.pop() {
                    let tmp_element=elements.get(tmp_element_ind).unwrap();

                    // if tmp_element.has_apply_script {} //should wrap below in this?

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
                            // // if tmp_element.has_apply_decl_script
                            if !only_script || template_decl_element.has_script
                            {

                                if tmp_element_ind==child_element_ind {
                                    return_items.push((None,ScriptSyntaxTemplateUseOrApplyDecl::TemplateUse(tmp_element_ind)));
                                } else {
                                    return_items.push((Some(ScriptSyntaxNode(child_element_ind)),ScriptSyntaxTemplateUseOrApplyDecl::TemplateUse(tmp_element_ind)));
                                }
                            }
                        }
                        _=>{}
                    }
                }
            }

            //apply uses returned from cur element
            for &apply_element_ind in cur_element.applies.iter() {
                let apply_element=elements.get(apply_element_ind).unwrap();

                if only_script && !apply_element.has_script {
                    continue;
                }

                return_items.push((None,ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(apply_element_ind)));
            }

            let last_func_syntax_ind=syntax_stk.last().cloned().unwrap();
            let decl_element=&mut elements[last_func_syntax_ind];
            decl_element.rets=return_items;

        } //end handle node/apply/template_decl returns


        //

        let cur_element=&elements[cur_work.element_ind];

        //
        if !cur_work.in_a_use //ie not in a template/apply use element
            // // && (cur_element.has_own_script||cur_element.has_template_use_script||cur_element.has_apply_decl_script)

            && (!only_script || cur_element.has_script)
            // // && cur_element.
        {

            match &cur_element.element_type {
                ElementType::Node {..} if cur_work.depth==0 => { //root
                }
                ElementType::Node {..}|ElementType::Apply {..}|ElementType::TemplateDecl {..} => {
                    if !cur_work.exit { //enter
                        syntax_stk.push(cur_work.element_ind);
                    } else { //exit
                        syntax_stk.pop().unwrap();
                    }
                }
                _ => {}
            }

        }

    } //end while

}
