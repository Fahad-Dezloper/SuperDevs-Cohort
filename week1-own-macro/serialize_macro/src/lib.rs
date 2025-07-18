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
                _ => panic!("Only named fields are supported")
            }
        }
        // Error if the data is not a struct
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


#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialize_number_struct(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    println(":?", ast);

    let name = &ast.ident;
    // let input_bytes = [0, 0, 0, 1, 0, 0, 0, 2];

  // ... generate code ...

    //“After all this work, we’ll return  deserialize_fields, field_assignments, total_size
    let (deserialize_fields, field_assignments, total_size) = match &ast.data {
        // if its a struct then go deep
        Data::Struct(data_struct) => {
            // Now check if the struct has named fields, like title: String
            match &data_struct.fields {
                // "Now that we confirmed this is a struct with named fields, prepare to generate code
                Fields::Named(fields) => {
                    // Keeps track of where each field starts in the byte stream when deserializing.
                    let mut offset: usize = 0;
                    // A list where you'll store code snippets for converting bytes into field values.
                    let mut field_deserializations = Vec::new();
                    // A list where you'll store how to assign those values to your struct’s fields.
                    let mut field_assignments = Vec::new();

                    for field in &fields.named {
                        // field_name is the field’s name (e.g., "age" or "height").
                        let field_name = &field.ident;
                        // we are assuming each field is i32, which takes 4 bytes.
                        let field_size = 4;
                        // offset is 0, so the for the first field is [0..4] which will be converted to age or height according to struct
                        let start_offset = offset;
                        // offset is 0, so the for the first field is [0..4] which will be converted to age or height according to struct
                        let end_offset = offset + field_size;

                        field_deserializations.push(quote! {
                            // Replaced at compile-time with actual field name like let age
                            let #field_name = {
                                //  Extracts the exact 4 bytes for this field from the byte slice
                                let bytes: [u8: 4] = base[#start_offset..#end_offset]
                                    .try_into()
                                    // of slice is not of [ 4 bytes show error]
                                    .map_err(|_| Error)?;
                                // Converts the 4 bytes into an i32 value using big-endian byte order.
                                i32::from_be_bytes(bytes)
                            };
                        });

                        field_assignments.push(quote! {
                            #field_name
                        });

                        offset += field_size;
                    }

                    (field_deserializations, field_assignments, offset)
                }
                //  If the struct doesn’t have named fields (l
             _ => panic!("Only named fields are supported"),
            }   
        }
        _ => panic!("Only structs are supported"),
    };
     




    let expanded = quote! {
        impl Deserialize for #name {
            fn deserialize(base: &[u8]) -> Result<Self, Error> {
                if base.len() < #total_size {
                    return Err(Error);
                }

                #(#deserialize_fields)*

                Ok(#name {
                    #(#field_assignments)*
                })
            }
        }
    };
    expanded.into()
}




///what serialize and deserialize exactly do
/// input = struct Example {
//     a: i32,
//     b: i32,
// }
// 
// 1. Serialize
// let example = Example { a: 1, b: 2 };
// let bytes = example.serialize();
// 
// let mut result = Vec::new();
// result.extend_from_slice(&self.a.to_be_bytes()); // → [0, 0, 0, 1]
// result.extend_from_slice(&self.b.to_be_bytes()); // → [0, 0, 0, 2]
//  Final output: [0, 0, 0, 1, 0, 0, 0, 2]

// 2. Deserialization (Bytes → Struct):
// let input_bytes = [0, 0, 0, 1, 0, 0, 0, 2];
// let example = Example::deserialize(&input_bytes);
// let a = i32::from_be_bytes(input_bytes[0..4].try_into()?); // a = 1
// let b = i32::from_be_bytes(input_bytes[4..8].try_into()?); // b = 2
//Final output: Example { a: 1, b: 2 }