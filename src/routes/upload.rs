use std::ffi::OsStr;
use std::path::Path;

use rocket::form::{self, FromFormField, DataField, Form};
use rocket::Either;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use crate::util::config::CONFIG;
use crate::util::error::ErrorResponse;
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

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let name = field.file_name;

        if name.is_none() {
            return Err(form::Error::validation("Invalid file name"))?;
        }

        let name = name.unwrap().dangerous_unsafe_unsanitized_raw().to_string();

        let extension = Path::new(&name)
            .extension()
            .and_then(OsStr::to_str)
            .map(|s| s.to_lowercase());
        let data = field.data;

        println!("name: {:?}", name);
        println!("extension: {:?}", extension);

        if extension.is_none() {
            return Err(form::Error::validation("Invalid file extension"))?;
        }

        let extension = extension.unwrap();

        if !CONFIG.allowed_extensions.contains(&extension.to_string()) {
            return Err(form::Error::validation("Not allowed file extension"))?;
        }

        let data = data.open(CONFIG.upload_limit).into_bytes().await?;

        if !data.is_complete() {
            return Err(form::Error::validation(format!("File is too large. Upload limit: {}", CONFIG.upload_limit)))?;
        }

        let data = data.into_inner();

        Ok(UploadRequest {
            extension,
            data
        })
    }
}

#[post("/upload", format = "multipart/form-data", data="<form>")]
pub async fn upload_file(_secret: Secret, form: Form<UploadRequest>) -> std::io::Result<(Status, Either<Json<UploadResponse>, Json<ErrorResponse>>)> {
    let form = form.into_inner();
    let extension = form.extension;
    let data = form.data;
    
    return Ok((Status::Ok, Either::Left(UploadResponse {
        file_name: "test".to_string(),
        encryption_key: "test".to_string()
    }.into())));
}

