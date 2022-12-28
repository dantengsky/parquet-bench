use anyhow::Result;
use opendal::services::s3::Builder;
use opendal::Operator;
use parquet2::read::{get_page_stream, read_metadata_async};
use std::sync::Arc;
use futures::StreamExt;

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

    let mut reader = object.seekable_reader(0..);
    let metadata = read_metadata_async(&mut reader).await?;

    println!("number of rows: {}", metadata.num_rows);

    let row_group = 0;
    let column = 0;
    let column_metadata = &metadata.row_groups[row_group].columns()[column];
    let pages = get_page_stream(
        column_metadata,
        &mut reader,
        vec![],
        Arc::new(|_, _| true),
        usize::MAX,
    )
    .await?;

    //pin_mut!(pages);
    let mut pages = Box::pin(pages);
    while let Some(_page) = pages.next().await {
        println!("got a page");
    }

    Ok(())
}
