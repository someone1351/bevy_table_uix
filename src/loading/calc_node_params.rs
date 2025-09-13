

use super::vals::*;

#[derive(Clone)]
struct Work {
    element_ind:usize,
    exit:bool,
    parent:Option<usize>,
    in_template_or_apply_decl:bool,
}

pub fn calc_node_params(elements:&mut Vec<Element>) {
    //for apply/template/node decls params
    //are these params also used for apply use, template use, node init calls?
    //what about apply returns? not here? no
    //where are params not used in scripts filtered out? not here
    //node's own self isn't included in its params

    let mut work_stk=vec![Work { element_ind:0, exit:false, parent:None,in_template_or_apply_decl:false, }];

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();

        //on enter
        if !cur_work.exit {
            //add exit to stack
            work_stk.push(Work{exit:true, ..cur_work.clone()});

            //
            let in_template_or_apply_decl=if let ElementType::TemplateDecl{..}|ElementType::Apply{..}=
                &cur_element.element_type {true} else {cur_work.in_template_or_apply_decl};

            //add cur's children enters to stack
            work_stk.extend(cur_element.children.iter().rev().map(|&element_ind|Work {
                element_ind,
                exit:false,
                parent:Some(cur_work.element_ind),
                in_template_or_apply_decl,
            }));
        }

        //not root
        let Some(parent_element_ind)=cur_work.parent else { continue; };

        //
        if !cur_work.exit { //on enter
            //works by traversing expanded nodes
            //  getting nodes and storing them as params in their parent
            //  and then extending ancestor params with those
            //it works fine for unexpanded apply decls
            //  since they don't need to store params from an apply to its ancestors
            //but doesn't work for templates in templates,
            //  as it does need to store params from its ancestors
            //  and it is unexpanded
            //  can just take params from template use's decl, and extend cur template use's params
            //    problem is need to differentiate when inside a template decl or not


            match &cur_element.element_type {
                ElementType::Node { .. }  => { //only nodes added
                    //add self as param in parent
                    let parent_element=elements.get_mut(parent_element_ind).unwrap();
                    parent_element.calcd_node_params.insert(cur_work.element_ind);
                }
                ElementType::TemplateUse{ template_decl_element_ind, .. } if cur_work.in_template_or_apply_decl  => {
                    //add cur template use's decl params to cur's params
                    let decl_element=elements.get(*template_decl_element_ind).unwrap();
                    let params=decl_element.calcd_node_params.clone();

                    let cur_element=elements.get_mut(cur_work.element_ind).unwrap();
                    cur_element.calcd_node_params.extend(params);
                }
                _ =>  {}
            }
        } else { //on exit
            match &cur_element.element_type {
                ElementType::Stub { .. }|ElementType::Apply { .. }|ElementType::ApplyUse{..}|ElementType::TemplateDecl { .. } => { }
                _ => { //not root, stub, apply, apply_use, template_decl, should just be node, template_use?,
                    //add cur element's params to parent
                    let cur_nodes_params=cur_element.calcd_node_params.clone();
                    let parent_element=elements.get_mut(parent_element_ind).unwrap();
                    parent_element.calcd_node_params.extend(cur_nodes_params);
                }
            }
        }
    }
}

