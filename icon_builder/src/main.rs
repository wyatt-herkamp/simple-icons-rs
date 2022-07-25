use std::{fs, io};
use std::fs::{DirEntry, File, remove_file};
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use bytes::Bytes;
use clap::Parser;
use convert_case::{Case, Casing};
use futures_util::StreamExt;
use genco::{quote, Tokens};
use genco::lang::Rust;
use reqwest::{ClientBuilder, Url};
use tokio::fs::{create_dir_all, OpenOptions, read_to_string, remove_dir_all};
use tokio::io::{AsyncWriteExt, BufReader, duplex};
use zip::result::ZipError;
use zip::write::FileOptions;
use genco::tokens::quoted;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct SimpleIconBuilder {
    #[clap(long, short)]
    pub version: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let builder = SimpleIconBuilder::parse();

    let arc = octocrab::instance();
    let release = arc.repos("simple-icons", "simple-icons").releases().get_by_tag(&builder.version).await;
    if let Ok(release) = release {
        let dir = PathBuf::new().join(&builder.version);
        if !dir.exists() {
            let asset = release.zipball_url.unwrap();
            let buf = PathBuf::new().join(format!("{}.zip", builder.version));
            download(asset, buf.clone()).await?;
            create_dir_all(&dir).await?;
            extract(dir.clone(), buf)?;
        }
        let mut icons_dir = None;
        for x in dir.read_dir().unwrap() {
            let entry = x.unwrap();
            let icons = entry.path().join("icons");
            icons_dir = Some(icons);
            break;
        }
        let output_dir = PathBuf::from("../src/built_icons");
        if !output_dir.exists() {
            create_dir_all(&output_dir).await?;
        }
        let mut modules = Vec::<Tokens<Rust>>::new();
        if let Some(icons) = icons_dir {
            for icon in icons.read_dir().unwrap() {
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
        }
    } else {
        println!("Release Not Found {}", builder.version);
    }
    Ok(())
}


pub async fn download(url: Url, location: PathBuf) -> anyhow::Result<()> {
    let client = ClientBuilder::new()
        .user_agent("simple-icons-builder by github.com/wyatt-herkamp")
        .build()?;
    if location.exists() {
        remove_file(&location)?;
    }
    let source = client.get(url).send().await?;
    if !source.status().is_success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download").into());
    }
    let mut file = OpenOptions::new().create_new(true).write(true).open(&location).await?;
    let mut stream = source.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk: Bytes = item.unwrap();
        file.write_all(chunk.as_ref()).await?;
    }
    Ok(())
}

/// Code From https://github.com/zip-rs/zip/blob/master/examples/write_dir.rs
fn extract(
    extract_to: PathBuf,
    archive: PathBuf,
) -> zip::result::ZipResult<()>
{
    let file = fs::File::open(&archive).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => extract_to.join(path),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    Ok(())
}

