/// AMF - Associated Methods and Functions
extern crate syn;
#[macro_use]
extern crate log;

use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::BTreeMap;
use syn::{ImplItem, Item, Type, Visibility};

/// Store the result
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct ItemCount {
    /// Number of public associated functions or methods
    public: u32,
    /// Number of private associated functions or methods
    private: u32,
}

impl ItemCount {
    pub fn private(self: Self) -> u32 {
        self.public
    }

    pub fn public(self: Self) -> u32 {
        self.private
    }

    pub fn total(self: Self) -> u32 {
        self.private + self.public
    }
}

impl Serialize for ItemCount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("NumberOfMethods", 3)?;
        state.serialize_field("public", &self.public)?;
        state.serialize_field("private", &self.private)?;
        state.serialize_field("total", &(self.public + self.private))?;
        state.end()
    }
}

pub fn amf_count(syntax_tree: syn::File) -> BTreeMap<String, ItemCount> {
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
            let mut entry = ItemCount::default();

            for impl_item_methods in impl_item.items.iter() {
                if let ImplItem::Method(m) = impl_item_methods {
                    debug!(
                        "Item: {}\tFound method {}",
                        ident.to_string(),
                        m.sig.ident.to_string(),
                    );
                    if let Visibility::Public(_) = m.vis {
                        entry.public += 1;
                    } else {
                        entry.private += 1;
                    };
                };
            }

            amfs.entry(ident.to_string())
                .and_modify(|v: &mut ItemCount| {
                    v.public += entry.public;
                    v.private += entry.private;
                })
                .or_insert(entry);
        }
    }

    amfs
}

#[cfg(test)]
mod test {
    use crate::*;

    fn test_amf_over_str(file: &str) -> ItemCount {
        let ast = syn::parse_file(&file).unwrap();
        let res = amf_count(ast);
        res["Tstruct"]
    }

    #[test]
    fn amf_test() {
        static AMF_TEST_FILE: &'static str = "
struct Tstruct {}

impl Tstruct {
    fn tfn() { }
    pub fn pubtfn() { }
}

trait Ttrait {
    fn trfn();
}

impl Ttrait for Tstruct {
    fn trfn() { }
}
";

        assert_eq!(
            test_amf_over_str(AMF_TEST_FILE),
            ItemCount {
                public: 1,
                private: 2
            }
        );
    }
}
