use darling::{Error, FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemStruct, parse_macro_input};

#[derive(Debug, FromMeta)]
struct RepoArgs {
    table: String,
    model: Option<Ident>,
}

#[proc_macro_attribute]
pub fn repo(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let item = parse_macro_input!(input as ItemStruct);
    let args = match RepoArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let name = &item.ident;
    let table = args.table;

    let crud_impl = if let Some(model) = args.model {
        quote! {
            #[lina_rs::prelude::async_trait]
            impl lina_rs::sqlx::Crud for #name {
                type Model = #model;

                async fn create(&self, model: &Self::Model) {
                    println!("{:?}", model)
                }
            }
        }
    } else {
        quote!()
    };

    quote! {
        #item

        impl lina_rs::sqlx::Repository for #name {
            fn table(&self) -> String {
                (#table).to_string()
            }
        }

        #crud_impl
    }
    .into()
}
