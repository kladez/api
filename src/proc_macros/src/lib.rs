use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    DeriveInput,
};

fn snake_to_camel(snake_str: &str) -> String {
    let mut camel_str = String::new();
    let mut capitalize_next = true;

    for c in snake_str.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            camel_str.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            camel_str.push(c);
        }
    }

    camel_str
}

#[proc_macro_derive(FieldEnum)]
pub fn generate_field_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let data = if let syn::Data::Struct(data) = input.data {
        data
    } else {
        panic!("Only works on structs");
    };

    let enum_name = syn::Ident::new(&format!("{}Field", struct_name), struct_name.span());

    let enum_variants: Vec<_> = data.fields.iter().filter_map(|f| {
        f.ident.as_ref().map(|field_name| {
            let field_str = field_name.to_string();
            let variant_name = snake_to_camel(&field_str);
            syn::Ident::new(&variant_name, field_name.span())
        })
    }).collect();

    let expanded = quote! {
        #[derive(Debug, poem_openapi::Enum, serde::Deserialize)]
        pub enum #enum_name {
            #(#enum_variants,)*
        }

        impl #enum_name {
            pub fn to_string(&self) -> String {
                match self {
                    #(#enum_name::#enum_variants => stringify!(#enum_variants).to_string(),)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
