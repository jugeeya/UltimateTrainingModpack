use datafusion::prelude::*;
use datafusion::arrow::util::pretty::pretty_format_batches;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::datasource::json::NdJsonFile;
use datafusion::physical_plan::json::NdJsonReadOptions;
use datafusion::arrow::datatypes::{Schema, Field, DataType};

use std::sync::Arc;

// cat export.json | jq -c '.SMASH_OPEN.device[][][]' > smash_open.json
#[derive(Debug)]
struct Event {
    device_id: String,
    event_name: String,
    event_time: i64,
    menu_settings: String,
    mod_version: String,
    session_id: String,
    smash_version: String,
    user_id: String
}

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // let smash_open_table = NdJsonFile::try_new(
    //     "smash_open.json",
    //     NdJsonReadOptions{
    //         schema: None,
    //         schema_infer_max_records: 1,
    //         file_extension: ".json",
    //     }
    // ).unwrap();

    let menu_open_table = NdJsonFile::try_new(
        "menu_open.json",
        NdJsonReadOptions{
            schema: Some(Arc::new(Schema::new(vec![
                Field::new("device_id", DataType::Utf8, false),
                Field::new("event_name", DataType::Utf8, false),
                Field::new("event_time", DataType::Int64, false),
                Field::new("menu_settings", DataType::Utf8, false),
                Field::new("session_id", DataType::Utf8, false),
                Field::new("smash_version", DataType::Utf8, false),
                Field::new("user_id", DataType::Utf8, false),
            ]))),
            schema_infer_max_records: 0,
            file_extension: ".json",
        }
    ).unwrap();

    // // declare a new context. In spark API, this corresponds to a new spark SQLsession
    let mut ctx = ExecutionContext::new();

    // ctx.register_table("smash_open", Arc::new(smash_open_table))?;
    ctx.register_table("menu_open", Arc::new(menu_open_table))?;

    // create a plan to run a SQL query
    let df = ctx.sql(
        "SELECT COUNT(DISTINCT device_id), COUNT(DISTINCT session_id), COUNT(*), 
        DATE_TRUNC('day', CAST(event_time * 1000000 AS timestamp)) AS hour FROM menu_open 
        GROUP BY hour ORDER BY hour"
    )?;

    let results: Vec<RecordBatch> = df.collect().await?;
    // format the results
    let pretty_results = pretty_format_batches(&results)?;
    println!("{}", pretty_results);

    Ok(())
}