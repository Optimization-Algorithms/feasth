use reqwest;
use scraper::{Html, Selector};

use crate::error;

use std::io::{Error, ErrorKind};
use std::path::Path;

const DEF_TAIL: &str = "-init.csv";

pub async fn search_model_size(log_name: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let log_name = get_path_name(log_name)?;
    if let Some(name) = derive_model_name(log_name) {
        if let Some(value) = get_model_size(name).await? {
            Ok(value)
        } else {
            Err(error::AutoGetSizeError::SizeNotFound(name.to_owned()))?
        }
    } else {
        Err(error::AutoGetSizeError::WrongFormat(log_name.to_owned()))?
    }
}

fn derive_model_name(name: &str) -> Option<&str> {
    if name.ends_with(DEF_TAIL) {
        let index = name.len() - DEF_TAIL.len();
        Some(&name[..index])
    } else {
        None
    }
}

fn get_path_name(log_name: &Path) -> Result<&str, Box<dyn std::error::Error>> {
    if let Some(log_name) = log_name.file_name() {
        if let Some(name) = log_name.to_str() {
            Ok(name)
        } else {
            Err(error::PathConversionError {})?
        }
    } else {
        let err = Error::new(
            ErrorKind::InvalidData,
            format!("cannot get file name from path {:?}", log_name),
        );
        Err(err)?
    }
}

async fn get_model_size(name: &str) -> Result<Option<usize>, Box<dyn std::error::Error>> {
    let html = get_instance_page(name).await?;
    Ok(get_model_size_from_html(&html))
}

async fn get_instance_page(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://miplib.zib.de/instance_details_{}.html", name);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        Err(error::GetError::new(url, response.status()))?
    }
}

fn get_model_size_from_html(html: &str) -> Option<usize> {
    let html = Html::parse_document(&html);
    let select = Selector::parse("td").unwrap();
    let mut status = false;

    for entry in html.select(&select) {
        let inner = entry.inner_html();
        if status {
            let output = if let Ok(i) = inner.parse() {
                Some(i)
            } else {
                None
            };
            return output;
        } else if inner == "Variables" {
            status = true;
        }
    }
    None
}
