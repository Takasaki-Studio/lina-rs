use darling::{Error, FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemImpl, LitStr, parse_macro_input};

#[derive(Debug, FromMeta)]
struct RepoArgs {
    db: Ident,
}

#[proc_macro_attribute]
pub fn repo(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match RepoArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let input = parse_macro_input!(item as ItemImpl);

    let name = &input.self_ty;
    let items = &input.items;
    let db = &args.db;

    quote! {
        pub struct #name<'a>
        {
            conn: &'a mut <#db as sqlx::Database>::Connection,
        }

        impl<'a> #name<'a>
        {
            pub fn new(conn: &'a mut <#db as sqlx::Database>::Connection) -> Self {
                Self { conn }
            }

            #(#items)*
        }
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct ImplReposArgs {
    db: Ident,
    name: Ident,
    #[darling(multiple)]
    method: Vec<MethodEntry>,
}

#[derive(Debug, FromMeta)]
struct MethodEntry {
    name: LitStr,
    repo: Ident,
}

#[proc_macro]
pub fn impl_repos(args: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match ImplReposArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let name = &args.name;
    let db = &args.db;

    let methods = args.method.iter().map(|m| {
        let repo = &m.repo;
        let name = Ident::new(&m.name.value(), m.name.span());

        quote! {
            fn #name(&'a mut self) -> #repo<'_>;
        }
    });

    let impl_methods = args.method.iter().map(|m| {
        let repo = &m.repo;
        let name = Ident::new(&m.name.value(), m.name.span());

        quote! {
            fn #name(&mut self) -> #repo<'_> {
                #repo::new(self)
            }
        }
    });

    quote! {
        pub trait #name<'a> {
            #(#methods)*
        }

        impl #name<'_> for <#db as sqlx::Database>::Connection {
            #(#impl_methods)*
        }
    }
    .into()
}
