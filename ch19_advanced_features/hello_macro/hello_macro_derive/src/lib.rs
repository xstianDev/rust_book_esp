/* Para poder usar proc_crate hay que ir a Cargo.toml y escribir:
 * [lib]
 * proc-macro = true
 * 
 * Luego hay que importarlo como crate externa, aunque es nativa de Rust.
 * Después se hacen las siguientes importaciones:
 * - proc_macro::TokenStream
 * - quote::quote : puede tomar una estructura de árbol sintáctico y convertirlo a código.
 * - syn : crate de sintaxis. Toma un string de código y lo convierte a estructura de árbol sintáctico. 
 */

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Crea una estructura de árbol sintáctico que podemos manipular
    let ast = syn::parse(input).unwrap();

    // Construye la implementación del trait
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!(
                    "Hello, Macro! My name is {}!",
                    stringify!(#name)
                );
            }
        }
    };
    gen.into()
}