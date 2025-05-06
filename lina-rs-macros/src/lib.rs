use darling::{Error, FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Field, Fields, FieldsNamed, Ident, ItemImpl, ItemStruct, parse_macro_input, parse_quote,
    punctuated::Punctuated,
};

#[derive(Debug, FromMeta)]
struct RepoArgs {
    database: Ident,
}

#[proc_macro_attribute]
pub fn repo(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let input = parse_macro_input!(item as ItemStruct);
    let args = match RepoArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let name = &input.ident;
    let vis = &input.vis;
    let database = &args.database;

    let conn_field: Field = parse_quote! {
        pub connection: T
    };

    let new_fields = match input.fields {
        Fields::Named(mut named) => {
            named.named.push(conn_field);
            Fields::Named(named)
        }
        _ => {
            let mut named = Punctuated::new();
            named.push(conn_field);
            Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named,
            })
        }
    };

    quote! {
        #vis struct #name<T> #new_fields

        impl<T> lina_rs::sqlx::Repository<T> for #name<T> {
            type DB = #database;

            fn new<'a>(connection: T) -> Self
            where
                T: sqlx::Acquire<'a, Database = Self::DB>,
            {
                #name { connection }
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn repo_impl(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let self_ty = &input.self_ty;
    let items = &input.items;

    quote! {
        impl<'a, T> #self_ty<T>
        where
            T: sqlx::Acquire<'a, Database = <#self_ty<T> as lina_rs::sqlx::Repository<T>>::DB>,
        {
            async fn conn(self) -> Result<T::Connection, sqlx::Error> {
                self.connection.acquire().await
            }

            #(#items)*
        }
    }
    .into()
}
