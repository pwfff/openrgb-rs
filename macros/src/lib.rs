extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashMap;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use syn::{
    parse, parse_macro_input, Attribute, Expr, ExprAssign, ExprPath, Fields, Ident, ItemStruct,
    Pat, PatSlice, Path, Result, Token,
};

#[proc_macro_attribute]
pub fn impl_packet(args: TokenStream, input: TokenStream) -> TokenStream {
    let decl = parse_macro_input!(input as ItemStruct);
    let body = parse_macro_input!(args as Ident);
    let ident = decl.ident.clone();

    // Error if the struct doesn't name its fields.
    match decl.clone().fields {
        Fields::Named(_) => (),
        _ => {
            return parse::Error::new(ident.span(), "`#[impl_packet]` type must name fields")
                .to_compile_error()
                .into()
        }
    };

    let mut out = quote! {
        #decl

        #[async_trait]
        impl Packet for #ident {
            async fn read(
                header: Box<Header>,
                stream: &mut impl OpenRGBReadableStream,
                protocol: u32,
            ) -> Result<Self, OpenRGBError> {
                let body = #body::read(stream, protocol).await?;
                Ok(Self { header: *header, body })
            }
            fn header(&self) -> &Header {
                &self.header
            }
        }
    };

    TokenStream::from(out)
}

// #[derive(Debug, Clone, Default)]
// struct GroupSpec {
//     pub fields: HashMap<String, Spec>,
//     pub field_order: Vec<String>,

//     pub report_id: Option<u32>,
//     pub usage_page: Option<u32>,
//     pub collection: Option<u32>,
//     pub logical_min: Option<u32>,

//     // Local items
//     pub usage: Vec<u32>,
//     pub usage_min: Option<u32>,
//     pub usage_max: Option<u32>,
// }

// impl Parse for GroupSpec {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let mut out = GroupSpec {
//             ..Default::default()
//         };
//         let fields: Punctuated<Expr, Token![,]> = input.parse_terminated(Expr::parse)?;
//         if fields.len() == 0 {
//             return Err(parse::Error::new(
//                 input.span(),
//                 "`#[gen_hid_descriptor]` expected information about the HID report",
//             ));
//         }
//         for field in fields {
//             if let Err(e) = out.from_field(input, field) {
//                 return Err(e);
//             }
//         }
//         Ok(out)
//     }
// }

// impl GroupSpec {
//     pub fn set_item(
//         &mut self,
//         name: String,
//         item_kind: MainItemKind,
//         settings: Option<MainItemSetting>,
//         bits: Option<u16>,
//         quirks: ItemQuirks,
//     ) {
//         if let Some(field) = self.fields.get_mut(&name) {
//             if let Spec::MainItem(field) = field {
//                 field.kind = item_kind;
//                 field.settings = settings;
//                 field.want_bits = bits;
//             }
//         } else {
//             self.fields.insert(
//                 name.clone(),
//                 Spec::MainItem(ItemSpec {
//                     kind: item_kind,
//                     settings: settings,
//                     want_bits: bits,
//                     quirks: quirks,
//                     ..Default::default()
//                 }),
//             );
//             self.field_order.push(name);
//         }
//     }

//     pub fn add_nested_group(&mut self, ng: GroupSpec) {
//         let name = (0..self.fields.len() + 1).map(|_| "_").collect::<String>();
//         self.fields.insert(name.clone(), Spec::Collection(ng));
//         self.field_order.push(name);
//     }

//     pub fn get(&self, name: String) -> Option<&Spec> {
//         self.fields.get(&name)
//     }

//     pub fn try_set_attr(&mut self, input: ParseStream, name: String, val: u32) -> Result<()> {
//         match name.as_str() {
//             "report_id" => {
//                 self.report_id = Some(val);
//                 Ok(())
//             }
//             "usage_page" => {
//                 self.usage_page = Some(val);
//                 Ok(())
//             }
//             "collection" => {
//                 self.collection = Some(val);
//                 Ok(())
//             }
//             // Local items.
//             "usage" => {
//                 self.usage.push(val);
//                 Ok(())
//             }
//             "usage_min" => {
//                 self.usage_min = Some(val);
//                 Ok(())
//             }
//             "usage_max" => {
//                 self.usage_max = Some(val);
//                 Ok(())
//             }
//             "logical_min" => {
//                 self.logical_min = Some(val);
//                 Ok(())
//             }
//             _ => Err(parse::Error::new(
//                 input.span(),
//                 format!(
//                     "`#[gen_hid_descriptor]` unknown group spec key: {}",
//                     name.clone()
//                 ),
//             )),
//         }
//     }
// }

// impl IntoIterator for GroupSpec {
//     type Item = String;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.field_order.into_iter()
//     }
// }
