use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

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
pub fn upload_file(data: Form<UploadRequest>) -> Json<UploadResponse> {
    println!("Secret: {}", data.secret);
    println!("image = {:?}", data.image);

    UploadResponse {
        file_name: "test".to_string(),
        encryption_key: "test".to_string()
    }.into()
}