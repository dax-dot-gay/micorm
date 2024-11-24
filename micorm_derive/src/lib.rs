extern crate proc_macro;

use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;

#[derive(FromMeta, Debug)]
struct DocConfig {
    pub database: String,
    pub collection: String
}

#[derive(FromDeriveInput)]
#[darling(attributes(micorm), forward_attrs(allow, doc, cfg))]
#[allow(dead_code)]
struct DocConfigOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    db: DocConfig
}

#[proc_macro_derive(Document, attributes(micorm))]
pub fn derive_document(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("AST construction failed");
    let opts = DocConfigOpts::from_derive_input(&ast).expect("Failed to get options");
    let name = opts.ident;
    let database = opts.db.database;
    let collection = opts.db.collection;

    let gen = quote! {
        impl Document for #name {
            fn collection_name() -> String {
                String::from(#collection)
            }

            fn database_name() -> String {
                String::from(#database)
            }

            fn get_id(&self) -> Option<Uuid> {
                self.id
            }

            fn set_id(&mut self, id: Uuid) -> () {
                self.id = Some(id);
            }
        }
    };

    gen.into()
}

