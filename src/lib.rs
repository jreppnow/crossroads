use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn crossroads(args: TokenStream, input: TokenStream) -> TokenStream {
    input
}