use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn tl_ir(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
