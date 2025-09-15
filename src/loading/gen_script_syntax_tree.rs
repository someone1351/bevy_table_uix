
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

pub fn gen_script_syntax_tree(elements:&Vec<Element>) -> Vec<ScriptSyntax> {
    let mut syntax_tree: Vec<ScriptSyntax> = vec![ScriptSyntax::Root { children: Vec::new(),  }];
    let mut syntax_stk: Vec<usize> = vec![0];//Vec::new(); //syntax_ind


    let mut work_stk=vec![Work{ element_ind: 0, depth: 0, exit:false,parent:None, in_a_use:false,inside:None}];

    let mut apply_calls_stk: Vec<Vec<ApplyCallStkItem>> = Vec::new();

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();

        // if !cur_element.has_script {
        //     // continue;
        // }

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

                //
                if from_ret.is_none() && !from_template_decls.is_empty() {
                    panic!("");
                }

                let apply_use_element_ind=apply_call.apply_use_element_ind;
                let ElementType::ApplyUse { apply_decl_element_ind }=&elements.get(apply_use_element_ind).unwrap().element_type else {panic!("");};
                let apply_decl_element=elements.get(*apply_decl_element_ind).unwrap();

                let ret=apply_decl_element.has_apply_decl_script.then_some(ScriptSyntaxApplyUse(apply_use_element_ind));

                let new_syntax_ind=syntax_tree.len();
                syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                // // let params=[apply_call.parent_element_ind].iter().chain().collect();
                // let mut params=vec![ScriptSyntaxNode(apply_call.parent_element_ind)];
                let mut params=Vec::new();
                let mut not_has_self=None;
                if apply_decl_element.has_own_script || apply_decl_element.has_template_use_script {
                    params.push(ScriptSyntaxNode(apply_call.parent_element_ind));
                } else {
                    not_has_self=Some(apply_call.parent_element_ind);
                }


                params.extend(apply_use_element.calcd_node_params.iter()
                    // .filter_map(|&param_element_ind|{
                    //     let param_element=elements.get(param_element_ind).unwrap();
                    //     let orig_element=param_element.calcd_original.map(|orig_element_ind|elements.get(orig_element_ind).unwrap());
                    //     // let orig_element=param_element.calcd_from_element_ind.map(|orig_element_ind|elements.get(orig_element_ind).unwrap());


                    //     let has_script=orig_element.map(|x|x.has_script).unwrap_or(param_element.has_script);
                    //     has_script.then_some(ScriptSyntaxNode(param_element_ind))
                    // })
                        .map(|&param_element_ind|ScriptSyntaxNode(param_element_ind))
                    );

                syntax_tree.push(ScriptSyntax::CallApply {
                    ret,
                    // func_froms: froms.iter().rev().cloned().collect(),
                    func_froms:from_ret.map(|from_ret|(from_ret,from_template_decls)),
                    func_apply: ScriptSyntaxApplyDecl(*apply_decl_element_ind),
                    params,
                    not_has_self
                });
            }
        }

        //add apply call to stk
        if !cur_work.exit { //enter //&& cur_element.has_script
            if let ElementType::ApplyUse{ apply_decl_element_ind } = &cur_element.element_type {
                let apply_element=elements.get(*apply_decl_element_ind).unwrap();
                // // let ElementType::Apply {used , ..}=&apply_element.element_type else {panic!("");};

                // if !apply_element.has_script {
                //     continue;
                // }

                apply_calls_stk.last_mut().unwrap().push(ApplyCallStkItem {
                    inside_element_ind:cur_work.inside,
                    parent_element_ind:cur_work.parent.unwrap(),
                    apply_use_element_ind:cur_work.element_ind,
                });
            }
        }

        //handle node,apply,template_decl returns
        if !cur_work.in_a_use && cur_work.exit
            // && cur_element.has_script
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
                // if !elements.get(child_element_ind).unwrap().has_script {
                //     continue;
                // }

                //
                let mut tmp_stk=vec![child_element_ind];

                while let Some(tmp_element_ind)=tmp_stk.pop() {
                    let tmp_element=elements.get(tmp_element_ind).unwrap();

                    // if tmp_element.has_apply_script {} //should wrap below in this?

                    match &tmp_element.element_type {
                        ElementType::Node{..}=>{
                            tmp_stk.extend(tmp_element.children.iter());

                            for &apply_element_ind in tmp_element.applies.iter() {
                                // let apply_element=elements.get(apply_element_ind).unwrap();

                                // if !apply_element.has_script {
                                //     continue;
                                // }

                                return_items.push((Some(ScriptSyntaxNode(child_element_ind)),ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(apply_element_ind)));
                            }
                        }
                        ElementType::TemplateUse{..}=>{
                            // if tmp_element.has_apply_decl_script
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
                // let apply_element=elements.get(apply_element_ind).unwrap();

                // if !apply_element.has_script {
                //     continue;
                // }

                return_items.push((None,ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(apply_element_ind)));
            }

            let last_func_syntax_ind=syntax_stk.last().cloned().unwrap();
            let ScriptSyntax::Decl { returns, .. }=syntax_tree.get_mut(last_func_syntax_ind).unwrap() else {panic!("");};
            *returns = return_items;
        }

        //
        if !cur_work.in_a_use && !cur_work.exit && match &cur_element.element_type {  //enter
            ElementType::Node{..} if cur_work.depth==0 => true,
            ElementType::Stub{..} => true,
            _ => false,
        } {
            apply_calls_stk.push(Vec::new());
        }

        if !cur_work.in_a_use {
            match &cur_element.element_type {
                ElementType::Stub{name,..} if !cur_work.exit => { //enter
                    //
                    // apply_calls_stk.push(Vec::new());

                    //stub func
                    let new_syntax_ind=syntax_tree.len();
                    syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                    syntax_stk.push(new_syntax_ind);
                    syntax_tree.push(ScriptSyntax::Stub { name: name.to_string(), children: Vec::new(), });

                    //call stub
                    // let new_syntax_ind=syntax_tree.len();
                    // syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                    // syntax_tree.push(ScriptSyntax::CallStub { is_root:true, stub: cur_work.element_ind } );
                }
                ElementType::Stub{..} => { //exit
                    syntax_stk.pop().unwrap();
                }
                _ => {

                }
            }
        }
        //
        if !cur_work.in_a_use
            // // && (cur_element.has_own_script||cur_element.has_template_use_script||cur_element.has_apply_decl_script)

            // && cur_element.has_script
            // // && cur_element.
        {

            match &cur_element.element_type {
                ElementType::Node {..} if cur_work.depth==0 => { //root
                }
                ElementType::Node {..} => {
                    if !cur_work.exit { //enter
                        let new_syntax_ind=syntax_tree.len();
                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                        syntax_stk.push(new_syntax_ind);

                        // let self_param=cur_element.has_own_script || cur_element.has_template_use_script;
                        let self_param=true;

                        let params= cur_element.calcd_node_params.iter()
                            // .filter_map(|&param_element_ind|{
                            //     let param_element=elements.get(param_element_ind).unwrap();
                            //     let orig_element=param_element.calcd_original
                            //         .map(|orig_element_ind|elements.get(orig_element_ind).unwrap())
                            //         .unwrap_or(param_element);
                            //     let has_script=orig_element.has_own_script || orig_element.has_template_use_script;
                            //     has_script.then_some(ScriptSyntaxNode(param_element_ind))
                            // })
                            .map(|&param_element_ind|ScriptSyntaxNode(param_element_ind))
                            .collect::<Vec<_>>();

                        syntax_tree.push(ScriptSyntax::Decl {
                            name: ScriptSyntaxNodeOrApplyOrTemplate::Node(cur_work.element_ind),
                            params,
                            children: Vec::new(),
                            returns: Vec::new(),
                            self_param,
                        });
                    } else { //exit
                        let parent_element=elements.get(cur_work.parent.unwrap()).unwrap();

                        syntax_stk.pop().unwrap();
                        let new_syntax_ind=syntax_tree.len();
                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                        let in_func=if let ElementType::Stub{..}=&parent_element.element_type {false}else{cur_work.depth!=1};

                        let mut params=Vec::new();

                        if cur_element.has_own_script || cur_element.has_template_use_script {
                            params.push(ScriptSyntaxNode(cur_work.element_ind));
                        }

                        params.extend(cur_element.calcd_node_params.iter()
                            // .filter_map(|&param_element_ind|{
                            //     let param_element=elements.get(param_element_ind).unwrap();
                            //     let orig_element=param_element.calcd_original.map(|orig_element_ind|elements.get(orig_element_ind).unwrap());
                            //     let has_script=orig_element.map(|x|x.has_script).unwrap_or(param_element.has_script);
                            //     has_script.then_some(ScriptSyntaxNode(param_element_ind))
                            // })
                            .map(|&param_element_ind|ScriptSyntaxNode(param_element_ind))
                    );

                        syntax_tree.push(ScriptSyntax::CallNode {
                            ret:cur_element.has_apply_decl_script,
                            in_func,
                            func: ScriptSyntaxNode(cur_work.element_ind),
                            params,

                        });
                    }
                }
                ElementType::Attrib {..} => {
		            continue;
                }
                ElementType::Script { record } => {
                    if !cur_work.exit { //enter
                        for t in record.text_values() {
                            let new_syntax_ind=syntax_tree.len();
                            syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                            syntax_tree.push(ScriptSyntax::Insert { path: t.path().map(|x|x.to_path_buf()), loc: t.start_loc(), insert: t.str().to_string() } );
                        }

                        continue;
                    } else { //exit

                    }
                }
                ElementType::Apply {..} => {
                    if !cur_work.exit { //enter
                        let params=cur_element.calcd_node_params.iter()
                            // .filter_map(|&param_element_ind|{
                            //     let param_element=elements.get(param_element_ind).unwrap();
                            //     // let param_element_ind2=param_element.calcd_original.unwrap_or(param_element_ind);
                            //     // let param_element2=elements.get(param_element_ind2).unwrap();

                            //     let orig_element=param_element.calcd_original
                            //         .map(|orig_element_ind|elements.get(orig_element_ind).unwrap())
                            //         .unwrap_or(param_element);

                            //         if orig_element.has_own_script || orig_element.has_template_use_script {
                            //             Some(ScriptSyntaxNode(param_element_ind))
                            //         } else {
                            //             None
                            //         }
                            //     // orig_element.has_script.then_some(ScriptSyntaxNode(param_element_ind))
                            // })

                            .map(|&param_element_ind|ScriptSyntaxNode(param_element_ind))
                            .collect::<Vec<_>>();

                        //

                        // let self_param=cur_element.has_own_script || cur_element.has_template_use_script;
                        let self_param=true;

                        let new_syntax_ind=syntax_tree.len();

                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                        syntax_stk.push(new_syntax_ind);
                        syntax_tree.push(ScriptSyntax::Decl {
                            // decl: ScriptSyntaxDecl::Apply,
                            name: ScriptSyntaxNodeOrApplyOrTemplate::Apply(cur_work.element_ind),
                            params,
                            children: Vec::new(),
                            returns: Vec::new(),
                            self_param ,
                        });
                    } else { //exit
		                syntax_stk.pop().unwrap();
                    }
                }
                ElementType::ApplyUse {..} => {
                    if !cur_work.exit { //enter
		                continue;
                    } else { //exit

                    }
                }
                ElementType::TemplateDecl {..} => {
                    if !cur_work.exit { //enter
                        let params=cur_element.calcd_node_params.iter()
                            // .filter_map(|&param_element_ind|{
                            //     let param_element=elements.get(param_element_ind).unwrap();

                            //     let orig_element=param_element.calcd_original
                            //         .map(|orig_element_ind|elements.get(orig_element_ind).unwrap())
                            //         .unwrap_or(param_element);

                            //         if orig_element.has_own_script || orig_element.has_template_use_script {
                            //             Some(ScriptSyntaxNode(param_element_ind))
                            //         } else {
                            //             None
                            //         }
                            //     // orig_element.has_script.then_some(ScriptSyntaxNode(param_element_ind))
                            // })
                            .map(|&param_element_ind|ScriptSyntaxNode(param_element_ind))
                            .collect::<Vec<_>>();


                        // let self_param=cur_element.has_own_script || cur_element.has_template_use_script;
                        let self_param=true;

                        let new_syntax_ind=syntax_tree.len();
                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                        syntax_stk.push(new_syntax_ind);

                        syntax_tree.push(ScriptSyntax::Decl {
                            name: ScriptSyntaxNodeOrApplyOrTemplate::Template(cur_work.element_ind),
                            params,
                            children: Vec::new(),
                            returns: Vec::new(),
                            self_param,
                        });
                    } else { //exit
		                syntax_stk.pop().unwrap();
                    }
                }
                ElementType::TemplateUse { template_decl_element_ind } => {
                    if !cur_work.exit { //enter
                        let new_syntax_ind=syntax_tree.len();
                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                        // let template_decl_element=elements.get(*template_decl_element_ind).unwrap();
                        let ret=cur_element.has_apply_decl_script.then_some(ScriptSyntaxTemplateUse(cur_work.element_ind));

                        let params=cur_element.calcd_node_params.iter()
                            // .filter_map(|&param_element_ind|{
                            //     let param_element=elements.get(param_element_ind).unwrap();
                            //     let orig_element=param_element.calcd_original.map(|orig_element_ind|elements.get(orig_element_ind).unwrap());
                            //     let has_script=orig_element.map(|x|x.has_script).unwrap_or(param_element.has_script);
                            //     has_script.then_some(ScriptSyntaxNode(param_element_ind))
                            // })
                            .map(|&param_element_ind|ScriptSyntaxNode(param_element_ind))
                            .collect();

                        // let has_self=template_decl_element.has_own_script || template_decl_element.has_template_use_script;
                        let has_self=true;

                        syntax_tree.push(ScriptSyntax::CallTemplate {
                            ret,
                            func: ScriptSyntaxTemplateDecl(*template_decl_element_ind),
                            params,
                            has_self,
                            // use_self,
                        });

                        continue;
                    } else { //exit

                    }
                }
                ElementType::Stub {..} => {
                }
            }

        }

        //call stubs
        if !cur_work.in_a_use && !cur_work.exit && match &cur_element.element_type { //enter
            ElementType::Node{..} if cur_work.depth==0 => true,
            ElementType::Stub{..} => true,
            _ => false,
        } {
            let new_syntax_ind=syntax_tree.len();
            syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

            let is_root=if let ElementType::Stub{..}=&cur_element.element_type{false}else{true};
            // let has_script = cur_element.has_script;

            syntax_tree.push(ScriptSyntax::CallStub {
                is_root,
                // has_script,
                stub: cur_work.element_ind,
            } );
        }
    } //end while

    //
    syntax_tree
}

