#![feature(proc_macro_quote)]

use proc_macro::quote;
use proc_macro::TokenStream;
#[proc_macro]
pub fn println_around_input(items: TokenStream) -> TokenStream {
    quote!(items);
    format!(
        "println!(\"\n{}\"); print!(\"$ \"); io::stdout().flush().unwrap();",
        items
    )
    .parse()
    .unwrap()
}
