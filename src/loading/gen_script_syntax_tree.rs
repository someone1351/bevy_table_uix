
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

pub fn gen_script_syntax_tree(elements:&Vec<Element>, only_used:bool,only_script:bool) -> Vec<ScriptSyntax> {
    let mut syntax_tree: Vec<ScriptSyntax> = vec![ScriptSyntax::Root { children: Vec::new(),  }];
    let mut syntax_stk: Vec<usize> = vec![0];//Vec::new(); //syntax_ind


    let mut work_stk=vec![Work{ element_ind: 0, depth: 0, exit:false,parent:None, in_a_use:false,inside:None}];

    let mut apply_calls_stk: Vec<Vec<ApplyCallStkItem>> = Vec::new();

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();

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
                let &ElementType::ApplyUse { apply_decl_element_ind }=&elements.get(apply_use_element_ind).unwrap().element_type else {panic!("");};
                let apply_decl_element=elements.get(apply_decl_element_ind).unwrap();

                // let ret=apply_decl_element.has_apply_decl_script.then_some(ScriptSyntaxApplyUse(apply_use_element_ind));
                // let ret=Some(ScriptSyntaxApplyUse(apply_use_element_ind));
                let ret=ScriptSyntaxApplyUse(apply_use_element_ind);
                let new_syntax_ind=syntax_tree.len();
                syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                let mut params=Vec::new();

                // let mut not_has_self=None;
                let self_node=ScriptSyntaxNode(apply_call.parent_element_ind);

                let self_param=!only_script || apply_decl_element.has_self_script;


                // if apply_decl_element.has_own_script || apply_decl_element.has_template_use_script
                if self_param {
                    // params.push(ScriptSyntaxNode(apply_call.parent_element_ind));
                    params.push(self_node);
                }
                //  else {
                //     not_has_self=Some(apply_call.parent_element_ind);
                // }


                params.extend(apply_use_element.calcd_node_params.iter().zip(apply_decl_element.calcd_node_params.iter()).filter_map(|(&param_element_ind,&decl_param_element_ind)|{
                    let param_element=&elements[decl_param_element_ind];
                    (!only_script || param_element.has_self_script).then_some(ScriptSyntaxNode(param_element_ind))
                }));

                syntax_tree.push(ScriptSyntax::CallApply {
                    ret,
                    // func_froms: froms.iter().rev().cloned().collect(),
                    func_froms:from_ret.map(|from_ret|(from_ret,from_template_decls)),
                    func_apply: ScriptSyntaxApplyDecl(apply_decl_element_ind),
                    params,
                    // not_has_self
                    self_node,
                    has_ret:true,
                    has_self:self_param,
                });
            }
        } //call applies end

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
            let ScriptSyntax::Decl { returns, .. }=syntax_tree.get_mut(last_func_syntax_ind).unwrap() else {panic!("");};
            *returns = return_items;
        } //end handle node/apply/template_decl returns

        //
        if !cur_work.in_a_use && !cur_work.exit && match &cur_element.element_type {  //enter
            ElementType::Node{..} if cur_work.depth==0 => true,
            ElementType::Stub{..} => true,
            _ => false,
        } {
            apply_calls_stk.push(Vec::new());
        }

        //stub decl
        // if !cur_work.in_a_use { //stub can only be in root, so no longer needed

        if let ElementType::Stub{name,..}=&cur_element.element_type {
            if !cur_work.exit { //enter
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
            } else { //exit
                syntax_stk.pop().unwrap();
            }
        }
        // }
        //stub decl end

        //
        if !cur_work.in_a_use //ie not in a template/apply use element
            // // && (cur_element.has_own_script||cur_element.has_template_use_script||cur_element.has_apply_decl_script)

            && (!only_script || cur_element.has_script)
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

                        // let has_self_param=cur_element.has_own_script || cur_element.has_template_use_script;
                        // let has_self_param=true;
                        let has_self_param=!only_script || cur_element.has_self_script;

                        // let params= cur_element.calcd_node_params.iter().map(|&param_element_ind|ScriptSyntaxNode(param_element_ind)).collect::<Vec<_>>();

                        let params= cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                            let param_element=&elements[param_element_ind];
                            // let param_element2=param_element.calcd_from_element_ind.map(|from_element_ind|&elements[from_element_ind]).unwrap_or(param_element);
                            (!only_script || param_element.has_self_script).then_some(ScriptSyntaxNode(param_element_ind))
                        }).collect::<Vec<_>>();

                        syntax_tree.push(ScriptSyntax::Decl {
                            name: ScriptSyntaxNodeOrApplyOrTemplate::Node(cur_work.element_ind),
                            params,
                            children: Vec::new(),
                            returns: Vec::new(),
                            has_self: has_self_param,
                            has_ret:true,
                        });
                    } else { //exit
                        let parent_element=elements.get(cur_work.parent.unwrap()).unwrap();

                        syntax_stk.pop().unwrap();
                        let new_syntax_ind=syntax_tree.len();
                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                        let in_func=if let ElementType::Stub{..}=&parent_element.element_type {false}else{cur_work.depth!=1};
                        let has_self_param=!only_script || cur_element.has_self_script;
                        let mut params=Vec::new();

                        // if cur_element.has_own_script || cur_element.has_template_use_script
                        if has_self_param
                        {
                            params.push(ScriptSyntaxNode(cur_work.element_ind));
                        }

                        // params.extend(cur_element.calcd_node_params.iter().map(|&param_element_ind|ScriptSyntaxNode(param_element_ind)));

                        params.extend(cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                            let param_element=&elements[param_element_ind];
                            // let param_element2=param_element.calcd_from_element_ind.map(|from_element_ind|&elements[from_element_ind]).unwrap_or(param_element); //doesn't need?

                            (!only_script || param_element.has_self_script).then_some(ScriptSyntaxNode(param_element_ind))
                        }));

                        syntax_tree.push(ScriptSyntax::CallNode {
                            // ret:cur_element.has_apply_decl_script,
                            has_ret:true,
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
                        // let params=cur_element.calcd_node_params.iter().map(|&param_element_ind|ScriptSyntaxNode(param_element_ind)).collect::<Vec<_>>();
                        let params=cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                            let param_element=&elements[param_element_ind];
                            // let param_element2=param_element.calcd_from_element_ind.map(|from_element_ind|&elements[from_element_ind]).unwrap_or(param_element);
                            (!only_script || param_element.has_self_script).then_some(ScriptSyntaxNode(param_element_ind))
                        }).collect::<Vec<_>>();

                        //

                        // let has_self_param=cur_element.has_own_script || cur_element.has_template_use_script;
                        // let has_self_param=true;
                        let has_self_param=!only_script || cur_element.has_self_script;

                        let new_syntax_ind=syntax_tree.len();

                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                        syntax_stk.push(new_syntax_ind);
                        syntax_tree.push(ScriptSyntax::Decl {
                            // decl: ScriptSyntaxDecl::Apply,
                            name: ScriptSyntaxNodeOrApplyOrTemplate::Apply(cur_work.element_ind),
                            params,
                            children: Vec::new(),
                            returns: Vec::new(),
                            has_self: has_self_param ,
                            has_ret:true,
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
                        // let params=cur_element.calcd_node_params.iter().map(|&param_element_ind|ScriptSyntaxNode(param_element_ind)).collect::<Vec<_>>();
                        let params=cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                            let param_element=&elements[param_element_ind];
                            // let param_element2=param_element.calcd_from_element_ind.map(|from_element_ind|&elements[from_element_ind]).unwrap_or(param_element);
                            (!only_script || param_element.has_self_script).then_some(ScriptSyntaxNode(param_element_ind))
                        }).collect::<Vec<_>>();


                        // let has_self_param=cur_element.has_own_script || cur_element.has_template_use_script;
                        // let has_self_param=true;
                        let has_self_param=!only_script || cur_element.has_self_script;

                        let new_syntax_ind=syntax_tree.len();
                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                        syntax_stk.push(new_syntax_ind);

                        syntax_tree.push(ScriptSyntax::Decl {
                            name: ScriptSyntaxNodeOrApplyOrTemplate::Template(cur_work.element_ind),
                            params,
                            children: Vec::new(),
                            returns: Vec::new(),
                            has_self: has_self_param,
                            has_ret:true,
                        });
                    } else { //exit
		                syntax_stk.pop().unwrap();
                    }
                }
                &ElementType::TemplateUse { template_decl_element_ind } => {
                    if !cur_work.exit { //enter
                        let new_syntax_ind=syntax_tree.len();
                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                        let template_decl_element=elements.get(template_decl_element_ind).unwrap();
                        // let ret=cur_element.has_apply_decl_script.then_some(ScriptSyntaxTemplateUse(cur_work.element_ind));
                        // let ret=Some(ScriptSyntaxTemplateUse(cur_work.element_ind));
                        let ret=ScriptSyntaxTemplateUse(cur_work.element_ind);
                        // let params=cur_element.calcd_node_params.iter().map(|&param_element_ind|ScriptSyntaxNode(param_element_ind)).collect();

                        if cur_element.calcd_node_params.len()!=template_decl_element.calcd_node_params.len() {
                            panic!("");
                        }

                        let params=cur_element.calcd_node_params.iter().zip(template_decl_element.calcd_node_params.iter()).filter_map(|(&param_element_ind,&decl_param_element_ind)|{
                            let param_element=&elements[decl_param_element_ind];
                            // let param_element2=param_element.calcd_from_element_ind.map(|from_element_ind|&elements[from_element_ind]).unwrap_or(param_element);
                            (!only_script || param_element.has_self_script).then_some(ScriptSyntaxNode(param_element_ind))
                        }).collect();


                        // let has_self=template_decl_element.has_own_script || template_decl_element.has_template_use_script;
                        // let self_param=true;
                        let self_param=!only_script || template_decl_element.has_self_script;

                        println!("params is {params:?}, self is {self_param}");
                        syntax_tree.push(ScriptSyntax::CallTemplate {
                            ret,
                            func: ScriptSyntaxTemplateDecl(template_decl_element_ind),
                            params,
                            has_self: self_param,
                            // use_self,
                            has_ret:true,
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

            syntax_tree.push(ScriptSyntax::CallStubCreate {
                is_root,
                // has_script,
                stub: cur_work.element_ind,
            } );
        } //end call stub
    } //end while

    //
    syntax_tree
}

// pub fn optimise_script_syntax_tree(elements:&Vec<Element>,syntax_tree:&mut Vec<ScriptSyntax>)  {
//     let mut element_map: HashMap<usize, usize> = HashMap::new(); //[element_ind]=syntax_ind
//     let mut has_src: HashSet<usize>=HashSet::new();
//     // let mut parent_map: HashMap<usize, usize> = HashMap::new(); //[element_ind]=syntax_ind

//     //
//     {
//         let mut stk=vec![(0,0)];

//         while let Some((cur_ind,parent))=stk.pop() {
//             let cur=syntax_tree.get(cur_ind).unwrap();
//             stk.extend(cur.get_children().map(|x|x.iter()).unwrap_or_default().map(|&child|(child,cur_ind)));

//             if let ScriptSyntax::Insert { .. } = cur {
//                 has_src.insert(parent);
//             }

//             if let Some(element_ind)=cur.element_ind() {
//                 element_map.insert(element_ind, cur_ind);
//             }
//         }
//     }

//     //
//     //need to check which ret vals are used
//     //in decls, check which params are used, then on their callers, remove unused
//     //  check most nested ones first?

//     //
//     struct Work {
//         syntax_ind:usize,
//     }
//     let mut work_stk=vec![Work{ syntax_ind: 0}];

//     while let Some(cur_work)=work_stk.pop() {
//         let cur_syntax=syntax_tree.get(cur_work.syntax_ind).unwrap();
//         work_stk.extend(cur_syntax.get_children().map(|x|x.iter()).unwrap_or_default().map(|&child_ind|Work{ syntax_ind: child_ind }));

//         match cur_syntax {
//             ScriptSyntax::Root { children } => {

//             }
//             ScriptSyntax::Insert { path, loc, insert } => {

//             }
//             ScriptSyntax::Decl { name, params, children, returns, has_self: self_param, has_ret } => {

//             }
//             ScriptSyntax::Stub { name, children } => {

//             }
//             ScriptSyntax::CallStubCreate { is_root, stub } => {

//             }
//             ScriptSyntax::CallTemplate { ret, func, params, has_self, has_ret } => {

//             }
//             ScriptSyntax::CallApply { ret, func_froms, func_apply, params, self_node, has_ret, has_self } => {

//             }
//             ScriptSyntax::CallNode { has_ret, in_func, func, params } => {

//             }
//         }
//     }
// }

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
            ScriptSyntax::CallStubCreate { is_root, stub, .. } => {
                println!("{indent}call_stub {stub}, is_root={is_root}");
            }
            ScriptSyntax::CallTemplate { ret, func, params, has_self,has_ret  } => {
                let ret=has_ret.then_some(ret);
                println!("{indent}call_template {func:?}({}{params:?}) => {ret:?}, ",has_self.then(||"self,").unwrap_or_default());
            }
            ScriptSyntax::CallApply { ret, func_froms, func_apply, params, self_node,has_ret, has_self } => {
                let ret=has_ret.then_some(ret);
                println!("{indent}call_apply {func_froms:?} {func_apply:?}({self_node:?},{params:?}) => {ret:?}");
            }
            ScriptSyntax::CallNode { in_func, func, params, has_ret,   } => {
                println!("{indent}call_node {func:?}({params:?}), in_func={in_func}, ret={has_ret}, ");
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
