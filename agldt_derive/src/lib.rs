use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::{
    Attribute, Data, DataEnum, DeriveInput, Ident, Lit, Meta, MetaList, NestedMeta, Variant,
};

fn get_variants(data: &DataEnum) -> Vec<&Variant> {
    let DataEnum { variants, .. } = data;
    let mut vv: Vec<&Variant> = vec![];
    for variant in variants {
        vv.push(variant);
    }
    vv
}

fn gen_postags(variants: Vec<&Variant>) -> Vec<char> {
    let mut vc: Vec<char> = vec![];
    for variant in variants.iter() {
        if variant.attrs.is_empty() {
            let pt = gen_postag(&variant);
            if vc.contains(&pt) {
                panic!("Two variants tried to use the same postag value: {} ", pt);
            } else {
                vc.push(pt);
            }
        } else if variant.attrs.len() > 1 {
            unimplemented!("Only one variant implemented");
        } else {
            let attr = &variant.attrs[0].parse_meta().unwrap();
            match attr {
                Meta::List(MetaList { path, nested, .. }) => {
                    assert_eq!(path.segments[0].ident, "postag");
                    if let NestedMeta::Lit(l) = &nested[0] {
                        if let Lit::Char(c) = l {
                            let pt = c
                                .token()
                                .to_string()
                                .replace('\'', "")
                                .replace('\\', "")
                                .chars()
                                .next()
                                .unwrap();
                            vc.push(pt);
                        }
                    }
                }
                _ => unimplemented!("Invalid meta"),
            }
        }
    }
    vc
}

fn gen_postag(variant: &Variant) -> char {
    let ident = variant.ident.to_string();
    if ident == "EMPTY" {
        return '-';
    }
    return ident.to_lowercase().chars().next().unwrap();
}

fn get_index(attrs: &Vec<Attribute>) -> u8 {
    if attrs.is_empty() {
        return 0;
    }
    if attrs.len() > 1 {
        let attr = attrs[0].parse_meta().unwrap();
        match attr {
            Meta::List(MetaList { path, nested, .. }) => {
                if path.segments[0].ident == "postagindex" {
                    let id = get_nested_id(&nested[0]);
                    id
                } else if path.segments[0].ident == "complexfeature" {
                    let attr = attrs[1].parse_meta().unwrap();
                    if let Meta::List(MetaList { path, nested, .. }) = attr {
                        assert_eq!(path.segments[0].ident, "postagindex");
                        let id = get_nested_id(&nested[0]);
                        id
                    } else {
                        panic!("{:#?}", attr);
                    }
                } else {
                    unimplemented!("Invalid attribute.");
                }
            }
            _ => unimplemented!("Invalid meta."),
        }
    } else {
        let attr = attrs[0].parse_meta().unwrap();
        match attr {
            Meta::List(MetaList { path, nested, .. }) => {
                assert_eq!(path.segments[0].ident, "postagindex");
                let id = get_nested_id(&nested[0]);
                id
            }
            _ => unimplemented!("Invalid meta."),
        }
    }
}

fn get_nested_id(nested: &NestedMeta) -> u8 {
    if let NestedMeta::Lit(l) = &nested {
        if let Lit::Int(i) = l {
            let id: u8 = i.token().to_string().parse().unwrap();
            return id;
        } else {
            panic!("{:#?}", l);
        }
    } else {
        panic!("epa");
    }
}

#[proc_macro_derive(PostagFeature, attributes(postag, postagindex, complexfeature))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let main_name = &ast.ident;
    let index = get_index(&ast.attrs);

    let expanded: proc_macro2::TokenStream = match &ast.data {
        Data::Enum(enumdata) => {
            let variants = get_variants(&enumdata);
            let variants_names = variants.iter().map(|v| &v.ident).collect::<Vec<&Ident>>();
            let postags: Vec<char> = gen_postags(variants);

            quote!(

                impl PostagFeature for #main_name {
                    fn to_agldt_postag(&self) -> POSFeature {
                        match self {
                            #(Self::#variants_names => POSFeature::new(#index, #postags).unwrap(),)*
                        }
                    }
                    fn to_string(&self) -> String {
                        format!("{}", self.to_agldt_postag())
                    }
                }
            )
        }
        _ => unimplemented!("Derive only implemented for Enums."),
    };
    TokenStream::from(expanded)
}
