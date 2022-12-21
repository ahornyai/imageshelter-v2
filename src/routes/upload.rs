use std::ffi::OsStr;
use std::path::Path;

use rand::Rng;
use rocket::form::{Result, FromFormField, DataField, Form, Error};
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use crate::util::config::CONFIG;
use crate::util::encryption::encrypt_with_random_key;
use crate::util::secret::Secret;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UploadResponse {
    pub file_name: String,
    pub encryption_key: String
}

pub struct UploadRequest {
    extension: String,
    data: Vec<u8>
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for UploadRequest {

    async fn from_data(field: DataField<'r, '_>) -> Result<'r, Self> {
        let name = match field.file_name {
            Some(name) => name.dangerous_unsafe_unsanitized_raw().to_string(),
            None => return Err(Error::validation("Invalid file name"))?
        };

        let extension = match Path::new(&name)
            .extension()
            .and_then(OsStr::to_str) {
                Some(extension) => extension.to_lowercase(),
                None => return Err(Error::validation("Invalid file extension"))?
            };

        if !CONFIG.allowed_extensions.contains(&extension.to_string()) {
            return Err(Error::validation("Not allowed file extension"))?;
        }

        let data = field.data.open(CONFIG.upload_limit).into_bytes().await?;

        if !data.is_complete() {
            return Err(Error::validation(format!("File is too large. Upload limit: {}", CONFIG.upload_limit)))?;
        }

        let data = data.into_inner();

        Ok(UploadRequest {
            extension,
            data
        })
    }
}

#[post("/upload", format = "multipart/form-data", data="<form>")]
pub async fn upload_file(_secret: Secret, form: Form<UploadRequest>) -> std::io::Result<Json<UploadResponse>> {
    let form = form.into_inner();
    let extension = form.extension;
    let data = form.data;

    let (encryption_key, output) = encrypt_with_random_key(data);

    let file_name = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>();
    let file_name = format!("{}.{}", file_name, extension);
    let path = Path::new(&CONFIG.upload_folder).join(&file_name);

    std::fs::write(path, output)?;
    
    return Ok(UploadResponse {
        file_name,
        encryption_key
    }.into());
}

