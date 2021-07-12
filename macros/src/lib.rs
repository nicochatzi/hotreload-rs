use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn from(attr: TokenStream, item: TokenStream) -> TokenStream {
    let lib_name = attr.into_iter().next().expect("Must supply a proc macro attribute with the name of a registered library to hotreload a function from");
    let fn_name_str = "print";

    let tokens = quote! {
        // derive the function pointer signature
        // from the parent function signature
        #[inline(always)]
        fn dylib_fn(hot_fn: hotreload::lib::Symbol<fn()>) {
            hot_fn()
        }

        // same signature as parent function
        #[inline(always)]
        fn fallback_fn() {
            println!("fallback function");
        }

        // use `crate::` to allow this hot function
        // to be declared anywhere in inside of the
        // consumer crate
        ::crate::HOTRELOAD_#lib_name.call_or_fallback(stringify!(#fn_name), dylib_fn, fallback_fn)
    };

    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    println!("quot: \"{}\"", tokens.to_string());

    TokenStream::from(tokens)
}
