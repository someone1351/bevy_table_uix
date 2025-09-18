
use std::collections::{HashMap, HashSet};
use std::collections::BTreeSet;

use bevy_table_ui as table_ui;
use table_ui::*;

use super::super::script_vals::*;
use super::vals::*;


struct Thing { //whats this for? something to do with applies ...
    applies : Vec<(
        usize, //apply_element_ind
        usize, //from_element_ind
    )>, //apply_element_ind, from_apply_use_element_ind
    apply_after:usize,
    element_ind:usize,
}




// expand templates and applies, other stuff as well ..

pub fn expand_elements(elements:&mut Vec<Element>) {
    struct Work {
        element_ind:usize,
        from_applies:HashSet<usize>, //apply_element_ind
        in_template:Option<usize>, //template_use_id, used for attribs
        in_apply:Option<usize>, //apply_decl_id, used for attribs
        new_from_parent:Option<usize>, //element_ind
        thing_apply_after_offset:usize, //Option<usize>,

        node_depth:usize,
        thing_depth:usize,

        created_from:usize,

        in_template_or_apply_decl:bool,
    }

    let mut work_stk=vec![Work{
        element_ind:0,
        in_template:None,
        in_apply:None,
        from_applies:HashSet::new(),
        new_from_parent:None,
        thing_apply_after_offset:0,

        node_depth:0,
        thing_depth:0,

        created_from:0,

        in_template_or_apply_decl:false,
    }];

    let mut things: Vec<Thing>=vec![]; //stack of elements, (cur element and its ancestors?) + info
    let mut node_stk_attribs: Vec<HashMap<(&str,Option<UiAffectState>),(Option<usize>,Option<usize>,bool,AttribFunc,usize)>> = Vec::new(); //[node_depth][(name,state)]=(in_template,in_apply.in_node,func,element_ind)

    while let Some(cur_work)=work_stk.pop() {
        let mut the_new_element_ind:Option<usize> = None;

        //add element
        // replace element added via apply/template with new copy
        // cur_work.inside_apply_after
        if let Some(new_from_parent)=cur_work.new_from_parent {
            let cur_element=elements.get(cur_work.element_ind).unwrap();

            // if let ElementType::Node {..}
            //     |ElementType::TemplateUse{..} //added from apply or template decl?
            //     // |ElementType::Stub{..} //no longer required, now that stubs can only be decl at root, and therefore never will be within an apply
            //     //not needed, can just pass the same one around, actually is needed, need unique element_ind for attrib for debug purposes
            //     //  also attrib needs to be added to its new parent, but don't need a new copy for that
            //     //
            //     |ElementType::Attrib{..}=&cur_element.element_type
            // {
            // }
            let ok=match &cur_element.element_type {
                // ElementType::Node {..} => true,
                ElementType::Node {..}|ElementType::TemplateUse{..}=>true,
                ElementType::Attrib{..} if !cur_work.in_template_or_apply_decl => true, //not needed for in_template_or_apply_decl
                _ => false,
            };

            if ok {

                //
                let mut new_element=Element {
                    children: Vec::new(),
                    calcd_original:Some(cur_work.element_ind),
                    ..cur_element.clone()
                };

                new_element.calcd_from_element_ind=Some(cur_work.element_ind);

                //
                let new_element_ind=elements.len();
                let parent_element=elements.get_mut(new_from_parent).unwrap();
                parent_element.children.push(new_element_ind);
                elements.push(new_element);

                the_new_element_ind=Some(new_element_ind);

                //
                let new_element=elements.get_mut(new_element_ind).unwrap();
                new_element.calcd_created_from=cur_work.created_from;
            }
        } //end add element

        //
        // {
        //     let cur_element=elements.get(cur_work.element_ind).unwrap();
        //     match &cur_element.element_type {
        //         ElementType::Apply {..}|ElementType::TemplateDecl {..} => {
        //             work_stk.extend(cur_element.children.iter().rev().map(|&child_element_ind|Work{
        //                 element_ind: child_element_ind,
        //                 from_applies:HashSet::new(), //don't care
        //                 in_apply:None, //don't care
        //                 in_template:None, //don't care
        //                 new_from_parent: None, //isn't a new
        //                 thing_apply_after_offset: 0, //don't care
        //                 node_depth:0, //don't care
        //                 thing_depth:0, //don't care
        //                 created_from:cur_work.element_ind, //just its parent, not actually created from

        //                 in_template_or_apply_decl: true,
        //             }));
        //         }
        //         _ => {}
        //     }
        // }

        //add children to work for in_template_or_apply_decl
        if cur_work.in_template_or_apply_decl {
            let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);
            let cur_element=elements.get(cur_work.element_ind).unwrap();

            match &cur_element.element_type {
                ElementType::Node { .. } => {
                    println!("hmm n");
                    work_stk.extend(cur_element.children.iter().rev().map(|&child_element_ind|Work{
                        element_ind: child_element_ind,
                        from_applies:HashSet::new(), //don't care
                        in_apply:None, //don't care
                        in_template:None, //don't care
                        new_from_parent: the_new_element_ind, //so new element is created for child
                        thing_apply_after_offset: 0, //don't care
                        node_depth:0, //don't care
                        thing_depth:0, //don't care
                        created_from:cur_element_ind,

                        in_template_or_apply_decl: true,
                    }));
                }
                ElementType::TemplateUse { template_decl_element_ind,   .. } => {
                    println!("hmm t");
                    let template_decl_element=elements.get(*template_decl_element_ind).unwrap();

                    work_stk.extend(template_decl_element.children.iter().rev().map(|&child_element_ind|Work{
                        element_ind: child_element_ind,
                        from_applies:HashSet::new(), //don't care
                        in_apply:None, //don't care
                        in_template:None, //don't care
                        new_from_parent: Some(cur_element_ind), //so new element is created for child
                        thing_apply_after_offset: 0, //don't care
                        node_depth:0, //don't care
                        thing_depth:0, //don't care
                        created_from:cur_element_ind,

                        in_template_or_apply_decl: true,
                    }));
                }
                // ElementType::Stub { .. } => {} //not needed, since only allowed at root. even if otherwise, wouldn't need it anyway?
                _ => {}
            }
            continue;
        }

        //
        node_stk_attribs.truncate(cur_work.node_depth);
        things.truncate(cur_work.thing_depth);

        //
        let (node_depth, thing_depth,)= {
            let cur_element=elements.get(cur_work.element_ind).unwrap();

            let cur_is_node=if let ElementType::Node{..}=&cur_element.element_type {true}else{false};
            let cur_is_thing=if let ElementType::Node{..}|ElementType::TemplateUse{..}|ElementType::Stub{..} =&cur_element.element_type {true}else{false};

            (
                cur_work.node_depth+if cur_is_node{1}else{0},
                cur_work.thing_depth+if cur_is_thing{1}else{0},
            )
        };

        //node attribs
        {
            let cur_element=elements.get(cur_work.element_ind).unwrap();

            if let ElementType::Node { .. } = &cur_element.element_type {
                node_stk_attribs.push(HashMap::new());
            }
        }

        //attrib
        {
            let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind); //for attribs added via applies
            let cur_element=elements.get_mut(cur_element_ind).unwrap();

            if let ElementType::Attrib { name,in_node, on_state, func, calcd,.. } = &mut cur_element.element_type {
                let in_node=*in_node;
                let on_state=*on_state;

                let node_attribs=node_stk_attribs.last_mut().unwrap();

                let prev=node_attribs.get(&(*name,on_state.clone()));
                let prev_element_ind=prev.map(|x|x.4);
                let (prev_in_template,prev_in_apply,prev_in_node, )=prev.map(|x|(x.0,x.1,x.2,)).unwrap_or_default();

                let in_apply=cur_work.in_apply;
                let in_template=cur_work.in_template;

                let ok = in_node || !prev_in_node || (in_apply!=prev_in_apply || prev_in_template!=in_template);
                let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);

                calcd.in_template=in_template;
                calcd.in_apply=in_apply;
                calcd.ok=ok;
                calcd.used=ok;

                if ok {
                    node_attribs.insert((name,on_state), (in_template,in_apply,in_node,func.clone(),cur_element_ind));

                    if let Some(prev_element_ind)=prev_element_ind{
                        let ElementType::Attrib { calcd,.. } = &mut elements.get_mut(prev_element_ind).unwrap().element_type else {
                            panic!("");
                        };

                        calcd.used=false;
                    }
                }
            }
        } //attribs end

        //
        // if let Some(cur_element_ind)=the_new_element_ind {
        //     let cur_element=elements.get_mut(cur_element_ind).unwrap();
        //     cur_element.calcd_created_from=cur_work.created_from;
        // }

        //
        let mut new_applies: Vec<(usize, usize, )> = Vec::new(); //apply_element_ind,from_element_ind
        //get template applies

        //applies
        {
            let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);

            //should use cur_element_ind?
            //  for cur_element.apply_after? since it's modified? that apply_after is only modified in thing,
            //    which is because the applies are all added to it,
            //    so shouldn't apply to here
            let cur_element=elements.get(cur_work.element_ind).unwrap();

            // let cur_element_depth=cur_element.depth;
            //should go depth first, instead of breadth for get new applies, but how would handle apply_after index?
            //    would need to increment it by the amount added when before after_apply,
            //    in thing have a count of the number added, which can be added to element.apply_after

            match cur_element.element_type.clone() {
                ElementType::TemplateUse { template_decl_element_ind, .. } => {
                    let template_decl_element=elements.get(template_decl_element_ind).unwrap();
                    new_applies.extend(template_decl_element.applies.iter().map(|&apply_element_ind|( apply_element_ind, cur_element_ind, )));
                }
                ElementType::Stub { .. } => {
                    new_applies.extend(cur_element.applies.iter().map(|&apply_element_ind|( apply_element_ind, cur_element_ind, )));
                }
                ElementType::Node { names, ignore_applies, .. } => {
                    let last_stub_thing_ind=things.iter().enumerate().rev().find_map(|(i,t)|{
                        if let ElementType::Stub{..}=&elements.get(t.element_ind).unwrap().element_type {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    //
                    let mut before_applies: Vec<(usize, usize)> = Vec::new(); //(apply_element_ind,from_element_ind)
                    let mut after_applies: Vec<(usize, usize)> = Vec::new();

                    //get before applies
                    for (thing_ind,thing) in things.iter().enumerate() {
                        let apply_after=things.get(thing_ind+1).map(|x|x.apply_after).unwrap_or(cur_element.apply_after);

                        for (thing_apply_ind,&(apply_element_ind,from)) in thing.applies.iter().enumerate() {
                            // let apply_element_ind=*apply_element_ind;
                            let apply_element=elements.get(apply_element_ind).unwrap();
                            let ElementType::Apply { name:apply_name, .. }=&apply_element.element_type else {panic!("")};

                            if !cur_work.from_applies.contains(&apply_element_ind) //prevent recursion //apply_decl_id
                                && thing_apply_ind<apply_after //is before
                                && names.contains(apply_name)
                            {
                                before_applies.push((apply_element_ind,from)); //need to add owner of the apply?
                            }
                        }
                    }

                    //get after applies
                    for (thing_ind,thing) in things.iter().enumerate().rev() {
                        //don't allow nodes in stubs to use applies that come after the stub
                        //  except if node's first stub ancestor is also parent of the apply
                        if let Some(last_stub_thing_ind)=last_stub_thing_ind { //ancestor or cur element stub,
                            if thing_ind<last_stub_thing_ind { //swapped <=
                                continue;
                            }
                        }

                        //
                        let apply_after=things.get(thing_ind+1) //why +1 ? a descendant?
                            .map(|x|x.apply_after).unwrap_or(cur_element.apply_after);

                        for (thing_apply_ind,&(apply_element_ind,from)) in thing.applies.iter().enumerate() {
                            // let apply_element_ind=*apply_element_ind;
                            let apply_element=elements.get(apply_element_ind).unwrap();
                            let ElementType::Apply { name:apply_name, .. }=&apply_element.element_type else {panic!("")};

                            if !cur_work.from_applies.contains(&apply_element_ind) //prevent recursion //apply_decl_id
                                && thing_apply_ind>=apply_after //is after //what does thing_ind have to do with apply_after? oh thing_apply_ind
                                && names.contains(apply_name)
                            {
                                after_applies.push((apply_element_ind,from));
                            }
                        }
                    }

                    //apply the applies on cur
                    //  should add ApplyUse, with children added to its element, filter_map_children, removing Apply
                    // let mut the_apply_after=cur_element.applies.len();
                    let mut the_apply_after_count=0;

                    //
                    let mut apply_use_element_inds: Vec<Option<usize>>=Vec::new();

                    //add apply_use elements
                    {
                        let parent_applies_len=cur_element.applies.len();

                        for &(apply_element_ind,from) in before_applies.iter().chain(after_applies.iter()) {
                            if ignore_applies.contains(&apply_element_ind) { //apply_decl_id
                                apply_use_element_inds.push(None);
                                continue;
                            }

                            let apply_use_element_ind=elements.len();
                            apply_use_element_inds.push(Some(apply_use_element_ind));

                            elements.get_mut(cur_element_ind).unwrap().children.push(apply_use_element_ind);  //the cur_work.element_ind was wrong,

                            elements.push(Element {
                                element_type:ElementType::ApplyUse { apply_decl_element_ind: apply_element_ind,} ,
                                children: Vec::new(),
                                applies: Vec::new(),
                                apply_after:parent_applies_len, //new_applies.len(), //wrong? since this refers to parent.applies and not thing.applies, so should be parent.applies.len()
                                calcd_from_element_ind: None,
                                calcd_node_params:BTreeSet::new(),
                                calcd_env_params: BTreeSet::new(),
                                calcd_created_from:from,
                                has_script:false,
                                //has_apply_decl_script:false,
                                has_own_script:false,
                                // has_template_use_script:false,
                                calcd_original:None,
                                env: HashMap::new(),
                            });
                        }
                    }

                    //
                    for (i,&(apply_element_ind,_)) in (after_applies.iter().rev().chain(before_applies.iter().rev()) ).enumerate() {
                        //apply_use
                        let Some(apply_use_element_ind)=apply_use_element_inds[apply_use_element_inds.len()-i-1] else {
                            continue;
                        };

                        //
                        let apply_element=elements.get(apply_element_ind).unwrap();
                        let ElementType::Apply{owner_apply_decl_id:parent_owner_apply_decl_id,..}=apply_element.element_type else {panic!("");};

                        let the_apply_after=the_apply_after_count;
                        the_apply_after_count+=apply_element.applies.len();

                        // if ignore_applies.contains(&apply_decl_id) {
                        //     continue;
                        // }

                        let mut from_applies = cur_work.from_applies.clone();
                        from_applies.insert(apply_element_ind); //apply_decl_id

                        //what happens with apply children
                        // applies are added to thing with new_applies

                        //
                        work_stk.extend(apply_element.children.iter().rev().map(|&child_element_ind|{
                            let child_element=elements.get(child_element_ind).unwrap();

                            let in_apply=if let ElementType::Attrib{..}|ElementType::TemplateUse{..}=child_element.element_type.clone(){
                                parent_owner_apply_decl_id //use cur apply's parent
                            }else{ //use cur apply
                                // Some(apply_decl_id)
                                Some(apply_element_ind)
                            };

                            let in_template=if let ElementType::Attrib{in_template,..}=child_element.element_type.clone(){
                                in_template
                            } else {
                                cur_work.in_template //why this?
                            };

                            Work{
                                element_ind: child_element_ind,
                                in_template,
                                in_apply,
                                from_applies:from_applies.clone(),
                                new_from_parent:Some(apply_use_element_ind),
                                thing_apply_after_offset: the_apply_after,
                                node_depth,
                                thing_depth,
                                created_from:apply_use_element_ind,

                                in_template_or_apply_decl: false, //this part is only done for elements not in template/apply decl
                            }
                        }));
                    }

                    //get applies for new thing
                    //  add cur_element.applies then before+after applies
                    //  could do before_applies, cur_applies, after_applies

                    for (i,element_ind) in
                        before_applies.iter().enumerate().map(|(i,&x)|(Some(i),x.0))
                        .chain([(None,cur_work.element_ind)])
                        .chain(after_applies.iter().enumerate().map(|(i,&x)|(Some(i+before_applies.len()),x.0)))
                    {
                        let element=elements.get(element_ind).unwrap();

                        let from=if let Some(i)=i {
                            let Some(apply_use_element_ind)=apply_use_element_inds[i] else {
                                continue;
                            };
                            apply_use_element_ind
                        } else {
                            cur_element_ind
                        };

                        new_applies.extend(element.applies.iter().map(|&apply_element_ind|(apply_element_ind,from)));
                    }
                }
                _ => {
                }
            }
        } //applies end

        //push children (to work)
        {
            let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);
            let cur_element=elements.get(cur_work.element_ind).unwrap();

            match &cur_element.element_type {
                ElementType::Node { .. } => {
                    work_stk.extend(cur_element.children.iter().rev().filter_map(|&child_element_ind|{
                        let child_element=elements.get(child_element_ind).unwrap();
                        if let ElementType::ApplyUse { .. } =&child_element.element_type {
                            None
                        } else {
                            let w=Work{
                                element_ind: child_element_ind,
                                from_applies:cur_work.from_applies.clone(),
                                in_apply:cur_work.in_apply,
                                in_template:cur_work.in_template,
                                new_from_parent: the_new_element_ind, //so new element is created for child
                                thing_apply_after_offset: 0,
                                node_depth,
                                thing_depth,
                                created_from:cur_element_ind,

                                in_template_or_apply_decl: false,
                            };
                            Some(w)
                        }
                    }));
                }
                ElementType::Stub { .. } => {
                    work_stk.extend(cur_element.children.iter().rev().map(|&child_element_ind|Work{
                        element_ind: child_element_ind,
                        from_applies:cur_work.from_applies.clone(),
                        in_apply:cur_work.in_apply,
                        in_template:cur_work.in_template,
                        new_from_parent: the_new_element_ind, //so new element is created for child
                        thing_apply_after_offset: 0,
                        node_depth,
                        thing_depth,
                        created_from:cur_element_ind,

                        in_template_or_apply_decl: false,
                    }));
                }
                ElementType::TemplateUse { template_decl_element_ind,   .. } => {
                    let template_decl_element=elements.get(*template_decl_element_ind).unwrap();

                    work_stk.extend(template_decl_element.children.iter().rev().map(|&child_element_ind|{
                        let child_element=elements.get(child_element_ind).unwrap();
                        let in_template=if let ElementType::Attrib{..}=&child_element.element_type {
                            cur_work.in_template
                        }else{
                            // Some(*template_use_id)
                            Some(cur_element_ind)
                        };

                        Work{
                            element_ind: child_element_ind,
                            from_applies:cur_work.from_applies.clone(),
                            in_apply:cur_work.in_apply,
                            in_template,
                            // new_from_parent: if the_new_element_ind.is_some(){the_new_element_ind}else{Some(cur_work.element_ind)}, //so new element is created for child
                            new_from_parent: Some(the_new_element_ind.unwrap_or(cur_work.element_ind)), //so new element is created for child
                            thing_apply_after_offset: 0,
                            node_depth,
                            thing_depth,
                            created_from:cur_element_ind,

                            in_template_or_apply_decl: false,
                        }
                    }));
                }
                ElementType::Apply {..}|ElementType::TemplateDecl {..} => {
                    work_stk.extend(cur_element.children.iter().rev().map(|&child_element_ind|Work{
                        element_ind: child_element_ind,
                        from_applies:HashSet::new(), //don't care
                        in_apply:None, //don't care
                        in_template:None, //don't care
                        new_from_parent: None, //isn't a new
                        thing_apply_after_offset: 0, //don't care
                        node_depth:0, //don't care
                        thing_depth:0, //don't care
                        created_from:cur_work.element_ind, //just its parent, not actually created from

                        in_template_or_apply_decl: true,
                    }));
                }
                // ElementType::ApplyUse { apply_decl_element_ind } => {
                // }
                _=> {}
            }
        } //push children end

        //push thing
        // if enter
        {
            let cur_element=elements.get(cur_work.element_ind).unwrap();
            let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);

            match &cur_element.element_type {
                ElementType::Node {  ..  } => {
                    things.push(Thing {
                        applies: new_applies,
                        apply_after: cur_element.apply_after+cur_work.thing_apply_after_offset, //for elements added by apply
                        element_ind:cur_element_ind,
                    });
                }
                ElementType::Stub { .. } => {
                    things.push(Thing {
                        // applies: Vec::new(), //new_applies, //doesn't use new applies? should be empty anyway
                        applies: new_applies,
                        apply_after: cur_element.apply_after+cur_work.thing_apply_after_offset, //for elements added by apply
                        element_ind:cur_element_ind,
                    });
                }
                ElementType::TemplateUse { .. } => {
                    things.push(Thing {
                        applies: new_applies,
                        apply_after: cur_element.apply_after+cur_work.thing_apply_after_offset, //for elements added by apply
                        element_ind:cur_element_ind,
                    });
                }
                _=> {}
            }
        } //push thing end
    } //end while
}

