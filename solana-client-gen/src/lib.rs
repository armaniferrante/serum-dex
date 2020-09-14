#![feature(box_syntax, box_patterns)]

extern crate proc_macro;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate proc_quote;

#[proc_macro_attribute]
pub fn solana_client_gen(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let instruction_enum = parse_macro_input!(input as syn::ItemMod);

    proc_macro::TokenStream::from(quote! {
            pub fn hello_test_inside() {
                    println!("iasdfasdfnside!");
            }
            #instruction_enum
    })
}
