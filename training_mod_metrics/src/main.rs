use datafusion::prelude::*;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::datasource::json::NdJsonFile;
use datafusion::physical_plan::json::NdJsonReadOptions;
use datafusion::arrow::datatypes::{Schema, Field, DataType};

use std::sync::Arc;

// export.json is relative to /event/
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

use chrono::{DateTime, NaiveDateTime, Utc};
fn timestamp_secs_to_datetime(ts: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts, 0), Utc)
}

use plotters::prelude::*;
const OUT_FILE_NAME: &'static str = "boxplot.svg";
fn draw_chart(results: Vec<RecordBatch>) -> Result<(), Box<dyn std::error::Error>> {
    let num_devices_idx = results[0].schema().column_with_name("num_devices").unwrap().0;
    let num_sessions_idx = results[0].schema().column_with_name("num_sessions").unwrap().0;
    let timestamps_idx = results[0].schema().column_with_name("date").unwrap().0;

    let num_devices = results[0].column(num_devices_idx).as_any()
        .downcast_ref::<datafusion::arrow::array::UInt64Array>()
        .expect("Failed to downcast").values();
    let num_sessions = results[0].column(num_sessions_idx).as_any()
        .downcast_ref::<datafusion::arrow::array::UInt64Array>()
        .expect("Failed to downcast").values();
    let timestamp_millis = results[0].column(timestamps_idx).as_any()
        .downcast_ref::<datafusion::arrow::array::TimestampMillisecondArray>()
        .expect("Failed to downcast").values();

    let device_data_points = num_devices.iter()
        .enumerate().map(|(i, x)| (timestamp_secs_to_datetime(timestamp_millis[i] / 1000), *x));
    let session_data_points = num_sessions.iter()
        .enumerate().map(|(i, x)| (timestamp_secs_to_datetime(timestamp_millis[i] / 1000), *x));
    
    let root = SVGBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Users and Sessions by Date", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (timestamp_secs_to_datetime(timestamp_millis[0] / 1000))..(timestamp_secs_to_datetime(*timestamp_millis.last().unwrap() / 1000)), 
            0..*num_sessions.iter().max().unwrap())?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            device_data_points,
            &RED,
        ))?
        .label("Unique Devices")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .draw_series(LineSeries::new(
            session_data_points,
            &BLUE,
        ))?
        .label("Unique Sessions")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
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
                Field::new("mod_version", DataType::Utf8, false),
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
        "SELECT 
            COUNT(DISTINCT device_id) num_devices, 
            -- COUNT(DISTINCT session_id) num_sessions, 
            COUNT(DISTINCT smash_version) num_sessions,  
            COUNT(*) num_events, 
            TO_TIMESTAMP_MILLIS(DATE_TRUNC('day', CAST(event_time * 1000000 AS timestamp))) AS date FROM menu_open
        WHERE
            -- after 09/01/2021
            event_time > 1630454400000
            -- before today
            AND CAST(event_time * 1000000 AS timestamp) < NOW()
        GROUP BY date ORDER BY date"
    )?;

    let results: Vec<RecordBatch> = df.collect().await?;
    // use datafusion::arrow::util::pretty::pretty_format_batches;
    // println!("{}", pretty_format_batches(&results)?);

    draw_chart(results).unwrap();

    Ok(())
}