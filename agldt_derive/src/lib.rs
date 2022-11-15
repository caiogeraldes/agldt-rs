use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Ident, Lit, Meta,
    MetaList, NestedMeta, Variant,
};

fn get_variants(data: &DataEnum) -> Vec<&Variant> {
    let DataEnum { variants, .. } = data;
    let mut vv: Vec<&Variant> = vec![];
    for variant in variants {
        vv.push(variant);
    }
    vv
}

fn get_fields(data: &DataStruct) -> Vec<&Field> {
    let DataStruct { fields, .. } = data;
    let mut vf: Vec<&Field> = vec![];
    for field in fields {
        vf.push(field)
    }
    vf
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
            unimplemented!();
        } else {
            let attr = &variant.attrs[0].parse_meta().unwrap();
            match attr {
                Meta::Path(_) => {
                    unimplemented!();
                }
                Meta::NameValue(_) => {
                    unimplemented!();
                }
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
            }
        }
    }
    vc
}

fn gen_postag(variant: &Variant) -> char {
    let ident = variant.ident.to_string();
    return ident.to_lowercase().chars().next().unwrap();
}

fn get_index(attrs: &Vec<Attribute>) -> u8 {
    if attrs.is_empty() {
        return 0;
    }
    if attrs.len() > 1 {
        unimplemented!()
    } else {
        let attr = attrs[0].parse_meta().unwrap();
        match attr {
            Meta::Path(_) => {
                unimplemented!();
            }
            Meta::NameValue(_) => {
                unimplemented!();
            }
            Meta::List(MetaList { path, nested, .. }) => {
                assert_eq!(path.segments[0].ident, "postagindex");
                if let NestedMeta::Lit(l) = &nested[0] {
                    if let Lit::Int(i) = l {
                        let id: u8 = i.token().to_string().parse().unwrap();
                        return id;
                    } else {
                        panic!("epa");
                    }
                } else {
                    panic!("epa");
                }
            }
        }
    }
}

#[proc_macro_derive(PostagFeature, attributes(postag, postagindex))]
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
                use agldt::PostagFeature;
                use agldt::POSFeature;

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
        Data::Struct(structdata) => {
            let fields = get_fields(&structdata);
            dbg!(fields);
            unimplemented!()
        }
        _ => unimplemented!(),
    };
    TokenStream::from(expanded)
}
