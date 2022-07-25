use std::collections::HashMap;
use std::env::temp_dir;
use std::fs;
use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use thiserror::Error;
use std::path::{Path, PathBuf};
use log::debug;
use reqwest::ClientBuilder;
use bytes::Bytes;
use once_cell::sync::Lazy;
use serde::Deserialize;
use futures_util::StreamExt;

#[derive(Error, Debug)]
pub enum IconLoaderError {
    #[error("Failure to load due to Github Issue: {0}")]
    GithubIssue(#[from] octocrab::Error),
    #[error("Zip Error: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Request Error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Json Error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub static TITLE_TO_SLUG_REPLACEMENTS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("+", "plus");
    map.insert(".", "dot");
    map.insert("&", "and");
    map.insert("đ", "d");
    map.insert("ã", "a");
    map.insert("ż", "z");
    map.insert("š", "s");
    map.insert("ħ", "h");
    map.insert("ı", "i");
    map.insert("ĸ", "k");
    map.insert("ŀ", "1");
    map.insert("ł", "1");
    map.insert("ß", "ss");
    map.insert("°", "");
    map.insert("ŧ", "t");
    map.insert("/", "");
    map.insert(" ", "");
    map.insert(":", "");
    map.insert("_", "");
    map.insert("!", "");
    map.insert("-", "");
    map.insert("é", "e");
    map.insert("é", "e");
    map.insert("ü", "u");
    map.insert("ü", "u");
    map.insert("'", "");
    map.insert("ë", "e");
    map.insert("è", "e");
    map.insert("è", "e");
    map.insert("è", "e");
    map
});

#[derive(Debug, Clone, Deserialize)]
pub struct IconData {
    pub title: String,
    pub source: String,
    pub hex: String,
    pub guidelines: Option<String>,
    pub slug: Option<String>,
    pub license: Option<IconLicense>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct IconLicense {
    #[serde(rename = "type")]
    pub license_type: Option<String>,
    pub url: Option<String>,

}
impl IconData {
    pub fn convert_title_to_slug(&self) -> String {
        if let Some(slug) = &self.slug {
            slug.clone()
        } else {
            let mut slug = self.title.to_lowercase().to_string();
            for (key, value) in TITLE_TO_SLUG_REPLACEMENTS.iter() {
                slug = slug.replace(*key, value);
            }
            slug
        }
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct IconDataFile {
    pub icons: Vec<IconData>,
}

pub struct SimpleIconLoader {
    pub icons_folder: PathBuf,
}

impl SimpleIconLoader {
    pub async fn load_icons(version: impl AsRef<str>) -> Result<SimpleIconLoader, IconLoaderError> {
        let buf = temp_dir();
        Self::load_icons_with_cache(version, buf).await
    }
    pub async fn load_icons_with_cache<P: AsRef<Path>>(version: impl AsRef<str>, cache_location: P) -> Result<SimpleIconLoader, IconLoaderError> {
        let icon_directory = cache_location.as_ref().join(version.as_ref());
        if icon_directory.exists() {
            return Ok(SimpleIconLoader {
                icons_folder: icon_directory,
            });
        }
        let arc = octocrab::instance();
        let release = arc.repos("simple-icons", "simple-icons").releases().get_by_tag(version.as_ref()).await?;
        let asset = release.zipball_url.unwrap();
        let buf = cache_location.as_ref().join(format!("{}.zip", version.as_ref()));
        download(asset, buf.clone()).await?;
        extract(&icon_directory, buf).await?;


        Ok(SimpleIconLoader {
            icons_folder: icon_directory,
        })
    }
    pub fn get_icons_file(&self) -> Result<IconDataFile, IconLoaderError> {
        let buf = self.icons_folder.join("_data").join("simple-icons.json");
        let files = OpenOptions::new().read(true).open(&buf)?;
        serde_json::from_reader(files).map_err(|e| IconLoaderError::JsonError(e))
    }
    pub fn get_icon_path(&self, data: &IconData) -> PathBuf {
        self.icons_folder.join("icons").join(format!("{}.svg", data.convert_title_to_slug()))
    }
    pub fn get_icon_path_from_name(&self, slug: impl AsRef<str>) -> PathBuf {
        self.icons_folder.join("icons").join(format!("{}.svg", slug.as_ref()))
    }

}


pub async fn download(url: reqwest::Url, location: PathBuf) -> Result<(), IconLoaderError> {
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
    let mut file = OpenOptions::new().create_new(true).write(true).open(&location)?;
    let mut stream = source.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk: Bytes = item?;
        file.write_all(chunk.as_ref())?;
    }
    Ok(())
}

async fn extract(
    extract_to: impl AsRef<Path>,
    archive: impl AsRef<Path>,
) -> Result<(), IconLoaderError>
{
    let file = std::fs::File::open(&archive)?;

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                let components = path.components();
                let buf = components.skip(1).fold(PathBuf::default(), |buf, component| {
                    buf.join(component)
                });
                extract_to.as_ref().join(buf)
            }
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            debug!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            debug!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

#[cfg(test)]
pub mod test {
    use std::path::PathBuf;
    use crate::icon_loader::{SimpleIconLoader};

    #[tokio::test]
    pub async fn download_test() {
        let test_dir = PathBuf::new().join("test");
        if test_dir.exists() {
            std::fs::remove_dir_all(&test_dir).unwrap();
        }
        std::fs::create_dir_all(&test_dir).unwrap();
        let icons = SimpleIconLoader::load_icons_with_cache("7.5.0", test_dir).await.unwrap();
    }

    #[tokio::test]
    pub async fn slug_convert_test() {
        let test_dir = PathBuf::new().join("test");
        let loader = SimpleIconLoader::load_icons_with_cache("7.5.0", test_dir).await.unwrap();
        let icons = loader.get_icons_file().unwrap();
        for icon in icons.icons {
            let buf = loader.get_icon_path(&icon);
            if !buf.exists() {
                panic!("{} slug {} not found path {}", icon.title, icon.convert_title_to_slug(), buf.display());
            }
        }
    }

}