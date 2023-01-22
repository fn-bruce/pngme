#![allow(dead_code)]
#![allow(unused)]

use std::io::Write;
use regex::{Regex, Error};

enum DownloadError {
    Syntax(String),
    Reqwest(reqwest::Error),
    File(std::io::Error),
}

struct Url {
    url: String,
}

impl Url {
    fn new(url: &str) -> Result<Url, regex::Error> {
        let re = Regex::new(r"https?://.+\.(png|PNG)$")?;
        if re.is_match(url) {
            Ok(Url { url: url.to_string() })
        } else {
            Err(Error::Syntax("Invalid URL".to_string()))
        }
    }

    async fn download(&self, file_name: &str) -> Result<(), DownloadError> {
        let response = reqwest::get(&self.url).await.map_err(DownloadError::Reqwest)?;
        let bytes = response.bytes().await.map_err(DownloadError::Reqwest)?;
        let mut file = std::fs::File::create(file_name).map_err(DownloadError::File)?;
        file.write_all(&bytes).map_err(DownloadError::File)?;
        Ok(())
    }
}

// create tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_url() {
        let url = Url::new("https://www.rust-lang.org/logos/rust-logo-512x512.png");
        assert!(url.is_ok());
        let url = url.unwrap();
        let result = url.download("rust-logo.png").await;
        assert!(result.is_ok());
    }
}
