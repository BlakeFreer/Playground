use anyhow::Result;
use s3::{Auth, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::builder("http://127.0.0.1:3900")?
        .region("garage")
        .auth(Auth::from_env()?)
        .build()?;

    let bucket = "default-bucket";

    let obj = client.objects().get(bucket, "blake.txt").send().await?;
    println!("{:?}", obj.bytes().await?);

    let obj = client.objects().list_v2(bucket).send().await?;
    for o in obj.contents {
        println!("{o:?}")
    }

    Ok(())
}
