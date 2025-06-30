use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn stated(args: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn stated_internal(args: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}
