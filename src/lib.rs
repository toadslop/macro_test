use proc_macro::TokenStream;
use quote::{quote, ToTokens};

use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta};

#[proc_macro_attribute]
pub fn routes(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("ABOUT TO PRINT ARGS");
    for arg in args.into_iter() {
        println!("{}", arg.to_string());
    }
    // println!("{}", args.to_string());
    // let arg_metadata: AttributeArgs = syn::parse(args).unwrap();
    // arg_metadata.len();
    // get_config_file_name(&arg_metadata);
    //let second_seg = &meta.path().segments[1];
    // let paren = match &first_seg.arguments {
    //     syn::PathArguments::None => panic!("Need an argument"),
    //     syn::PathArguments::AngleBracketed(_) => panic!("No angle brackets please"),
    //     syn::PathArguments::Parenthesized(paren) => paren,
    // };

    // let inputs = &paren.inputs;
    // let output = &paren.output;
    // let paren_token = paren.paren_token;

    //let item_ast = parse_macro_input!(input as ItemFn);

    // impl_routes(&item_ast);
    input
}

fn get_config_file_name(arg_metadata: &Vec<NestedMeta>) {
    println!("Length of arg metadata {}", arg_metadata.len());
    let first_atr = &arg_metadata[0];
    let meta = match first_atr {
        NestedMeta::Meta(meta) => meta,
        NestedMeta::Lit(_) => panic!("This macro does not accept literal arguments."),
    };
    let path = meta.path();

    println!("TOKEN STREAM {}", path.to_token_stream().to_string());

    let first_seg = meta
        .path()
        .segments
        .first()
        .expect("Please provide a segment");
    let first_seg_ident = &first_seg.ident;

    println!("FIRST SEG IDENT {:?}", first_seg_ident.to_string());
    println!("FIRST sEG {}", first_seg.to_token_stream());
    match first_seg.arguments {
        syn::PathArguments::None => println!("Nothing"),
        syn::PathArguments::AngleBracketed(_) => println!("Angless"),
        syn::PathArguments::Parenthesized(_) => println!("Parens"),
    }
}

fn impl_routes(item_ast: &ItemFn) {
    let name = "name";
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // gen.into();
}
