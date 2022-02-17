extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::visit::{self, Visit};

#[proc_macro_derive(NTupleNewtype)]
pub fn ntuple_newtype_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_ntuple_newtype(&ast)
}

fn impl_ntuple_newtype(ast: &syn::DeriveInput) -> TokenStream {
    let mut visitor = DeriveVisitor {
        ident: None,
        nttp: None,
        gargs: None,
    };
    visitor.visit_derive_input(ast);
    let ident = visitor.ident.unwrap();
    let nttp = visitor.nttp.unwrap();
    let gargs = visitor.gargs.unwrap();
    let gen = quote! {
        impl NTupleNewtype #gargs for #ident {
            fn ntuple(&self) -> #nttp {
                self.0
            }
        }

        impl std::convert::From< #nttp > for #ident {
            fn from(ntuple: #nttp) -> #ident {
                Self(ntuple)
            }
        }

        impl std::convert::From< #ident > for #nttp {
            fn from(i: #ident) -> #nttp {
                i.ntuple()
            }
        }
    };
    gen.into()
}

fn panic() {
    panic!("NTupleNewtype can only be derived on newtype structs of NTuple");
}

struct DeriveVisitor<'ast> {
    pub ident: Option<&'ast syn::Ident>,
    pub nttp: Option<&'ast syn::TypePath>,
    pub gargs: Option<&'ast syn::AngleBracketedGenericArguments>,
}

impl<'ast> syn::visit::Visit<'ast> for DeriveVisitor<'ast> {
    fn visit_derive_input(&mut self, node: &'ast syn::DeriveInput) {
        self.ident = Some(&node.ident);
        visit::visit_data(self, &node.data);
    }

    fn visit_data(&mut self, node: &'ast syn::Data) {
        if let syn::Data::Struct(ds) = node {
            visit::visit_data_struct(self, &ds);
        } else {
            panic();
        }
    }

    fn visit_fields(&mut self, node: &'ast syn::Fields) {
        if let syn::Fields::Unnamed(fu) = node {
            visit::visit_fields_unnamed(self, &fu);
        } else {
            panic();
        }
    }

    fn visit_fields_unnamed(&mut self, node: &'ast syn::FieldsUnnamed) {
        let fields = &node.unnamed;
        if fields.len() != 1 {
            panic();
        } else {
            visit::visit_field(self, fields.first().unwrap());
        }
    }

    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        let last = node.path.segments.last().unwrap();
        if last.ident.to_string() != "NTuple" {
            panic();
        }
        self.nttp = Some(node);
        visit::visit_path(self, &node.path);
    }

    fn visit_angle_bracketed_generic_arguments(&mut self, node: &'ast syn::AngleBracketedGenericArguments) {
        self.gargs = Some(node);
    }
}
