use crate::Error;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
pub enum DownloadError {
    InvalidUrl(String),
    DownloadFailed,
}

impl Default for DownloadError {
    fn default() -> Self {
        Self::DownloadFailed
    }
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            DownloadError::DownloadFailed => write!(f, "Download failed"),
        }
    }
}

impl std::error::Error for DownloadError {}

pub struct Download(String);

impl FromStr for Download {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Download::new(s)
    }
}

impl Download {
    pub fn new(url: &str) -> Result<Self, Error> {
        let re = Regex::new(r"https?://")?;
        if re.is_match(url) {
            Ok(Download(url.to_string()))
        } else {
            Err(Box::new(DownloadError::InvalidUrl(url.to_string())))
        }
    }

    pub async fn download(&self, filename: &str) -> Result<(), Error> {
        let response = reqwest::get(&self.0).await?;
        if response.status().is_success() {
            let bytes = response.bytes().await?;
            std::fs::write(filename, bytes)?;
            Ok(())
        } else {
            Err(Box::new(DownloadError::DownloadFailed))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_png() {
        let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
        let filename = "rust-logo.png";
        let download = Download::from_str(url).unwrap();
        download.download(filename).await.unwrap();
    }
}
