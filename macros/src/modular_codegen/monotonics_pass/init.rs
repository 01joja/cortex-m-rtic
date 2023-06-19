use proc_macro2::{Span,TokenStream as TokenStream2};
use quote::quote;
use rtic_syntax::{ast::{App, Monotonic}, Context, analyze::Priority, ast::SoftwareTask};

use std::str::FromStr;
use std::collections::HashMap;

use crate::{
    analyze::Analysis,
    check::Extra,
    codegen::util,
};


use syn::{Ident, LitInt, Path};

use super::m_names;

/// Generates code for pre init, post init and more. 
pub fn codegen(
    app: &App, 
    extra: &Extra,
) -> (
    // pre_init - 
    Vec<TokenStream2>,
    // post_init - 
    Vec<TokenStream2>,
){

    let mut spawn_handlers = vec![quote!()];
    let mut pre_init = vec![quote!()];
    let mut post_init = vec![];

    
    let device = &extra.device;
    let nvic_prio_bits = quote!(#device::NVIC_PRIO_BITS);

    for (index, (name, monotonic)) in app.monotonics.iter().enumerate(){
        let monotonic_storage= m_names::monotonic_storage(name);

        pre_init.extend(codegen_pre_init(monotonic, &nvic_prio_bits));

        post_init.push(
            quote!{
                monotonics.#index.reset();
                #monotonic_storage.get_mut().write(Some(monotonics.#index));
            }
        )

    }


    return (pre_init,post_init);
}

/// generates init needed for monotonic.
fn codegen_pre_init(monotonic: &Monotonic, nvic_prio_bits: &TokenStream2) 
    -> 
    // pre_init
    Vec<TokenStream2>
    {
    let mut pre_init = vec![];

    let name = &monotonic.ident;
    let interrupt = m_names::interrupt();
    let binds = &monotonic.args.binds;
    let priority = if let Some(prio) = &monotonic.args.priority{
        quote! { #prio }
    } else {
        quote! { (1 << #nvic_prio_bits) }
    };
    
    let es = format!(
        "Maximum priority used by monotonic '{}' is more than supported by hardware",
        name
    );
    
    // Compile time assert that this priority is supported by the device
    pre_init.push(quote!(
        const _: () =  if (1 << #nvic_prio_bits) < #priority as usize { ::core::panic!(#es); };
    ));

    let mono_type = &monotonic.ty;

    if &*binds.to_string() == "SysTick" {
        pre_init.push(quote!(
            core.SCB.set_priority(
                rtic::export::SystemHandler::SysTick,
                rtic::export::logical2hw(#priority, #nvic_prio_bits),
            );

            // Always enable monotonic interrupts if they should never be off
            if !<#mono_type as rtic::Monotonic>::DISABLE_INTERRUPT_ON_EMPTY_QUEUE {
                core::mem::transmute::<_, rtic::export::SYST>(())
                    .enable_interrupt();
            }
        ));
    } else {
        let rt_err = m_names::rt_error();
        pre_init.push(quote!(
            core.NVIC.set_priority(
                #rt_err::#binds::#binds,
                rtic::export::logical2hw(#priority, #nvic_prio_bits),
            );

            // Always enable monotonic interrupts if they should never be off
            if !<#mono_type as rtic::Monotonic>::DISABLE_INTERRUPT_ON_EMPTY_QUEUE {
                rtic::export::NVIC::unmask(#rt_err::interrupt::#binds);
            }
        ));
    }
    pre_init
}

