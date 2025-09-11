
use super::vals::*;

pub fn calc_node_apply_ignores(elements:&mut Vec<Element>) { //not currently used?

    let mut work_stk=vec![CalcNodeApplyIgnoresWork{ element_ind: 0, depth:0 }];

    let mut ancestor_stk: Vec<usize> = Vec::new(); //element_ind

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();
        work_stk.extend(cur_element.children.iter().rev().map(|&c|CalcNodeApplyIgnoresWork { element_ind: c,depth:cur_work.depth+1 }));
        ancestor_stk.truncate(cur_work.depth);

        /*
        apply x
            apply a #ignored
            apply y
                apply a #ignored
                apply z
                    apply b #used
                    node a b
        apply b
        node x
            node y
                node z
        -------------
        apply x
            node
                apply y
                    apply z
                        node a
                apply a #used
                node y
                    node z
        node x
        -------------
        apply x
            apply q
                apply a #used? yes
                apply y
                    apply a
                    apply z
                        node a
                node y
                    node z
        node x
            node q


        */

        /*
        apply x
            node y
        apply y #no?
        node x
            apply y #yes
        */
        //

        // if let ElementType::Node { .. } = &cur_element.element_type {

        //     let mut calcd_ignores = HashSet::new();

        //     if ancestor_stk.len()>0 {
        //         for ancestor_ind in (0.. ancestor_stk.len()-1).rev() { //ancestor_stk.len()-1 == grandparent
        //             let ancestor_element_ind=ancestor_stk.get(ancestor_ind).cloned().unwrap();
        //             let ancestor_element=elements.get(ancestor_element_ind).unwrap();

        //             if let //ElementType::Root|
        //                 ElementType::Node{..}|ElementType::TemplateDecl{..}
        //                 |ElementType::Stub { .. } //needed? probably since similar to a node, no ? yes stub can contain nodes which can contain applies
        //                 // |ElementType::ApplyUse { .. }
        //                 // |ElementType::ApplyUse { .. }|ElementType::TemplateUse { .. } //these needed? don't think so, since when element tree created, these aren't filled, and working off applies which aren't evaluated
        //                 =&ancestor_element.element_type { //why? and what about apply_use? or stub, or template_use?
        //                 break;
        //             }
        //             println!("=== {:?} {:?}",ancestor_element.element_type,ancestor_element.applies);
        //             // break;

        //             // calcd_ignores.extend(ancestor_element.applies.iter().map(|&apply_element_ind|{
        //             //     // let apply_element=elements.get(apply_element_ind).unwrap();
        //             //     // let ElementType::Apply { .. }=apply_element.element_type else {panic!("");};
        //             //     // apply_decl_id
        //             //     apply_element_ind
        //             // }));
        //             calcd_ignores.extend(ancestor_element.applies.iter());
        //         }
        //     }

        //     //


        //     let cur_element=elements.get_mut(cur_work.element_ind).unwrap();
        //     let ElementType::Node { ignore_applies, .. } = &mut cur_element.element_type else {panic!("");};
        //     *ignore_applies=calcd_ignores;
        // }



        // let cur_element=elements.get(cur_work.element_ind).unwrap();
        // println!("= {} : {:?} : {:?}",cur_work.element_ind,ancestor_stk,cur_element.element_type);
        //
        ancestor_stk.push(cur_work.element_ind);
    }


    //
    // applies inside a template should only affect the template's descendants and not siblings
    // attrib set by apply/template are always overwritten
}
