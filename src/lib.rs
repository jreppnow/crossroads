use proc_macro::TokenStream;

use syn::ItemFn;

#[proc_macro_attribute]
pub fn crossroads(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = dbg!(syn::parse_macro_input!(input as ItemFn));


    TokenStream::new()
}