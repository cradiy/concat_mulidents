use proc_macro::TokenStream;

#[proc_macro]
pub fn concat_idents(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.replace(" - - ", "");
    input.parse().unwrap()
}
