use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, FnArg, GenericArgument, Ident, Item, ItemMod, PathArguments, Type};

pub fn impl_handlers_module(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let module = parse_macro_input!(input as ItemMod);
    let mod_name = &module.ident;
    let (_, items) = module
        .content
        .as_ref()
        .expect("#[handlers] module should contain rpc handler functions");

    let mut output_module = TokenStream::new();
    let mut output_match_arms = TokenStream::new();
    let mut rpc_registers = TokenStream::new();

    for item in items.iter() {
        let Item::Fn(func) = item else {
            output_module.extend(item.to_token_stream());
            continue;
        };

        let sig = &func.sig;
        let name = &sig.ident;

        assert_eq!(
            sig.inputs.len(),
            1,
            "functions in #[handlers] module should accept only 1 argument"
        );

        let argument = match sig.inputs.get(0).as_ref().unwrap() {
            FnArg::Typed(argument) => argument,
            FnArg::Receiver(_) => {
                panic!("receivers are not allowed in #[handlers] module function arguments")
            }
        };

        output_module.extend(quote! {
            #[tracing::instrument(skip_all)]
            #func
        });

        let rpc_arg_type = get_underlying_type(argument.ty.as_ref());
        let rpc_base_name = {
            let Type::Path(path) = rpc_arg_type else {
                panic!();
            };

            path.path
                .segments
                .last()
                .unwrap()
                .ident
                .to_string()
                .strip_suffix("Arg")
                .unwrap()
                .to_string()
        };

        rpc_registers.extend(quote! {
            point.register_rpc_recv(#rpc_arg_type::PROTOCOL_ID, process_rpc);
        });

        let rpc_base_ident = Ident::new(&rpc_base_name, Span::call_site());
        if rpc_base_name.starts_with("Rpc") {
            let rpc_ret_type = Ident::new(&format!("{rpc_base_name}Ret"), Span::call_site());

            output_match_arms.extend(quote! {
                #rpc_arg_type::PROTOCOL_ID => {
                    let Ok(arg) = rpc_ptc.get_arg::<#rpc_arg_type>() else {
                        return;
                    };

                    let mut ctx = NetworkContext {
                        arg,
                        rpc_ptc,
                        session: &mut session,
                        globals: ::common::util::Ptr::Static(crate::GLOBALS.get().unwrap()),
                    };

                    match #mod_name::#name(&mut ctx).await {
                        Ok(ret) => ctx.rpc_ptc.send_ret(ret).await,
                        Err(retcode) => ctx.rpc_ptc.send_ret(#rpc_ret_type {
                            retcode,
                            ..Default::default()
                        }).await
                    }

                    ::tracing::info!("successfully handled {}", stringify!(#rpc_base_ident));
                    super::post_rpc_handle(ctx.session).await;
                }
            });
        } else if rpc_base_name.starts_with("Ptc") {
            output_match_arms.extend(quote! {
                #rpc_arg_type::PROTOCOL_ID => {
                    let Ok(arg) = rpc_ptc.get_arg::<#rpc_arg_type>() else {
                        return;
                    };

                    let mut ctx = NetworkContext {
                        arg,
                        rpc_ptc,
                        session: &mut session,
                        globals: ::common::util::Ptr::Static(crate::GLOBALS.get().unwrap()),
                    };

                    #mod_name::#name(&mut ctx).await;
                    ::tracing::info!("successfully handled {}", stringify!(#rpc_base_ident));
                    super::post_rpc_handle(ctx.session).await;
                }
            });
        }
    }

    quote! {
        pub fn register_handlers(point: &::qwer_rpc::RpcPtcPoint) {
            use ::protocol::*;
            use ::qwer::ProtocolID;

            #rpc_registers
        }

        pub async fn process_rpc(rpc_ptc: qwer_rpc::RpcPtcContext) {
            use ::protocol::*;
            use ::qwer::ProtocolID;
            use crate::rpc_ptc::*;

            let Some(::qwer_rpc::middleware::MiddlewareModel::Account(account_mw)) = rpc_ptc
                .middleware_list
                .iter()
                .find(|&mw| matches!(mw, ::qwer_rpc::middleware::MiddlewareModel::Account(_)))
            else {
                ::tracing::warn!("failed to handle rpc: account middleware is missing");
                return;
            };

            let Some(mut session) = crate::PLAYER_MAP.get_mut(&account_mw.player_uid) else {
                ::tracing::warn!("failed to handle rpc: player session with uid {} is not active", account_mw.player_uid);
                return;
            };

            match rpc_ptc.protocol_id {
                #output_match_arms
                _ => (),
            }
        }

        mod #mod_name {
            #output_module
        }
    }.into()
}

fn get_underlying_type(ty: &Type) -> &Type {
    let Type::Reference(reference) = ty else {
        panic!(
            "the argument type of #[handlers] module function should be &mut NetworkContext<'_, T>"
        );
    };

    let Type::Path(path) = reference.elem.as_ref() else {
        panic!(
            "the argument type of #[handlers] module function should be &mut NetworkContext<'_, T>"
        );
    };

    let last_segment = path.path.segments.last();
    let last_segment = last_segment.as_ref().unwrap();

    let PathArguments::AngleBracketed(args) = &last_segment.arguments else {
        panic!(
            "the argument type of #[handlers] module function should be &mut NetworkContext<'_, T>"
        );
    };

    if let GenericArgument::Type(ty) = args.args.iter().nth(1).as_ref().unwrap() {
        &ty
    } else {
        panic!(
            "the argument type of #[handlers] module function should be &mut NetworkContext<'_, T>"
        );
    }
}
