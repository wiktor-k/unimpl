#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use proc_macro::TokenStream;

struct SignatureWithSemicolon {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    sig: syn::Signature,
    _semicolon_token: syn::Token![;],
}

impl syn::parse::Parse for SignatureWithSemicolon {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let outer_attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse()?;
        let sig = input.parse()?;

        Ok(Self {
            attrs: outer_attrs,
            vis,
            sig,
            _semicolon_token: input.parse()?,
        })
    }
}

/// Indicates unimplemented function by panicking with a message of
/// `not implemented: <function name>` when the function is called.
///
/// This allows your code to provide required function
/// implementations, which is useful if you are prototyping or
/// implementing a trait that requires multiple methods which you
/// don’t plan to implement.
///
/// # Panics
///
/// This attribute generates a function body that immediately panics
/// with `not implemented: <function name>` message. This allows you
/// to quickly see which exact function is not implemented.
///
/// # Examples
///
/// ```rust
/// use unimpl::unimpl;
/// # use panic_message::panic_message;
///
/// #[unimpl]
/// pub fn func(a: u32) -> u32; // function body is autogenerated
///
/// let error = std::panic::catch_unwind(|| {
///     func(42);
/// }).unwrap_err();
///
/// // function name is automatically appended
/// assert_eq!(panic_message(&error), "not implemented: func");
/// ```
#[proc_macro_attribute]
pub fn unimpl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as SignatureWithSemicolon);

    let fn_name = input.sig.ident.to_string();

    let attrs = input.attrs;
    let signature = input.sig;
    let vis = input.vis;
    let output = quote::quote! {
        #(#attrs)*
        #vis
        #signature
        {
            unimplemented!(#fn_name);
        }
    };

    output.into()
}