pub fn debug_print_script_syntax_tree(syntax_tree:&Vec<ScriptSyntax>) {
    let mut stk=vec![(0,0)]; //ind,depth

    while let Some((cur_ind,depth))=stk.pop() {
        let indent="    ".repeat(depth);
        let cur=syntax_tree.get(cur_ind).unwrap();
        match cur {
            ScriptSyntax::Root { .. } => {
                println!("{indent}root");
            }
            ScriptSyntax::Insert {..} => {
                println!("{indent}script");
            }
            ScriptSyntax::Stub { name, ..  } => {
                println!("{indent}stub {name:?}");
            }
            ScriptSyntax::CallStub { is_root, stub, .. } => {
                println!("{indent}call_stub {stub}, is_root={is_root}");
            }
            ScriptSyntax::CallTemplate { ret, func, params, has_self  } => {
                println!("{indent}call_template {func:?}({}{params:?}) => {ret:?}, ",has_self.then(||"self,").unwrap_or_default());
            }
            ScriptSyntax::CallApply { ret, func_froms, func_apply, params, not_has_self: has_self } => {
                println!("{indent}call_apply {func_froms:?} {func_apply:?}({has_self:?},{params:?}) => {ret:?}");
            }
            ScriptSyntax::CallNode { in_func, func, params, ret,   } => {
                println!("{indent}call_node {func:?}({params:?}), in_func={in_func}, ret={ret}, ");
            }
            ScriptSyntax::Decl { name, params, returns, ..  } => {
                println!("{indent}decl {name:?}({params:?}), => {returns:?}");
            }
        }

        if let Some(children)=cur.get_children() {
            stk.extend(children.iter().map(|&child_ind|(child_ind,depth+1)).rev());
        }

    }
}
