use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, ExprLit, FnArg, ItemFn, Lit, LitStr, MetaNameValue, Token, Type};

#[derive(Default)]
struct MacroOptions {
    export_rename: Option<LitStr>,
    check_args: Option<bool>,
}

fn parse_options(args: TokenStream) -> syn::Result<MacroOptions> {
    let mut options = MacroOptions::default();
    let args = Punctuated::<MetaNameValue, Token![,]>::parse_terminated.parse(args)?;
    for arg in args {
        if arg.path.is_ident("export_rename") {
            match arg.value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(lit), ..
                }) => options.export_rename = Some(lit),
                value => {
                    return Err(syn::Error::new_spanned(
                        value,
                        "`export_rename` expects a string literal",
                    ));
                }
            }
        } else if arg.path.is_ident("check_args") {
            match arg.value {
                Expr::Lit(ExprLit {
                    lit: Lit::Bool(lit),
                    ..
                }) => options.check_args = Some(lit.value),
                value => {
                    return Err(syn::Error::new_spanned(
                        value,
                        "`check_args` expects a boolean literal",
                    ));
                }
            }
        } else {
            return Err(syn::Error::new_spanned(
                arg.path,
                "unknown argument; expected `export_rename = \"...\"` or `check_args = true/false`",
            ));
        }
    }
    Ok(options)
}

fn expand(func: ItemFn, options: MacroOptions) -> syn::Result<proc_macro2::TokenStream> {
    let mut types = Vec::<Type>::new();
    for arg in &func.sig.inputs {
        match arg {
            FnArg::Receiver(receiver) => {
                return Err(syn::Error::new_spanned(
                    receiver,
                    "`self` is not supported by the wasm plugin protocol",
                ));
            }
            FnArg::Typed(pat_type) => {
                if matches!(*pat_type.ty, Type::ImplTrait(_)) {
                    return Err(syn::Error::new_spanned(
                        &pat_type.ty,
                        "`impl Trait` parameters are not supported; name a concrete type instead",
                    ));
                }
                types.push((*pat_type.ty).clone());
            }
        }
    }

    let name = &func.sig.ident;
    let vis = &func.vis;
    let export_name = options
        .export_rename
        .unwrap_or_else(|| LitStr::new(&name.to_string(), name.span()));
    let inner_name = format_ident!("__wasm_minimal_protocol_internal_function_{}", name);

    let lens = (0..types.len())
        .map(|i| format_ident!("__wasm_arg_len_{}", i))
        .collect::<Vec<_>>();
    let args = (0..types.len())
        .map(|i| format_ident!("__wasm_arg_{}", i))
        .collect::<Vec<_>>();

    let body = if types.is_empty() {
        quote! {
            let __wasm_result = #name();
            typst_wasm_protocol::send_response(__wasm_result)
        }
    } else {
        if options.check_args.unwrap_or(false) {
            quote! {
                let __wasm_args = match typst_wasm_protocol::read_args(&[#(#lens),*]) {
                    Ok(args) => args,
                    Err(error) => return typst_wasm_protocol::send_response(error),
                };
                let mut __wasm_reader = typst_wasm_protocol::ArgsReader::new(&__wasm_args);

                #(
                    let #args: #types = match __wasm_reader.read(#lens) {
                        Ok(value) => value,
                        Err(error) => return typst_wasm_protocol::send_response(error),
                    };
                )*

                let __wasm_result = #name(#(#args),*);
                typst_wasm_protocol::send_response(__wasm_result)
            }
        } else {
            quote! {
                let __wasm_args = typst_wasm_protocol::read_args_unchecked(&[#(#lens),*]);
                let mut __wasm_reader = typst_wasm_protocol::ArgsReader::new(&__wasm_args);

                #(
                    let #args: #types = match __wasm_reader.read_unchecked(#lens) {
                        Ok(value) => value,
                        Err(error) => return typst_wasm_protocol::send_response(error),
                    };
                )*

                let __wasm_result = #name(#(#args),*);
                typst_wasm_protocol::send_response(__wasm_result)
            }
        }
    };

    Ok(quote! {
        #func

        #[unsafe(export_name = #export_name)]
        #vis extern "C" fn #inner_name(#(#lens: usize),*) -> i32 {
            #body
        }
    })
}

/// Export a function as a Typst wasm plugin entry point.
///
/// Each parameter receives one argument passed from Typst, in declaration
/// order, and must implement `FromArg` (for example `Vec<u8>`, `&[u8]`,
/// `String`, or `Json<T>`). The return type must implement `IntoResponse`
/// (for example `Vec<u8>`, `String`, `Json<T>`, or `Result<_, PluginError>`).
///
/// Parameters may use patterns, so extractor-style signatures work:
///
/// ```ignore
/// #[wasm_export]
/// pub fn detect(Json(input): Json<Input>) -> Result<Json<Output>, PluginError> { ... }
/// ```
///
/// ## Export Rename
///
/// ```ignore
/// #[wasm_export(export_rename = "other_name")]
/// pub fn my_function() -> String { ... }
/// ```
///
/// ## Length Checking
///
/// For performance reasons, argument length checking is disabled by default.
/// You can enable it by setting `check_args = true` in the attribute options.
///
/// ```ignore
/// #[wasm_export(check_args = true)]
/// pub fn say_six_seven() -> String { ... }
/// ```
///
#[proc_macro_attribute]
pub fn wasm_export(args: TokenStream, item: TokenStream) -> TokenStream {
    let options = match parse_options(args) {
        Ok(options) => options,
        Err(error) => return error.to_compile_error().into(),
    };
    let func = match syn::parse::<ItemFn>(item) {
        Ok(func) => func,
        Err(error) => return error.to_compile_error().into(),
    };
    match expand(func, options) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
