extern crate proc_macro;

use std::path::PathBuf;
use litrs::Literal;
use quote::quote;

fn include_gresource_from_xml_with_filename(filename: &str) -> proc_macro2::TokenStream {
    let path = PathBuf::from(filename);
    let xml = gvdb::gresource::xml::GResourceXMLDoc::from_file(&path).unwrap();
    let builder = gvdb::gresource::builder::GResourceBuilder::from_xml(xml).unwrap();
    let data = builder.build().unwrap();

    let bytes = proc_macro2::Literal::byte_string(&data);

    quote! {
        #bytes
    }
}

fn include_gresource_from_xml_inner(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut iter = input.into_iter();

    let first = iter.next().expect("Expected exactly one string literal argument (gresource file location)");
    let second = iter.next();
    if let Some(second) = second {
        panic!("Unexpected token '{}', expected exactly one string literal argument (gresource file location)", second)
    }

    match Literal::try_from(first) {
        Err(e) => return proc_macro2::TokenStream::from(e.to_compile_error()),
        Ok(Literal::String(str)) => {
            include_gresource_from_xml_with_filename(str.value())
        }
        Ok(other) => panic!("Unexpected token '{:?}', expected exactly one string literal argument (gresource file location)", other)
    }
}

#[proc_macro]
pub fn include_gresource_from_xml(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = include_gresource_from_xml_inner(input);
    proc_macro::TokenStream::from(output)
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use crate::include_gresource_from_xml_inner;

    #[test]
    fn include_gresource_from_xml() {
        let tokens = quote! {
            "test/test3.gresource.xml"
        };

        let tokens = include_gresource_from_xml_inner(tokens);
        assert!(tokens.to_string().starts_with(r#"b"GVariant"#));
        assert!(tokens.to_string().ends_with("\""));
    }
}
