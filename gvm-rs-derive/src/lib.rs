// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use proc_macro::TokenStream;
use quote::quote;

fn impl_has_id(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HasId for #name {
            fn id(&self) -> Option<&Uuid> {
                Some(&self.id)
            }
        }
    };
    generated.into()
}

#[proc_macro_derive(HasId)]
pub fn has_id_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_has_id(&ast)
}
