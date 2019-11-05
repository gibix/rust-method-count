extern crate syn;
#[macro_use]
extern crate log;

use std::collections::BTreeMap;

pub fn amf_count(syntax_tree: syn::File) {
    // global type counter
    let mut amfs = BTreeMap::new();

    // filter out all the impl blocks
    for impl_item in syntax_tree.items.iter().filter_map(|node| match node {
        syn::Item::Impl(item_impl) => Some(item_impl),
        _ => None,
    }) {
        // debug!("Self Type: {:?}", impl_item.self_ty);

        // print attributes
        // for attr in impl_item.attrs.iter() {

        // }

        // count associated methods into an impl block
        let mut amf_per_impl = 0;

        for impl_item_methods in impl_item.items.iter() {
            if let syn::ImplItem::Method(m) = impl_item_methods {
                // log!("Found method {:?}", m.signature)
                amf_per_impl += 1;
            };
        }

        amfs.insert("test", amf_per_impl);
    }
}
