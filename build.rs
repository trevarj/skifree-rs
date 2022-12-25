use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use quote::__private::TokenStream;
use quote::{format_ident, quote};

fn main() {
    // Generate asset structs based on the files in assets/ dir
    println!("cargo:rerun-if-changed=assets");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("assets.rs");
    let mut assets_file = File::create(dest_path).expect("error creating generated file");

    let (_, assets_struct) = visit_dir(Path::new("assets"));

    // println!("cargo:warning={:#}", assets_struct.to_string());

    assets_file.write_all(b"// @generated\n\n").unwrap();
    assets_file
        .write_all(assets_struct.to_string().as_bytes())
        .expect("error writing struct to assets.rs file");
}

fn capitalize_first(s: &str) -> String {
    s.char_indices()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect()
}

fn project_dir() -> PathBuf {
    Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).to_path_buf()
}

fn visit_dir(path: &Path) -> (String, TokenStream) {
    let mut fields = vec![];
    let mut defs = vec![];
    let mut field_constructors = vec![];
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            let (field_type, def) = visit_dir(entry.path().as_path());
            defs.push(def);
            let field_name = format_ident!("{}", field_type.to_lowercase());
            let field_type = format_ident!("{field_type}");
            fields.push(quote! { pub #field_name: #field_type });
            field_constructors.push(quote! { #field_name: <#field_type>::new(ctx)? });
        } else {
            let field_name = format_ident!(
                "{}",
                entry
                    .file_name()
                    .to_string_lossy()
                    .trim_end_matches(".png")
                    .to_string()
            );

            let image_path = entry.path();
            let image_path = project_dir().join(image_path);
            let image_bytes = std::fs::read(&image_path).unwrap();
            fields.push(quote! { pub #field_name : std::rc::Rc<ggez::graphics::Image> });
            field_constructors
                .push(quote! { #field_name: std::rc::Rc::new(ggez::graphics::Image::from_bytes(ctx, &[ #(#image_bytes,)*])?) })
        }
    }
    let struct_type = capitalize_first(&path.file_name().unwrap().to_string_lossy());
    let struct_name = format_ident!("{struct_type}");
    (struct_type, quote! {
        pub struct #struct_name {
            #(#fields),*
        }

        impl #struct_name {
            pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<#struct_name> {
                Ok(
                    #struct_name {
                        #(#field_constructors),*
                    }
                )
            }
        }

        #(#defs)*
    })
}
