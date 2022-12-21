use std::ffi::OsStr;
use rocket::{tokio::{fs::File, io::AsyncReadExt}, http::{Status, ContentType}};
use std::path::Path;

use crate::util::{config::CONFIG, error_responder::ApiError, encryption::decrypt_with_key};

#[get("/<file_name>/<key>")]
pub async fn show_file(file_name: &str, key: &str) -> (ContentType, Result<Vec<u8>, ApiError>) {
    if file_name.chars().any(|c| !c.is_alphanumeric() && c != '.') {
        return (ContentType::JSON, Err(ApiError::new("Invalid file name", Status::BadRequest)));
    }

    let path = Path::new(&CONFIG.upload_folder).join(&file_name);
    let file = File::open(path).await;

    let mut file = match file {
        Ok(file) => file,
        Err(_) => return (ContentType::JSON, Err(ApiError::new("File not found", Status::NotFound)))
    };

    let extension = match Path::new(&file_name)
        .extension()
        .and_then(OsStr::to_str) {
            Some(extension) => extension.to_lowercase(),
            None => return (ContentType::JSON, Err(ApiError::new("Invalid extension", Status::BadRequest)))
        };

    // read file
    let mut contents = vec![];

    if file.read_to_end(&mut contents).await.is_err() {
        return (ContentType::JSON, Err(ApiError::new("Failed to read file", Status::InternalServerError)));
    }
    
    // decrypt file
    let decrypted = match decrypt_with_key(contents, key.to_string()) {
        Ok(decrypted) => decrypted,
        Err(err) => return (ContentType::JSON, Err(ApiError::new(&err, Status::BadRequest)))
    };
    
    return (ContentType::from_extension(&extension).unwrap_or(ContentType::Binary), Ok(decrypted));
}
