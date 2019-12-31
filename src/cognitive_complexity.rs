use std::collections::HashMap;
pub use syn::visit::{self, Visit};
use syn::{ExprMatch, ItemFn, ExprIf};

#[derive(Default)]
pub struct CognitiveComplexity {
    pub tree: HashMap<String, u64>,
}

impl CognitiveComplexity {
    pub fn new() -> Self {
        CognitiveComplexity::default()
    }
}

impl<'ast> Visit<'ast> for CognitiveComplexity {
    fn visit_item_fn(&mut self, f: &ItemFn) {
        let mut helper = CCHelper { cc: 1 };
        helper.visit_block(&f.block);

        self.tree
            .entry(f.sig.ident.to_string())
            .and_modify(|v| *v += helper.cc)
            .or_insert(helper.cc);
    }
}

struct CCHelper {
    cc: u64,
}

impl<'tcx> Visit<'tcx> for CCHelper {
    fn visit_expr_match(&mut self, e: &ExprMatch) {
        // walk_expr(self, e);
        if e.arms.len() > 1 {
            self.cc += 1;
        }
        self.cc += e.arms.iter().filter(|arm| arm.guard.is_some()).count() as u64;
    }

    fn visit_expr_if(&mut self, i: &ExprIf) {
        // walk_expr(self, e);
        if i.else_branch.is_some() {
            self.cc +=1;
        }
    }
}
