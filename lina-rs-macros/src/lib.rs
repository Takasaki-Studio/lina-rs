use darling::{Error, FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemImpl, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn repo(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let name = &input.self_ty;
    let items = &input.items;

    quote! {
        pub struct #name<'a, T>
        where
            T: sqlx::Database,
        {
            conn: &'a mut T::Connection,
        }

        impl<'a, T> #name<'a, T>
        where
            T: Database,
        {
            pub fn new(conn: &'a mut T::Connection) -> Self {
                Self { conn }
            }
        }

        impl<T> #name<'_, T>
        where
            T: sqlx::Database,
            for<'c> &'c mut T::Connection:  sqlx::Executor<'c, Database = T>,
            for<'q> <T as  sqlx::Database>::Arguments<'q>:  sqlx::IntoArguments<'q, T>,
        {
            #(#items)*
        }
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct ReposArgs {
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

    let args = match ReposArgs::from_list(&attr_args) {
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
            fn #name(&'a mut self) -> #repo<'_, Self::DB>;
        }
    });

    let impl_methods = args.method.iter().map(|m| {
        let repo = &m.repo;
        let name = Ident::new(&m.name.value(), m.name.span());

        quote! {
            fn #name(&mut self) -> #repo<'_, Self::DB> {
                #repo::new(self)
            }
        }
    });

    quote! {
        pub trait #name<'a> {
            type DB: sqlx::Database;
            #(#methods)*
        }

        impl #name<'_> for <#db as sqlx::Database>::Connection {
            type DB = #db;

            #(#impl_methods)*
        }
    }
    .into()
}
