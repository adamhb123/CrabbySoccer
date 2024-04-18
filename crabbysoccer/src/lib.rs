use proc_macro::TokenStream;

#[proc_macro]
pub fn println_around_input(items: TokenStream) -> TokenStream {
    format!(
        "println!(\"\n{}\"); print!(\"$ \"); io::stdout().flush().unwrap();",
        items
    )
    .parse()
    .unwrap()
}
