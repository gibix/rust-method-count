/// AMF - Associated Methods and Functions
extern crate syn;
#[macro_use]
extern crate log;

use std::collections::BTreeMap;
use syn::{Ident, ImplItem, Item, Type};

pub fn amf_count(syntax_tree: syn::File) -> BTreeMap<Ident, u32> {
    // global type counter
    let mut amfs = BTreeMap::new();

    // filter out all the impl blocks
    for impl_item in syntax_tree.items.iter().filter_map(|node| match node {
        Item::Impl(item_impl) => Some(item_impl),
        _ => None,
    }) {
        if let Type::Path(ref typepath) = *impl_item.self_ty {
            let ident = &typepath.path.segments[0].ident;

            // print attributes
            for attr in impl_item.attrs.iter() {
                debug!("{:?}", attr.parse_meta());
            }

            // count associated methods into an impl block
            let mut amf_per_impl: u32 = 0;

            for impl_item_methods in impl_item.items.iter() {
                if let ImplItem::Method(m) = impl_item_methods {
                    debug!(
                        "Item: {}\tFound method {}",
                        ident.to_string(),
                        m.sig.ident.to_string()
                    );
                    amf_per_impl += 1;
                };
            }

            amfs.entry(ident.clone())
                .and_modify(|v| *v += amf_per_impl)
                .or_insert(amf_per_impl);
        }
    }

    amfs
}
