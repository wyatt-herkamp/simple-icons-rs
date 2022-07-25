use std::{fs, io};
use std::fs::{DirEntry, File, remove_file};
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use convert_case::{Case, Casing};
use genco::{quote, Tokens};
use genco::lang::Rust;
use tokio::fs::{create_dir_all, OpenOptions, read_to_string, remove_dir_all};
use tokio::io::{AsyncWriteExt, BufReader, duplex};
use genco::tokens::quoted;
use simple_icons_rs::icon_loader::{SimpleIconLoader};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct SimpleIconBuilder {
    #[clap(long, short)]
    pub version: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let builder = SimpleIconBuilder::parse();
    let loader = SimpleIconLoader::load_icons(builder.version.as_str()).await?;


    let mut icons_dir = loader.icons_folder.join("icons");
    let output_dir = PathBuf::from("../src/built_icons");
    if !output_dir.exists() {
        create_dir_all(&output_dir).await?;
    }
    let mut modules = Vec::<Tokens<Rust>>::new();
    for icon in icons_dir.read_dir().unwrap() {
        let icon = icon.unwrap();
        let icon_content = read_to_string(icon.path()).await?;
        let string = format!("Icon{}", icon.file_name().to_str().unwrap().replace(".svg", "").to_case(Case::UpperCamel));
        let struct_name = string.to_case(Case::UpperCamel);
        let module_name = string.to_case(Case::Snake);
        let mut options = OpenOptions::new().create_new(true).write(true).open(output_dir.join(format!("{}.rs", module_name))).await?;
        let name = struct_name.as_str();
        let mod_n = module_name.as_str();
        let value: Tokens<Rust> = quote! {
                    use crate::SimpleIcon;
                    pub struct $name;
                    impl Default for $name {
                        fn default() -> Self {
                            $name
                        }
                    }
                    impl SimpleIcon for $name {
                        fn icon(&self) -> &'static str {
                            $(quoted(icon_content))
                        }
                    }
                };
        let content: String = value.to_file_string().unwrap();

        options.write_all(content.as_bytes()).await?;
        modules.push(quote! {
                    mod $mod_n;
                    pub use $mod_n::$name;
                });
    }
    let mut options = OpenOptions::new().create_new(true).write(true).open(output_dir.join("mod.rs")).await?;
    for module in modules {
        let string1 = module.to_file_string().unwrap();
        options.write_all(string1.as_bytes()).await?;
    }


    Ok(())
}
