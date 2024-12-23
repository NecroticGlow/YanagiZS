use proc_macro::TokenStream;

mod rpc;

#[proc_macro_attribute]
pub fn handlers(_attr: TokenStream, input: TokenStream) -> TokenStream {
    rpc::impl_handlers_module(input.into()).into()
}
