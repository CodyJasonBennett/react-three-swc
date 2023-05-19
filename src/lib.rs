use swc_core::common::util::take::Take;
use swc_core::ecma::ast::{ CallExpr, Expr, Program };
use swc_core::ecma::visit::{ VisitMut, FoldWith, as_folder };
use swc_core::ecma::transforms::testing::test;
use swc_core::plugin::{ plugin_transform, proxies::TransformPluginProgramMetadata };

pub struct ReactThreeVisitor;

impl VisitMut for ReactThreeVisitor {
  fn visit_mut_call_expr(&mut self, node: &mut CallExpr) {
    // Invalidate node
    node.take();
  }

  // TODO: not called for CallExpr
  fn visit_mut_exprs(&mut self, nodes: &mut Vec<Box<Expr>>) {
    // Remove all invalid nodes
    nodes.retain(|node| !node.is_invalid());
  }
}

#[plugin_transform]
pub fn transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
  program.fold_with(&mut as_folder(ReactThreeVisitor))
}

// removes namespaced extend calls
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

    // TODO: take() assigns a dummy callee which is super() here.
    // Is this a bug in SWC? This isn't valid at the top-level.
    super();
  "#
);