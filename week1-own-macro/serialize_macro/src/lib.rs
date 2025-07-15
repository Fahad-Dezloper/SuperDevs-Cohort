// tokenstream is used to recive and return rust code in macro defination
// tokenStream is data type in proc_macro. which contains stream of rust code token like fn , struct
// proc_macro is procedural macro
use proc_macro::TokenStream;
use quote::quote;

//  user writes #[derive(SerializeNumberStruct)], Rust turns the struct code into a TokenStream and passes it here.
#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialise_number_struct(input: TokenStream) -> TokenStream {
    // syn::parse(input) converts the code to AST (Abstract Syntax Tree)
    // ex input: struct Book {
    //     title: String,
    // } 
    // ex output: DeriveInput {
    //     ident: "Book",
    //     data: Struct {
    //         fields: Named {
    //             title: String
    //         }
    //     },
    //     attrs: ...
    // }
    // .unwrap says if it fails panic and crash the macro with an error message
    let ast = syn::parse(input).unwrap();
}

#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialize_number_struct(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
}