
use std::collections::HashMap;


use super::vals::*;

pub fn calc_envs(elements:&mut Vec<Element>) {
    let mut work_stk=vec![0];

    while let Some(cur_element_ind)=work_stk.pop() {
        let cur_element=elements.get(cur_element_ind).unwrap();

        //
        match &cur_element.element_type {
            ElementType::Node {..}|ElementType::Apply {..}|ElementType::TemplateDecl {..}|ElementType::Stub {..}  => {
            }
            _ => {
                continue;
            }
        }

        //
        let mut env: HashMap<String, Vec<usize>>= HashMap::new();

        //
        for &child_element_ind in  cur_element.children.iter() {
            let child_element=elements.get(child_element_ind).unwrap();

            if let ElementType::Node { names, .. }=&child_element.element_type {
                for &name in names {
                    env.entry(name.to_string()).or_default().push(child_element_ind);
                }
            }
        }

        //
        let cur_element=elements.get_mut(cur_element_ind).unwrap();
        cur_element.env=env;
    }

}