//need to expand template uses in both template decls and apply decls
//  which are only needed for getting node params, and probably also env params
//    do I even need to do this? in node params calc, could do own extending ie pushing on to stk.
//      No, need actualy element inds for params. Could just use ghost element inds ... Easier to debug if don't though.
//  could probably put the code in the func above? no since don't want it messing with attribs/applies etc, just expanding template_uses, adding nodes and further template uses
//don't need the same thing for applies as that is done via the before/after applies above

pub fn expand_template_apply_decl_elements(elements:&mut Vec<Element>) {
    // struct Work {
    //     element_ind:usize,
    //     in_template_or_apply_decl:bool,
    // }

    // let mut work_stk=vec![Work{
    //     element_ind:0,
    //     in_template_or_apply_decl:false,

    // }];

    // while let Some(cur_work)=work_stk.pop() {
    //     let cur_element=elements.get(cur_work.element_ind).unwrap();

    //     //
    //     // let is_template_or_apply_decl=match &cur_element.element_type{
    //     //     ElementType::Apply{..}|ElementType::TemplateDecl{..}=>true,
    //     //     _=>cur_work.in_template_or_apply_decl,
    //     // };

    //     //
    //     match &cur_element.element_type {
    //         ElementType::Apply{..}|ElementType::TemplateDecl{..} => {
    //             work_stk.extend(cur_element.children.iter().map(|&child_element_ind|Work {
    //                 element_ind: child_element_ind,
    //                 in_template_or_apply_decl:true,
    //             }));
    //         }
    //         &ElementType::TemplateUse{ template_decl_element_ind } if cur_work.in_template_or_apply_decl => {
    //             let decl_element=elements.get(template_decl_element_ind).unwrap();
    //             work_stk.extend(decl_element.children.iter().map(|&child_element_ind|Work {
    //                 element_ind: child_element_ind,
    //                 in_template_or_apply_decl:cur_work.in_template_or_apply_decl,
    //             }));
    //         }
    //         ElementType::Node{..} => {
    //             work_stk.extend(cur_element.children.iter().map(|&child_element_ind|Work {
    //                 element_ind: child_element_ind,
    //                 in_template_or_apply_decl:cur_work.in_template_or_apply_decl,
    //             }));
    //         }

    //         _ => {}
    //     }

    //     //
    //     if !cur_work.in_template_or_apply_decl {
    //         continue;
    //     }

    //     //
    //     if let ElementType::TemplateUse{ template_decl_element_ind }=cur_element.element_type {
    //         let decl_element=elements.get(template_decl_element_ind).unwrap();

    //     }
    // }
}

            // ElementType::Node { .. } => todo!(),
            // ElementType::Script {.. } => todo!(),
            // ElementType::Apply { .. } => todo!(),
            // ElementType::TemplateDecl { .. } => todo!(),
            // ElementType::TemplateUse {.. } => todo!(),
            // ElementType::Stub { .. } => todo!(),

