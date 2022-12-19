use std::ffi::OsStr;
use std::path::Path;

use rocket::Either;
use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use crate::util::config::CONFIG;
use crate::util::error::ErrorResponse;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UploadResponse {
    pub file_name: String,
    pub encryption_key: String
}

#[derive(FromForm)]
pub struct UploadRequest<'f> {
    pub secret: String,
    pub image: TempFile<'f>
}

#[post("/upload", format = "multipart/form-data", data="<data>")]
pub fn upload_file(data: Form<UploadRequest>) -> (Status, Either<Json<UploadResponse>, Json<ErrorResponse>>) {
    let name = data.image.raw_name();

    if name.is_none() {
        return (Status::BadRequest, Either::Right(Json(ErrorResponse {
            error: "Invalid file name".to_string()
        })));
    }

    let name = name.unwrap().dangerous_unsafe_unsanitized_raw().as_str();
    let extension = get_extension_from_filename(name);

    if !CONFIG.secrets.contains(&data.secret) {
        return (Status::Unauthorized, Either::Right(Json(ErrorResponse {
            error: "Invalid secret".to_string()
        })));
    }

    if extension.is_none() {
        return (Status::BadRequest, Either::Right(Json(ErrorResponse {
            error: "Invalid file extension".to_string()
        })));
    }

    let extension = extension.unwrap();

    if !CONFIG.allowed_extensions.contains(&extension.to_string()) {
        return (Status::BadRequest, Either::Right(Json(ErrorResponse {
            error: "Not allowed file extension".to_string()
        })));
    }

    return (Status::Ok, Either::Left(UploadResponse {
        file_name: "test".to_string(),
        encryption_key: "test".to_string()
    }.into()));
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}
