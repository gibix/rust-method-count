/// AMF - Associated Methods and Functions
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
pub use syn::visit::{self, Visit};
use syn::ItemImpl;
use syn::{ImplItem, ItemMod, Type, Visibility};

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

/// AMF
#[derive(Default, Debug)]
pub struct AMF {
    /// Map of results h(ident) -> ItemCount
    pub tree: HashMap<String, ItemCount>,
    ext_mods: Vec<ItemMod>,
    trees: Vec<syn::File>,
    path: Option<PathBuf>,
}

impl<'ast> Visit<'_> for AMF {
    fn visit_item_impl(&mut self, node: &ItemImpl) {
        self.parse_item_impl(node);
        visit::visit_item_impl(self, node);
    }
    fn visit_item_mod(&mut self, node: &ItemMod) {
        if node.content.is_none() {
            self.parse_item_mod(node);
        }
        visit::visit_item_mod(self, node);
    }
}

impl<'ast> AMF {
    pub fn new() -> Self {
        AMF::default()
    }

    /// Store basic workdir for souce file path resolve
    pub fn from_path(path: impl Into<Option<PathBuf>>) -> Self {
        AMF {
            path: path.into().map(|p| {
                if p.is_file() {
                    p.parent();
                }
                p
            }),
            ..Default::default()
        }
    }

    fn parse_item_impl(self: &mut Self, impl_item: &'ast ItemImpl) {
        if let Type::Path(ref typepath) = *impl_item.self_ty {
            let ident = &typepath.path.segments[0].ident;

            // print attributes
            for attr in impl_item.attrs.iter() {
                debug!("{:?}", attr.parse_meta());
            }

            let count = impl_item
                .items
                .iter()
                .filter_map(|i| {
                    if let ImplItem::Method(m) = i {
                        debug!(
                            "Item: {}\tFound method {}",
                            ident.to_string(),
                            m.sig.ident.to_string(),
                        );
                        if let Visibility::Public(_) = m.vis {
                            Some(ItemCount {
                                public: 1,
                                private: 0,
                            })
                        } else {
                            Some(ItemCount {
                                public: 0,
                                private: 1,
                            })
                        }
                    } else {
                        None
                    }
                })
                .fold(ItemCount::default(), |mut c1, c2| {
                    c1.public += c2.public;
                    c1.private += c2.public;
                    c1
                });

            self.tree
                .entry(ident.to_string())
                .and_modify(|v: &mut ItemCount| {
                    v.public += count.public;
                    v.private += count.private;
                })
                .or_insert(count);
        }
    }

    pub fn merge(self: &mut Self, other: &mut Self) {
        for r_v in other.tree.iter() {
            self.tree
                .entry(r_v.0.to_owned())
                .and_modify(|l_v: &mut ItemCount| {
                    l_v.public += r_v.1.public;
                    l_v.private += r_v.1.private;
                })
                .or_insert(*r_v.1);
        }
    }

    fn file_into_syn(path: PathBuf) -> syn::File {
        let mut file = File::open(path).expect("Error on file");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");
        syn::parse_file(&src).expect("Unable to parse file")
    }

    /// try -> path/{mod}.rs || path/{mod}/$files
    fn parse_item_mod(self: &mut Self, mod_item: &ItemMod) {
        let basedir = self.path.as_ref().unwrap();

        let filename = mod_item.ident.to_string();

        let modpath = basedir.join(filename);

        if modpath.with_extension("rs").is_file() {
            let mut amf = AMF::from_path(modpath.with_extension("rs"));
            amf.visit_file(&AMF::file_into_syn(modpath.with_extension("rs")));
            self.merge(&mut amf)
        } else if modpath.is_dir() {
            for modfile in modpath.read_dir().unwrap().filter_map(|f| {
                if f.as_ref().unwrap().path().extension().unwrap_or_default() == "rs" {
                    Some(f.unwrap().path())
                } else {
                    None
                }
            }) {
                let mut amf = AMF::from_path(modfile.clone()); // TODO: bad clone
                amf.visit_file(&AMF::file_into_syn(modfile));
                self.merge(&mut amf)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    fn test_amf_over_str(file: &str) -> ItemCount {
        let ast = syn::parse_file(&file).unwrap();
        let mut counter = AMF::new();
        counter.visit_file(&ast);
        counter.tree["Tstruct"]
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

mod submod {
    impl Tstruct {
        pub fn submodstruct() {}
    }
}
";

        assert_eq!(
            test_amf_over_str(AMF_TEST_FILE),
            ItemCount {
                public: 2,
                private: 2
            }
        );
    }
}
