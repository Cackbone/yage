use proc_macro::TokenStream;
use syn::{ ItemEnum, Variant, Fields, Attribute,
           Token, token::{ Comma },
           TypeTuple, LitInt, Result,
           parse_macro_input, parse::{Parse, ParseStream}};
use quote::quote;

// mod opcode;
// use opcode::Opcode;

mod yage_kw {
    syn::custom_keyword!(opcode);
    syn::custom_keyword!(args);
}

struct Op {
    name: yage_kw::opcode,
    eq_token: Token![=],
    value: LitInt
}

struct Args {
    name: yage_kw::args,
    eq_token: Token![=],
    args: TypeTuple
}

struct Bind {
    opcode: Op,
    comma_token: Comma,
    args: Args
}


impl Parse for Bind {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Bind {
            opcode: input.parse()?,
            comma_token: input.parse()?,
            args: input.parse()?
        })
    }
}

impl Parse for Op {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Op {
            name: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?
        })
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Args {
            name: input.parse()?,
            eq_token: input.parse()?,
            args: input.parse()?
        })
    }
}


#[proc_macro_derive(YageInstructions, attributes(bind))]
pub fn instructions(input: TokenStream) -> TokenStream {
    let enumItem: ItemEnum = syn::parse(input).expect("Instructions derive only works on enums");
    let enumName = &enumItem.ident;
    let mut structs = Vec::new();

    for variant in &enumItem.variants {
        let vStruct = create_struct(variant);
        let bindings = parse_bindings(&variant.attrs);

        structs.push(vStruct);
    }

    TokenStream::from(quote! {
        #(#structs)*
    })
}


fn create_struct(variant: &Variant) -> proc_macro2::TokenStream {
    let name = &variant.ident;
    let variant_struct = match &variant.fields {
        Fields::Unnamed(fields) => {
            quote! {
                struct #name(#fields);
            }
        },
        Fields::Unit => {
            quote! {
                struct #name;
            }
        },
        Fields::Named(_) => {
            panic!("Named field are not supported in Instructions derive");
        }
    };

    return variant_struct;
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
