use proc_macro::TokenStream;
use syn::{ ItemEnum, Variant, Fields, Attribute,
           Token, token::{ Comma }, Field,
           Type, LitInt, Result, Ident,
           parse::{Parse, ParseStream},
           token::Paren, punctuated::Punctuated,
           parenthesized};
use quote::quote;

// mod opcode;
// use opcode::Opcode;

mod yage_kw {
    syn::custom_keyword!(opcodes);
    syn::custom_keyword!(args);
}

struct Op {
    name: yage_kw::opcodes,
    eq_token: Token![=],
    paren_token: Paren,
    values: Punctuated<LitInt, Token![,]>
}

struct Args {
    name: yage_kw::args,
    eq_token: Token![=],
    paren_token: Paren,
    values: Punctuated<Type, Token![,]>
}

struct Bind {
    opcodes: Op,
    comma_token: Comma,
    args: Args
}


impl Parse for Bind {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Bind {
            opcodes: input.parse()?,
            comma_token: input.parse()?,
            args: input.parse()?
        })
    }
}

impl Parse for Op {
    fn parse(input: ParseStream) -> Result<Self> {
        let values;

        Ok(Op {
            name: input.parse()?,
            eq_token: input.parse()?,
            paren_token: parenthesized!(values in input),
            values: values.parse_terminated(LitInt::parse)?
        })
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let values;

        Ok(Args {
            name: input.parse()?,
            eq_token: input.parse()?,
            paren_token: parenthesized!(values in input),
            values: values.parse_terminated(Type::parse)?
        })
    }
}


#[proc_macro_derive(YageInstructions, attributes(bind))]
pub fn instructions(input: TokenStream) -> TokenStream {
    let enum_item: ItemEnum = syn::parse(input).expect("Instructions derive only works on enums");
    let enum_name = &enum_item.ident;
    let mut all_matchs = Vec::new();

    for variant in &enum_item.variants {
        let name = &variant.ident;
        let fields = get_fields(variant);
        let bindings = parse_bindings(&variant.attrs);
        let variant_matchs = build_matchs(&enum_name, &name, &bindings, &fields);

        all_matchs.extend(variant_matchs);
    }

    let enum_impl = build_impl(&enum_name, &all_matchs);

    panic!(enum_impl.to_string());

    TokenStream::from(enum_impl)
}

fn build_impl(enum_name: &Ident, matchs: &Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    quote! {
        impl #enum_name {
            fn consume(bytes: Vec<u8>) -> Self {
                match bytes[0..3] {
                    #(#matchs),*
                }
            }
        }
    }
}

fn build_matchs(enum_name: &Ident, name: &Ident, bindings: &Vec<Bind>, fields: &Option<&Punctuated<Field, Comma>>) -> Vec<proc_macro2::TokenStream> {
    let mut match_op = Vec::new();

    for binding in bindings {
        let bytes = &binding.opcodes.values;
        let values = &binding.args.values;
        match fields {
            Some(f) => {
                let args = compute_args(f, values, &bytes);
                match_op.push(quote! {
                    [#bytes] => #enum_name::#name(#args)
                });
            }
            None => {
                match_op.push(quote! {
                    [#bytes] => #enum_name::#name
                });
            }
        }
    }

    match_op
}


fn compute_args(fields: &Punctuated<Field, Comma>,
                values: &Punctuated<Type, Comma>,
                bytes: &Punctuated<LitInt, Comma>) -> proc_macro2::TokenStream {
    let mut result = Vec::new();

    for (i, value) in values.iter().enumerate() {
        match quote!{ #value }.to_string().as_str() {
            // 16 bit unsigned integer
            "nn" => {
                let v1 = bytes[i].base10_parse::<u8>().unwrap();
                let v2 = bytes[i + 1].base10_parse::<u8>().unwrap();
                let val = ((v1 as u16) << 8) | v2 as u16;
                let ty = &fields[i].ty;
                result.push(quote! { #val as #ty });
            },

            // 8bit signed/unsigned integer
            "n" | "d" => {
                let val = &bytes[i];
                let ty = &fields[i].ty;
                result.push(quote! { #val as #ty });
            },

            _ => result.push(quote! { #value })
        }
    }

    if fields.len() != result.len() {
        panic!("Binding's arguments doesn't match with variant fields.");
    }

    quote! { #(#result),* }
}


fn get_fields(variant: &Variant) -> Option<&Punctuated<Field, Comma>> {
    match &variant.fields {
        Fields::Unit => {
            None
        },
        Fields::Unnamed(fields) => {
            Some(&fields.unnamed)
        }
        Fields::Named(_) => {
            panic!("Named fields are not supported in YageInstructions derive");
        }
    }
}


fn parse_bindings(attrs: &Vec<Attribute>) -> Vec<Bind> {
    let mut bindings = Vec::new();

    for attr in attrs {
        let path = match attr.path.get_ident() {
            Some(ident) => ident.to_string(),
            None => continue
        };

        match path.as_str() {
            "bind" => {
                let bind: Bind = attr.parse_args().unwrap();

                bindings.push(bind);
            },

            _ => ()
        }
    }

    bindings
}
