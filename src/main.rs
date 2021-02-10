use rusoto_core::Region;
use rusoto_core::credential::ProfileProvider;
use rusoto_s3::{S3, S3Client, GetObjectRequest};
use tokio::io;

#[tokio::main]
async fn main() {
    let region = Region::Custom{
        name: Region::UsEast1.name().to_owned(),
        endpoint: String::from("https://s3.isk01.sakurastorage.jp"),
    };
    let s3 = S3Client::new_with(
        rusoto_core::request::HttpClient::new().expect("Failed to creat HTTP client"),
        ProfileProvider::new().unwrap(),
        region,
    );
    let bucket_name = String::from("gh-action-test");
    let key = String::from("index.html");
    let get_obj_req = GetObjectRequest {
        bucket: bucket_name,
        key: key,
        ..Default::default()
    };
    let mut output = s3.get_object(get_obj_req).await.unwrap();
    let body = output.body.take().expect("The object has no body");
    let mut body = body.into_async_read();
    io::copy(&mut body, &mut tokio::io::stdout()).await.unwrap(); 
}
