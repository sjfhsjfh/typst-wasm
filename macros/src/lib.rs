use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use venial::*;

fn construct_export(
    name: &proc_macro2::Ident,
    vis_marker: Option<VisMarker>,
    params: Vec<FnTypedParam>,
) -> proc_macro2::TokenStream {
    fn param_idx(name: &proc_macro2::Ident) -> proc_macro2::Ident {
        format_ident!("__{}_idx", name)
    }

    let inner_name = format_ident!("__wasm_minimal_protocol_internal_function_{}", name);
    let export_name = proc_macro2::Literal::string(&name.to_string());
    let p_idx = params
        .iter()
        .map(|p| &p.name)
        .map(param_idx)
        .collect::<Vec<_>>();
    let param_names = params.iter().map(|p| p.name.clone()).collect::<Vec<_>>();

    let get_unsplit_params = if params.len() == 0 {
        quote!()
    } else {
        quote!(
            let __total_len = #(#p_idx + )* 0;
            let mut __unsplit_params = vec![0u8; __total_len];
            typst_wasm_protocol::write_args_to_buffer(__unsplit_params.as_mut_ptr());
        )
    };

    let mut set_args = quote!(
        let mut start: usize = 0;
    );
    for param in params.iter() {
        let name = &param.name;
        let ty = &param.ty;
        let idx = param_idx(name);
        set_args.extend(quote!(
            let #name: #ty = (&__unsplit_params[start..start + #idx]).into();
            start += #idx;
        ));
    }

    quote!(
        #[export_name = #export_name]
        #vis_marker extern "C" fn #inner_name(#(#p_idx: usize),*) -> i32 {
            #get_unsplit_params
            #set_args

            let result = #name(#(#param_names),*);
            typst_wasm_protocol::PluginResult::send_result(result)
        }
    )
    .into()
}

#[proc_macro_attribute]
pub fn wasm_export(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    let decl = parse_item(item.clone()).expect("invalid declaration");
    let func = match decl.as_function() {
        Some(func) => func.clone(),
        None => {
            let error = venial::Error::new_at_tokens(
                &item,
                "#[wasm_export] can only be applied to a function",
            );
            item.extend(error.to_compile_error());
            return item.into();
        }
    };
    let Function {
        name,
        params,
        vis_marker,
        ..
    } = func.clone();

    let mut error = None;
    let p = params
        .items()
        .filter_map(|x| match x {
            FnParam::Receiver(_p) => {
                let x = x.to_token_stream();
                error = Some(venial::Error::new_at_tokens(
                    &x,
                    format!("the {x} argument is not allowed by the protocol"),
                ));
                None
            }
            FnParam::Typed(p) => Some(p.clone()),
        })
        .collect::<Vec<_>>();

    let mut result = quote!(#func);
    if let Some(error) = error {
        result.extend(error.to_compile_error());
    } else {
        result.extend(construct_export(&name, vis_marker, p));
    }
    result.into()
}
