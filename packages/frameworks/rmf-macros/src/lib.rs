use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, ImplItem, ImplItemFn, Visibility, parse_macro_input, spanned::Spanned, token::Pub,
};

#[proc_macro_attribute]
pub fn delegate_implements(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

    delegate_implements_internal(input).into()
}

fn delegate_implements_internal(input: syn::ItemImpl) -> proc_macro2::TokenStream {
    if input.trait_.is_some() {
        let mut struct_impl = input.clone();
        struct_impl.trait_ = None;
        struct_impl.items = struct_impl
            .items
            .iter()
            .filter(|item| matches!(item, ImplItem::Fn(_) | ImplItem::Const(_)))
            .cloned()
            .map(|item| match item {
                ImplItem::Fn(mut f) => {
                    f.vis = Visibility::Public(Pub::default());
                    ImplItem::Fn(f)
                }
                ImplItem::Const(mut c) => {
                    c.vis = Visibility::Public(Pub::default());
                    ImplItem::Const(c)
                }
                _ => panic!("not implemented"),
            })
            .collect();

        let mut trait_impl = input;
        let trait_items = trait_impl
            .items
            .iter()
            .map(|item| match item {
                ImplItem::Fn(impl_item_fn) => {
                    let sig = &impl_item_fn.sig;
                    let fn_ident = &sig.ident;
                    let patterns = sig.inputs.iter().map(|input| match input {
                        FnArg::Typed(ty) => {
                            let pat = &ty.pat;
                            quote! {#pat}
                        }
                        FnArg::Receiver(receiver) => {
                            let self_token = receiver.self_token;
                            quote! {#self_token}
                        }
                    });
                    let block = syn::parse2(quote! {
                    {
                        Self::#fn_ident(#(#patterns,)*)
                    }
                    })
                    .expect("must parse succeed");

                    syn::ImplItem::Fn(ImplItemFn {
                        block,
                        attrs: impl_item_fn.attrs.clone(),
                        vis: impl_item_fn.vis.clone(),
                        defaultness: impl_item_fn.defaultness,
                        sig: impl_item_fn.sig.clone(),
                    })
                }
                _ => item.clone(),
            })
            .collect::<Vec<_>>();
        trait_impl.items = trait_items;

        quote! {
            #struct_impl

            #trait_impl
        }
    } else {
        syn::Error::new(input.span(), "Only use impl trait").to_compile_error()
    }
}
