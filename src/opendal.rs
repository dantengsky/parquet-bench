use anyhow::Result;
use opendal::services::s3::Builder;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let bucket = "ursa-labs-taxi-data";
    let path = "2009/01/data.parquet";

    let mut builder: Builder = Builder::default();
    builder.bucket(bucket);

    let op: Operator = Operator::new(builder.build()?);
    let object = op.object(path);
    let meta = object.metadata().await?;
    let length = meta.content_length();
    println!("blob size: {} mb", length / 1024 / 1024);
    Ok(())
}
