
use std::cmp::Ordering;



use super::vals::*;


struct ApplyCallStkItem {
    inside_element_ind:Option<usize>,
    parent_element_ind:usize,
    apply_use_element_ind:usize,
}


#[derive(Clone)]
struct Work {
    element_ind:usize,
    depth:usize,
    exit:bool,
    parent:Option<usize>,
    in_a_use:bool,
    inside:Option<usize>,
}

pub fn calc_script_apply_calls(elements:&mut Vec<Element>,only_script:bool )  {
    // let only_script=true;


    let mut work_stk=vec![Work{ element_ind: 0, depth: 0, exit:false,parent:None, in_a_use:false,inside:None}];

    let mut apply_calls_stk: Vec<Vec<ApplyCallStkItem>> = Vec::new();

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


        //call applies, at root node and stubs
        if cur_work.exit
            && !cur_work.in_a_use
            && match &cur_element.element_type {  //on exit, not in apply/template use
            ElementType::Node{..} if cur_work.depth==0 => true,
            ElementType::Stub{..} => true,
            _ => false,
        } {
            let mut element_apply_calls=Vec::new();

            let mut apply_calls=apply_calls_stk.pop().unwrap();

            apply_calls.sort_by(|x,y|{
                match x.apply_use_element_ind.cmp(&y.apply_use_element_ind) {
                    Ordering::Equal=>x.inside_element_ind.cmp(&y.inside_element_ind),
                    r=>r,
                }
            });

            for apply_call in apply_calls.iter() {
                let apply_use_element=elements.get(apply_call.apply_use_element_ind).unwrap();
                // let ElementType::ApplyUse { apply_decl_element_ind,  }=&apply_use_element.element_type else {panic!("");};

                let apply_in = {
                    if let ElementType::Stub{..} = &cur_element.element_type {
                        let mut b=false;
                        let mut cur_from=apply_use_element.calcd_created_from;

                        while cur_from!=0 {
                            let from_element=elements.get(cur_from).unwrap();

                            if from_element.calcd_created_from==cur_work.element_ind {
                                b=true;
                                break;
                            }

                            cur_from=from_element.calcd_created_from;
                        }

                        b
                    } else {
                        true
                    }
                };

                //
                let mut from_ret = None;
                let mut from_template_decls = Vec::new();
                if apply_in
                {
                    let mut cur_from=apply_use_element.calcd_created_from;
                    let to=if let ElementType::Stub{..} = &cur_element.element_type { cur_work.element_ind } else {0};

                    //
                    while cur_from!=to {
                        let from_element=elements.get(cur_from).unwrap();

                        match &from_element.element_type {
                            ElementType::ApplyUse { .. } => {
                                from_ret= Some(ScriptSyntaxNodeOrApplyUse::ApplyUse(cur_from));
                                break;
                            }
                            ElementType::TemplateUse { .. } => {
                                let cur_from=from_element.calcd_from_element_ind.unwrap_or(cur_from);
                                from_template_decls.push(ScriptSyntaxTemplateUse(cur_from));
                            }
                            ElementType::Node { .. } if from_element.calcd_created_from==to => {
                                from_ret= Some(ScriptSyntaxNodeOrApplyUse::Node(cur_from));
                                break; //not necessary
                            }
                            _ => { //will skip over nodes,
                            }
                        }

                        cur_from=from_element.calcd_created_from;
                    }
                }

                from_template_decls.reverse();

                //
                if from_ret.is_none() && !from_template_decls.is_empty() {
                    panic!("");
                }


                element_apply_calls.push(ElementApplyCall {
                    func_froms:from_ret.map(|from_ret|(from_ret,from_template_decls)),
                    parent_element_ind: apply_call.parent_element_ind,
                    apply_use_element_ind:apply_call.apply_use_element_ind,
                });
            }

            if !element_apply_calls.is_empty() {
                let cur_element=&mut elements[cur_work.element_ind];
                cur_element.apply_calls=element_apply_calls;
            }
        } //call applies end


        let cur_element=&elements[cur_work.element_ind];

        //add apply call to stk
        if !cur_work.exit
            // // && cur_element.has_script
        { //enter //
            if let &ElementType::ApplyUse{ apply_decl_element_ind } = &cur_element.element_type {
                let apply_element=elements.get(apply_decl_element_ind).unwrap();
                // // let ElementType::Apply {used , ..}=&apply_element.element_type else {panic!("");};

                if !only_script || apply_element.has_script
                {
                    apply_calls_stk.last_mut().unwrap().push(ApplyCallStkItem {
                        inside_element_ind:cur_work.inside,
                        parent_element_ind:cur_work.parent.unwrap(),
                        apply_use_element_ind:cur_work.element_ind,
                    });
                }
            }
        } //end add apply call to stk


        //
        if !cur_work.in_a_use && !cur_work.exit && match &cur_element.element_type {  //enter
            ElementType::Node{..} if cur_work.depth==0 => true,
            ElementType::Stub{..} => true,
            _ => false,
        } {
            apply_calls_stk.push(Vec::new());
        }

    } //end while

    //
}
