use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

#[proc_macro_derive(SqlModel, attributes(sql))]
pub fn derive_sql_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // insert(...) => SqlNew
    let mut insert_fields: Vec<Ident> = Vec::new();

    // raw(...) => SqlRaw (Row -> RawStruct)
    let mut raw_sql_fields: Vec<(Ident, usize)> = Vec::new();

    // raw_type = Tipo => FromRaw<Tipo> para Schema
    let mut raw_type: Option<Type> = None;
    // raw(...) con raw_type presente => campos para FromRaw
    let mut raw_from_fields: Vec<Ident> = Vec::new();

    // Parse attributes: #[sql(insert(...), raw(...), raw_type = ...)]
    for attr in input.attrs.iter().filter(|a| a.path().is_ident("sql")) {
        if let Err(err) = attr.parse_nested_meta(|meta| {
            // #[sql(insert(...))]
            if meta.path.is_ident("insert") {
                meta.parse_nested_meta(|inner| {
                    if let Some(ident) = inner.path.get_ident() {
                        insert_fields.push(ident.clone());
                        Ok(())
                    } else {
                        Err(inner.error("expected identifier in insert(...)"))
                    }
                })?;
            }
            // #[sql(raw_type = "Ruta::AlTipo", ...)]
            else if meta.path.is_ident("raw_type") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                let ty: Type =
                    syn::parse_str(&lit.value()).expect("raw_type debe ser un tipo válido");
                raw_type = Some(ty);
            }
            // #[sql(raw(...))]
            else if meta.path.is_ident("raw") {
                if raw_type.is_some() {
                    // Caso Schema: raw(...) indica qué campos vienen de Raw
                    meta.parse_nested_meta(|inner| {
                        if let Some(ident) = inner.path.get_ident() {
                            raw_from_fields.push(ident.clone());
                            Ok(())
                        } else {
                            Err(inner.error("expected identifier in raw(...) for FromRaw"))
                        }
                    })?;
                } else {
                    // Caso RawXxxSchema: raw(...) es Row -> Raw
                    let mut idx = 0usize;
                    meta.parse_nested_meta(|inner| {
                        if let Some(ident) = inner.path.get_ident() {
                            raw_sql_fields.push((ident.clone(), idx));
                            idx += 1;
                            Ok(())
                        } else {
                            Err(inner.error("expected identifier in raw(...) for SqlRaw"))
                        }
                    })?;
                }
            }
            Ok(())
        }) {
            return err.to_compile_error().into();
        }
    }

    // --- impl SqlNew ---
    let sql_new_impl = if !insert_fields.is_empty() {
        // Generar tipos: ( &'a dyn ToSql, &'a dyn ToSql, ... )
        let param_types = insert_fields.iter().map(|_| {
            quote! { &'a dyn rusqlite::ToSql }
        });

        // Generar valores: ( &self.codigo, &self.name, ... )
        let param_values = insert_fields.iter().map(|f| {
            quote! { &self.#f }
        });

        quote! {
            impl sql_model::SqlNew for #struct_name {
                type Params<'a> = ( #( #param_types ),* )
                where Self: 'a;

                fn to_params<'a>(&'a self) -> Self::Params<'a> {
                    ( #( #param_values ),* )
                }
            }
        }
    } else {
        quote! {}
    };

    // --- impl SqlRaw (Row -> RawStruct) ---
    let sql_raw_impl = if !raw_sql_fields.is_empty() {
        let extracts = raw_sql_fields.iter().map(|(name, idx)| {
            quote! { #name: r.get(#idx)? }
        });

        quote! {
            impl sql_model::SqlRaw for #struct_name {
                fn from_sql(r: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
                    Ok(Self {
                        #( #extracts ),*
                    })
                }
            }
        }
    } else {
        quote! {}
    };

    // --- impl FromRaw<RawType> para Schema ---
    let from_raw_impl = if let Some(raw_ty) = raw_type {
        if !raw_from_fields.is_empty() {
            // Generamos:
            // let created_at = string_2_datetime(Some(r.created_at)).unwrap();
            // let deleted_at = string_2_datetime(r.deleted_at);
            // y luego:
            // Ok(Self { id: r.id, code: r.code, ..., created_at, deleted_at })
            let mut prep_stmts = Vec::new();
            let mut field_inits = Vec::new();

            for field in &raw_from_fields {
                let name_str = field.to_string();
                if name_str == "created_at" {
                    prep_stmts.push(quote! {
                        let created_at = sql_model::string_2_datetime(Some(r.created_at)).unwrap();
                    });
                    field_inits.push(quote! { created_at: created_at });
                } else if name_str == "deleted_at" {
                    prep_stmts.push(quote! {
                        let deleted_at = sql_model::string_2_datetime(r.deleted_at);
                    });
                    field_inits.push(quote! { deleted_at: deleted_at });
                } else {
                    field_inits.push(quote! { #field: r.#field });
                }
            }

            quote! {
                impl sql_model::FromRaw<#raw_ty> for #struct_name {
                    #[inline]
                    fn from_raw(r: #raw_ty) -> sql_model::ModelResult<Self> {
                        #(#prep_stmts)*

                        Ok(Self {
                            #( #field_inits ),*
                        })
                    }
                }
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #sql_new_impl
        #sql_raw_impl
        #from_raw_impl
    };

    TokenStream::from(expanded)
}
