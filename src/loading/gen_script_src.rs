
/*
todo
* swap dict usage with own type
*/
use super::vals::*;

pub fn gen_script_src(syntax_tree:&Vec<ScriptSyntax>) -> String {
    let mut stk=vec![(0,0,false)]; //ind,depth,exit
    let mut src=String::new();
    src+="var root, _stubs\n";

    while let Some((cur_ind,depth, exit))=stk.pop() {
        let indent="    ".repeat(if depth<=1{0}else{depth-1});
        let cur=syntax_tree.get(cur_ind).unwrap();
        match cur {
            ScriptSyntax::Root { .. } => {
            }
            ScriptSyntax::Insert { //path, loc,
                insert , ..} => {
                src+=&format!("{indent}{insert}\n");
            }
            ScriptSyntax::Stub { name, .. } if !exit => {
                src+=&format!("{indent}fn stub_{name} (self) {{\n");
                // let xx=children.iter().map(|&child_ind|{
                //     let y=syntax_tree.get(child_ind).unwrap();

                // });
                // src+=&format!("{indent}    var env [dict {xx}]\n");
            }
            ScriptSyntax::Stub { .. } => {
                src+=&format!("{indent}}}\n");
            }
            &ScriptSyntax::CallStubCreate { is_root, stub, .. } => {
                let parent = if is_root{"root"}else{"self"};
                src+=&format!("{indent}var _ns [call _stubs {stub} {parent}]\n");

                if is_root //&& cur.h
                {
                    src+=&format!("{indent}var env _ns.e0\n");
                }
            }
            ScriptSyntax::CallTemplate { ret, func, params, has_self,has_ret } => {
                // let mut params2=vec!["self".to_string()];
                let mut params2= Vec::new();

                if *has_self {
                    params2.push("self".to_string());
                }

                params2.extend(params.iter().map(|x|format!("_p{x}")));
                let params2=params2.join(" ");
                let c=format!("call _t{func} {params2}");
                // let x=if let Some(ret)=ret {
                //     format!("var _rt{ret} [{c}]")
                // } else {
                //     c
                // };
                let x=has_ret.then(||format!("var _rt{ret} [{c}]")).unwrap_or_else(||c);
                // // let self_info=if *use_self {format!(" #self")} else {};
                // let self_info=use_self.then(||" #self").unwrap_or_default();
                // src+=&format!("{indent}{x}{self_info}\n");
                src+=&format!("{indent}{x}\n");
            }
            ScriptSyntax::CallApply { ret, func_froms, func_apply, params, self_node,has_ret } => {
                let mut params2=Vec::new();
                params2.extend(params.iter().map(|x|format!("_ns.n{x}")));
                let params2=params2.join(" ");
                let mut func = Vec::new();

                if let Some((func_first,rest))=func_froms {
                    let first=match func_first {
                        ScriptSyntaxNodeOrApplyUse::Node(e) => format!("n{e}"),
                        ScriptSyntaxNodeOrApplyUse::ApplyUse(e) => format!("a{e}"),
                    };

                    func.push(format!("r{first}"));
                    func.extend(rest.iter().map(|t|format!("t{t}")));
                }

                func.push(format!("a{func_apply}"));
                let func=func.join(".");

                // let not_has_self=not_has_self.map(|self_element_ind|format!("#!n{self_element_ind}!#{}",(!params2.is_empty()).then(||" ").unwrap_or_default())).unwrap_or_default();
                // //#!{n12}!

                // let c=format!("call _{func} {not_has_self}{params2}");
                let c=format!("call _{func} {params2}");
                // let x=if let Some(ret)=ret {
                //     format!("var _ra{ret} [{c}]")
                // } else {
                //     c
                // };
                let x=has_ret.then(||format!("var _ra{ret} [{c}]")).unwrap_or_else(||c);

                //
                if params.get(0)==Some(self_node) {

                }
                let not_has_self=(params.get(0)!=Some(self_node)).then(||format!(" # <- n{self_node}")).unwrap_or_default();
                // let not_has_self=not_has_self.map(|self_element_ind|format!(" # <- n{self_element_ind}")).unwrap_or_default();
                src+=&format!("{indent}{x}{not_has_self}\n");
                // if let Some(not_has_self)=not_has_self {
                //     src+=&format!("{indent}#n{not_has_self}\n");
                // }
                // src+=&format!("{indent}{x}\n");
            }
            ScriptSyntax::CallNode { in_func, func, params, has_ret,  } => {

                let params=params.iter().map(|x|format!("_{}{x}",if *in_func{"p"}else{"ns.n"})).collect::<Vec<_>>().join(" ");

                let c=format!("call _n{func} {params}");
                let x=if *has_ret {
                    format!("var _rn{func} [{c}]")
                }else {
                    c
                };
                src+=&format!("{indent}{x}\n");
            }
            ScriptSyntax::Decl { name, params, has_self: self_param, .. }  if !exit => { //enter
                // let mut params2=vec!["self".to_string()];
                let mut params2 = Vec::new();

                if *self_param {
                    params2.push("self".to_string());
                }

                params2.extend(params.iter().map(|x|format!("_p{x}")));
                let params2=params2.join(" ");
                let name = match name {
                    ScriptSyntaxNodeOrApplyOrTemplate::Node(x) => format!("_n{x}"),
                    ScriptSyntaxNodeOrApplyOrTemplate::Apply(x) => format!("_a{x}"),
                    ScriptSyntaxNodeOrApplyOrTemplate::Template(x) => format!("_t{x}"),
                };



                src+=&format!("{indent}fn {name} ({params2}) {{\n");
                // src+=&format!("{indent}fn {name} ({params2}) {{{}\n",(*self_param).then(||));
            }
            ScriptSyntax::Decl { returns,has_ret, .. } => { //exit
                //if !returns.is_empty()
                if *has_ret
                {
                    let returns=returns.iter().map(|(k,v)|match (k,v) {
                        (Some(k),ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(v)) => format!("\"a{v}\" _rn{k}.a{v}"),
                        (Some(k),ScriptSyntaxTemplateUseOrApplyDecl::TemplateUse(v)) => format!("\"t{v}\" _rn{k}.t{v}"),
                        (None,ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(v)) => format!("\"a{v}\" _a{v}"),
                        (None,ScriptSyntaxTemplateUseOrApplyDecl::TemplateUse(v)) => format!("\"t{v}\" _rt{v}"),
                    }).map(|x|format!("({x})")).collect::<Vec<_>>().join(" ");
                    let spc=returns.is_empty().then_some("").unwrap_or(" ");
                    src+=&format!("{indent}    return [dict{spc}{returns}]\n");
                }

                src+=&format!("{indent}}}\n");
            }
        }

        if !exit {
            match cur {
                ScriptSyntax::Decl{..}|ScriptSyntax::Stub{..} => {
                    stk.push((cur_ind,depth,true));
                }
                _ => {}
            }

            if let Some(children)=cur.get_children() {
                stk.extend(children.iter().map(|&child_ind|(child_ind,depth+1,false)).rev());
            }
        }
    }

    src
}
