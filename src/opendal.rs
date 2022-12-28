use anyhow::Result;
use async_compat::Compat;
use futures_util::stream::stream::StreamExt;
use opendal::services::s3::Builder;
use opendal::Operator;
use parquet2::read::{get_page_stream, read_metadata};
use parquet2::statistics::BinaryStatistics;
use std::io::Cursor;
use std::sync::Arc;

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

    let data = object.range_read(0..length).await?;
    let mut reader = Compat::new(data);
    let metadata = read_metadata(&mut reader)?;

    println!("number of rows: {}", metadata.num_rows);

    let row_group = 0;
    let column = 0;
    let column_metadata = &metadata.row_groups[row_group].columns()[column];
    let mut pages = get_page_stream(
        column_metadata,
        &mut reader,
        vec![],
        Arc::new(|_, _| true),
        usize::MAX,
    )
    .await?;
    while let Some(_page) = pages.next().await {
        println!("got a page");
    }

    Ok(())
}
