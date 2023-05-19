use swc_core::ecma::ast::{ CallExpr, Program };
use swc_core::ecma::visit::{ as_folder, FoldWith, VisitMut };
use swc_core::ecma::transforms::testing::test;
use swc_core::plugin::{ plugin_transform, proxies::TransformPluginProgramMetadata };

pub struct ReactThreeVisitor;

impl VisitMut for ReactThreeVisitor {
    fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {}
}

#[plugin_transform]
pub fn transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
  program.fold_with(&mut as_folder(ReactThreeVisitor))
}

test!(
  Default::default(),
  |_| as_folder(ReactThreeVisitor),
  namespace_extend,
  r#"
    import * as THREE from "three";
    import { extend } from "@react-three/fiber";

    extend(THREE);
  "#,
  r#"
    import * as THREE from "three";
    import { extend } from "@react-three/fiber";

    extend(THREE);
  "#
);