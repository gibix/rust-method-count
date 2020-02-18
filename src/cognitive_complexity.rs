use std::collections::HashMap;
pub use syn::visit::{self, Visit};
use syn::{ExprForLoop, ExprIf, ExprLoop, ExprMatch, ImplItemMethod, ItemFn};

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

    fn visit_impl_item_method(&mut self, f: &ImplItemMethod) {
        let mut helper = CCHelper { cc: 1 };
        helper.visit_block(&f.block);

        debug!("{:?}", f.sig);
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
    fn visit_expr_loop(&mut self, node: &ExprLoop) {
        self.cc += 1;
        visit::visit_expr_loop(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &ExprForLoop) {
        self.cc += 1;
        visit::visit_expr_for_loop(self, node);
    }

    fn visit_expr_match(&mut self, e: &ExprMatch) {
        // walk_expr ?
        if e.arms.len() > 1 {
            self.cc += 1;
        }
        self.cc += e.arms.iter().filter(|arm| arm.guard.is_some()).count() as u64;
    }

    fn visit_expr_if(&mut self, node: &ExprIf) {
        self.cc += 1;
        visit::visit_expr_if(self, node);
    }
}

#[cfg(test)]
mod test {
    use crate::cognitive_complexity::*;

    fn test_cc_over_str(file: &str) -> HashMap<String, u64> {
        let ast = syn::parse_file(&file).unwrap();
        let mut complexity = CognitiveComplexity::new();
        complexity.visit_file(&ast);
        complexity.tree
    }

    #[test]
    fn cc_test() {
        static CC_TEST_FILE: &'static str = "
            pub fn bubble_sort_rust() {
                let mut arr: [i32; 4] = [4, 8, 3, 1];
                let mut new_len: usize;
                let mut len = arr.len();
                loop {
                    new_len = 0;
                    for i in 1..len {
                        if arr[i - 1] > arr[i] {
                            arr.swap(i - 1, i);
                            new_len = i;
                        }
                    }
                    if new_len == 0 {
                        break;
                    }
                    len = new_len;
                }
            }
        ";
        let mut check = HashMap::new();
        check.insert("bubble_sort_rust".to_string(), 5);
        assert_eq!(test_cc_over_str(CC_TEST_FILE), check);
    }
}