//starts with template or apply decl element
// pub fn do_template_apply_decl_elements(elements:&mut Vec<Element>,start_element_ind:usize,) {
//     struct Work {
//         element_ind:usize,
//         add:bool,
//     }

//     let mut work_stk=vec![Work{
//         element_ind:start_element_ind,
//         add:false,

//     }];

//     while let Some(cur_work)=work_stk.pop() {
//         let cur_element=elements.get(cur_work.element_ind).unwrap();

//        match &cur_element.element_type {
//             ElementType::Apply{..}|ElementType::TemplateDecl{..} => {
//                 work_stk.extend(cur_element.children.iter().map(|&child_element_ind|Work {
//                     element_ind: child_element_ind,
//                     add:false,
//                 }));
//             }
//             &ElementType::TemplateUse{ template_decl_element_ind } => {
//                 let decl_element=elements.get(template_decl_element_ind).unwrap();
//                 work_stk.extend(decl_element.children.iter().map(|&child_element_ind|Work {
//                     element_ind: child_element_ind,
//                     add:true,
//                 }));
//             }
//             ElementType::Node{..} => {
//                 work_stk.extend(cur_element.children.iter().map(|&child_element_ind|Work {
//                     element_ind: child_element_ind,
//                     add:cur_work.add,
//                 }));
//             }

//             _ => {}
//         }

//     }
// }