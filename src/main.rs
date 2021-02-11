use anyhow::{Context, Result};
use rusoto_core::credential::ProfileProvider;
use rusoto_core::Region;
use rusoto_s3::{GetObjectOutput, GetObjectRequest, S3Client, S3};
use tokio::io;

const SAKURA_OBJECT_STORAGE_ENDPOINT: &str = "https://s3.isk01.sakurastorage.jp";

fn sakura_object_storage_region() -> Region {
    Region::Custom {
        name: String::from(Region::UsEast1.name()),
        endpoint: String::from(SAKURA_OBJECT_STORAGE_ENDPOINT),
    }
}

async fn get_object(region: Region, bucket_name: String, key: String) -> Result<GetObjectOutput> {
    let s3 = S3Client::new_with(
        rusoto_core::request::HttpClient::new()?,
        ProfileProvider::new()?,
        region,
    );
    let get_obj_req = GetObjectRequest {
        bucket: bucket_name,
        key: key,
        ..Default::default()
    };
    s3.get_object(get_obj_req)
        .await
        .context("get object from object storage")
}

#[tokio::main]
async fn main() {
    let region = sakura_object_storage_region();
    let bucket_name = String::from("gh-action-test");
    let key = String::from("index.html");
    let mut output = get_object(region, bucket_name, key).await.unwrap();
    let body = output.body.take().expect("The object has no body");
    let mut body = body.into_async_read();
    io::copy(&mut body, &mut tokio::io::stdout()).await.unwrap();
}
