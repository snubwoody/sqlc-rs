use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, LitStr};
use sqlc_parser::{parse_schema, Table};

#[proc_macro]
pub fn parse_sql_str(input: TokenStream) -> TokenStream {
    let value = parse_macro_input!(input as LitStr);
    let schema = value.value();
    dbg!(&schema);
    let tables = parse_schema(&schema);
    let tables = parse_tables(&tables);
    let tokens = quote! {
        #(#tables)*
    };
    
    TokenStream::from(tokens)
}

fn parse_tables(tables: &[Table]) -> Vec<proc_macro2::TokenStream> {
    let mut structs = vec![];
    for table in tables {
        let name = to_pascal_case(table.name());
        let ident = Ident::new(&name, Span::call_site());
        let fields = parse_fields(&table);
        let tokens = quote! {
            #[derive(Debug)]
            pub struct #ident{
                #(#fields),*
            }
        };
        structs.push(tokens);
    }
    structs
}

fn parse_fields(table: &Table) -> Vec<proc_macro2::TokenStream> {
    let mut fields = vec![];
    for column in table.columns(){
        let name = column.name.to_lowercase();
        let name = Ident::new(&name, Span::call_site());
        let ty = &column.field_type;
        let field = quote! {
            pub #name: #ty
        };
        fields.push(field);
    }
    fields
}

fn to_pascal_case(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s|{
            let mut chars = s.chars();
            return chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str();
        })
        .collect()
}

