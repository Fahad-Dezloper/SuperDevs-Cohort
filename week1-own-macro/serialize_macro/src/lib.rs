// tokenstream is used to recive and return rust code in macro defination
// tokenStream is data type in proc_macro. which contains stream of rust code token like fn , struct
// proc_macro is procedural macro
use proc_macro::TokenStream;
use quote::quote;

//  user writes #[derive(SerializeNumberStruct)], Rust turns the struct code into a TokenStream and passes it here.
#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialise_number_struct(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    // ex input: struct Book {
    //     title: String,
    // } 

    // here it is matching whethere the data is struct, enum, union
    let serialize_fields = match &ast.data {
        // if its a struct do deeper into it
        Data::Struct(data_struct) => {
            // here it gives the list of fields inside the struct.
            // fields = Named {
            //     title: String
            // }
            match &data_struct.fields {
                // here we are handling named fields like => title: String
                Fields::Named(fields) => {
                    // here we are looping over each field
                    let field_serializations = fields.named.iter().map(|field|{
                        // field_name = Some("title")
                        let field_name = &field.ident;
                        quote! {
                            // result.extend_from_slice(&self.title.to_be_bytes());
                            result.extend_from_slice(&self.#field_name.to_be_bytes());
                        }
                });
                /*
                        field_serializeations = [quote!(result.extend_from_slice(&self.qty_1.to_be_bytes())), quote!(result.extend_from_slice(&self.qty_2.to_be_bytes()))]
                     */
                    quote! {
                        #(#field_serializations)*
                    }
                }
                // Error if fields are not named properly
                _ => panic!("Only named fields are supported").
            }
        }
        // Error if not a struct data is struct
        _ => panic!("Only structs are supported"),
    };
    /*
        serialize_fields ->
        result.extend_from_slice(&self.qty_1.to_be_bytes())
        result.extend_from_slice(&self.qty_2.to_be_bytes())
        result.extend_from_slice(&self.qty_3.to_be_bytes())
     */



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
    let expanded = quote! {
        // generated code here
        impl Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::new();
                #serialize_fields
                result
            }
        }
    };
    
    // println!("{:?}", ast);

    expanded.into()
}

// #[proc_macro_derive(DeserializeNumberStruct)]
// #[proc_macro_derive(DeserializeNumberStruct)]
// pub fn deserialize_number_struct(input: TokenStream) -> TokenStream {
//     let ast: syn::DeriveInput = syn::parse(input).unwrap();
//     // ... generate code ...
//     let expanded = quote! {
//         // generated code here
//     };
//     expanded.into()
// }
