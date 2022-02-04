#![recursion_limit = "256"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate heck;
extern crate proc_macro2;

use heck::SnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::Ident;

struct Variant {
    key: Ident,
    value: String,
}

#[proc_macro_derive(DieselEnum)]
pub fn diesel_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = ast.ident;

    if let syn::Data::Enum(enum_data) = ast.data {
        let mut variants = Vec::new();
        for variant in enum_data.variants.into_iter() {
            variants.push(Variant {
                key: variant.ident.clone(),
                value: variant.ident.to_string().to_snake_case(),
            });
        }

        impl_diesel_enum(name, &variants)
    } else {
        panic!("#[derive(DieselEnum)] works with enums only!");
    }
}

fn impl_diesel_enum(name: Ident, variants: &[Variant]) -> TokenStream {
    let name_iter = std::iter::repeat(&name); // need an iterator for proc macro repeat pattern
    let name_iter1 = std::iter::repeat(&name);
    let name_iter2 = std::iter::repeat(&name);
    let name_iter3 = std::iter::repeat(&name);

    let scope = Ident::new(&format!("diesel_enum_{}", name), Span::call_site());

    let keys = &variants.iter().map(|v| v.key.clone()).collect::<Vec<_>>();
    let values = &variants.iter().map(|v| v.value.clone()).collect::<Vec<_>>();

    let expanded = quote! {
        mod #scope {
            use super::*;
            use diesel::{
                deserialize::{self, FromSql, FromSqlRow, Queryable},
                dsl::AsExprOf,
                expression::AsExpression,
                pg::Pg,
                row::Row,
                serialize::{self, IsNull, Output, ToSql},
                sql_types::{Nullable, VarChar},
            };
            use std::{error::Error, io::Write};

            impl #name {
                pub fn db_value(&self) -> &'static str {
                    match self {
                        #(#name_iter::#keys => #values,)*
                    }
                }
            }

            impl AsExpression<VarChar> for #name {
                type Expression = AsExprOf<String, VarChar>;
                fn as_expression(self) -> Self::Expression {
                    <String as AsExpression<VarChar>>::as_expression(self.db_value().to_string())
                }
            }

            impl<'a> AsExpression<VarChar> for &'a #name {
                type Expression = AsExprOf<String, VarChar>;
                fn as_expression(self) -> Self::Expression {
                    <String as AsExpression<VarChar>>::as_expression(self.db_value().to_string())
                }
            }

            impl AsExpression<Nullable<VarChar>> for #name {
                type Expression = AsExprOf<String, Nullable<VarChar>>;
                fn as_expression(self) -> Self::Expression {
                    <String as AsExpression<Nullable<VarChar>>>::as_expression(self.db_value().to_string())
                }
            }

            impl<'a> AsExpression<Nullable<VarChar>> for &'a #name {
                type Expression = AsExprOf<String, Nullable<VarChar>>;
                fn as_expression(self) -> Self::Expression {
                    <String as AsExpression<Nullable<VarChar>>>::as_expression(self.db_value().to_string())
                }
            }

            impl ToSql<VarChar, Pg> for #name {
                fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
                    match *self {
                        #(#name_iter1::#keys => out.write_all(#values.as_bytes())?,)*
                    }
                    Ok(IsNull::No)
                }
            }

            impl FromSql<VarChar, Pg> for #name {
                fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
                    match match bytes.map(|b| String::from_utf8_lossy(b).to_string()) {
                        Some(bytes) => bytes,
                        None => return Err(Box::new(::diesel::result::UnexpectedNullError)),
                    }
                    .as_ref()
                    {
                        #(#values => Ok(#name_iter2::#keys),)*
                        v => Err(format!("Unknown value {:?} for {}", v, stringify!(#name)).into()),
                    }
                }
            }

            impl FromSqlRow<VarChar, Pg> for #name {
                fn build_from_row<R: Row<Pg>>(row: &mut R) -> Result<Self, Box<Error + Send + Sync>> {
                    match String::build_from_row(row)?.as_ref() {
                        #(#values => Ok(#name_iter3::#keys),)*
                        v => Err(format!("Unknown value {} for {}", v, stringify!(#name)).into()),
                    }
                }
            }

            impl Queryable<VarChar, Pg> for #name {
                type Row = Self;

                fn build(row: Self::Row) -> Self {
                    row
                }
            }
        }
    };
    expanded.into()
}
