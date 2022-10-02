use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::io::prelude::*;
use datafusion::prelude::*;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::logical_plan::FileType::NdJson;
use datafusion::execution::options::NdJsonReadOptions;
use datafusion::arrow::datatypes::{Schema, Field, DataType};
use serde::{Serialize, Deserialize};

use std::sync::Arc;

// export.json is relative to /event/
// cat export.json | jq -c '.SMASH_OPEN.device[][][]' > smash_open.json
#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
struct EventExport {
    event: HashMap<String, DeviceExport>
}

#[derive(Debug, Deserialize, Serialize)]
struct DeviceExport {
    device: HashMap<String, HashMap<String, HashMap<String, Event>>>
}

use chrono::{DateTime, NaiveDateTime, Utc};
use datafusion::arrow::array::Array;
use flate2::read::GzDecoder;

fn timestamp_secs_to_datetime(ts: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts, 0), Utc)
}

use plotters::prelude::*;
use cloud_storage::Client;
use serde_json::{Deserializer, Value};
use tokio::io::AsyncReadExt;

const OUT_FILE_NAME: &'static str = "boxplot.svg";
fn draw_chart(results: Vec<RecordBatch>) -> Result<(), Box<dyn std::error::Error>> {
    let num_devices_idx = results[0].schema().column_with_name("num_devices").unwrap().0;
    let num_sessions_idx = results[0].schema().column_with_name("num_sessions").unwrap().0;
    let timestamps_idx = results[0].schema().column_with_name("date").unwrap().0;

    let num_devices = results[0].column(num_devices_idx).as_any()
        .downcast_ref::<datafusion::arrow::array::Int64Array>()
        .expect("Failed to downcast").values();
    let num_sessions = results[0].column(num_sessions_idx).as_any()
        .downcast_ref::<datafusion::arrow::array::Int64Array>()
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
    // use firerust::FirebaseClient;
    // use serde_json::Value;
    // use std::error::Error;
    //
    // let client = FirebaseClient::new("https://my-project-1511972643240-default-rtdb.firebaseio.com/").unwrap();
    // let reference = client.reference("/event/SMASH_OPEN/device");
    //
    // use curl::easy::Easy;
    //
    // let smash_open_device_ids = reqwest::get(format!("{FIREBASE_URL}/event/SMASH_OPEN/device.json?shallow=true"))
    //     .await.unwrap()
    //     .json::<HashMap<String, bool>>()
    //     .await.unwrap();
    // let mut smash_open_device_buckets : HashMap<i32, Vec<&String>> = (0..10)
    //     .into_iter()
    //     .map(|bucket| (bucket, vec![]))
    //     .collect();
    //
    // for device_id in smash_open_device_ids.keys() {
    //     let mut hasher = std::collections::hash_map::DefaultHasher::new();
    //     device_id.hash(&mut hasher);
    //     let hash = hasher.finish();
    //     let device_bucket = (hash % 10) as i32;
    //     smash_open_device_buckets.get_mut(&device_bucket).unwrap().push(device_id);
    // }
    //
    // for bucket in smash_open_device_buckets.keys() {
    //     if *bucket != 0 {
    //         continue;
    //     }
    //     let device_ids = smash_open_device_buckets.get(bucket).unwrap();
    //     for device_id in device_ids {
    //         let device_id_url = format!("{FIREBASE_URL}/event/SMASH_OPEN/device/{device_id}.json");
    //         let events = reqwest::get(device_id_url)
    //             .await.unwrap()
    //             .json::<HashMap<String, HashMap<String, Event>>>()
    //             .await.unwrap();
    //         println!("{events:#?}");
    //     }
    // }

    let FIREBASE_URL = "https://my-project-1511972643240-default-rtdb.firebaseio.com";
    println!("About to get url");
    let client = Client::default();
    let bucket = "my-project-1511972643240-default-rtdb-backups";
    let object = "2022-07-22T02:19:06Z_my-project-1511972643240-default-rtdb_data.json.gz";
    let bytes = client.object().download(bucket, object).await.unwrap();
    println!("Received bytes");
    let mut d = GzDecoder::new(&bytes[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    println!("Decoded gzip: {}", s.len());
    let v: EventExport = serde_json::from_str(&s).unwrap();
    let smash_open = &v.event
        .get("SMASH_OPEN").unwrap()
        .device.values().nth(0).unwrap()
        .values().nth(0).unwrap()
        .values().nth(0).unwrap();

    println!("{:#?}", smash_open);

    Ok(())
    // agg_data_and_chart()
}

#[test]
fn test_json_transform() {
    let s = "{ \
        \"event\" : {\
            \"SMASH_OPEN\": {\
                \"device\": {\
                    \"8ffdce0d8b9788160000000000000000\": {\
                        \"1636608807000\": {\
                            \"-MoCU8jJ6sK98wBJff6D\" : {\
                                \"device_id\" : \"8ffdce0d8b9788160000000000000000\",\
                                \"event_name\" : \"MENU_OPEN\",\
                                \"event_time\" : 1636608807000,\
                                \"menu_settings\" : \"http://localhost/?mash_state=1,&follow_up=&attack_angle=&ledge_state=1,2,4,8,16,&ledge_delay=&tech_state=1,2,4,8,&miss_tech_state=1,2,4,8,&defensive_state=1,2,4,8,16,&aerial_delay=&oos_offset=&reaction_time=&fast_fall=&fast_fall_delay=&falling_aerials=&full_hop=&shield_tilt=&di_state=1,&sdi_state=&air_dodge_dir=&sdi_strength=0,&shield_state=0,&save_state_mirroring=0,&input_delay=0,&save_state_enable=1&save_damage=1&hitbox_vis=1&stage_hazards=0&frame_advantage=0&mash_in_neutral=0\",
                                \"mod_version\" : \"3.1.0\",\
                                \"session_id\" : \"c80468a22fec7c282cc2830869adbb01490105b34d6bd54acd734a3c76db54e9\",\
                                \"smash_version\" : \"13.0.0\",\
                                \"user_id\" : \"1000060bfb6ccd6f59437c48a725df9b\"\
                            }\
                        },\
                        \"1636608807001\": {\
                            \"-MoCU8jJ6sK98wBJff6D\" : {\
                                \"device_id\" : \"8ffdce0d8b9788160000000000000000\",\
                                \"event_name\" : \"MENU_OPEN\",\
                                \"event_time\" : 1636608807000,\
                                \"menu_settings\" : \"http://localhost/?mash_state=1,&follow_up=&attack_angle=&ledge_state=1,2,4,8,16,&ledge_delay=&tech_state=1,2,4,8,&miss_tech_state=1,2,4,8,&defensive_state=1,2,4,8,16,&aerial_delay=&oos_offset=&reaction_time=&fast_fall=&fast_fall_delay=&falling_aerials=&full_hop=&shield_tilt=&di_state=1,&sdi_state=&air_dodge_dir=&sdi_strength=0,&shield_state=0,&save_state_mirroring=0,&input_delay=0,&save_state_enable=1&save_damage=1&hitbox_vis=1&stage_hazards=0&frame_advantage=0&mash_in_neutral=0\",
                                \"mod_version\" : \"3.1.0\",\
                                \"session_id\" : \"c80468a22fec7c282cc2830869adbb01490105b34d6bd54acd734a3c76db54e9\",\
                                \"smash_version\" : \"13.0.0\",\
                                \"user_id\" : \"1000060bfb6ccd6f59437c48a725df9b\"\
                            }\
                        }\
                    }\
                }\
            }\
        }\
    }";

    println!("{:#?}", json_transform(&s));
}

fn json_transform(s: &str) -> Vec<Event> {
    let v: EventExport = serde_json::from_str(s).unwrap();

    let smash_open_events = v.event.get("SMASH_OPEN").unwrap().device.values();

    // v.event.get("SMASH_OPEN").unwrap().device.values().flat_map(|smash_open_event| {
    //     smash_open_event.values()
    //         .flat_map(|event| event.values().collect::<Vec<&Event>>())
    //         .collect::<Vec<&Event>>()
    // }).collect::<Vec<&Event>>()

    let mut events = vec![];
    for smash_open_event in smash_open_events {
        for event in smash_open_event.into_values() {
            let mut e = event.into_values().collect::<Vec<Event>>();
            events.append(&mut e);
        }
    }
    events
}

async fn agg_data_and_chart() -> datafusion::error::Result<()> {
    let mut ctx = SessionContext::new();
    let json_options = NdJsonReadOptions{
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
        table_partition_cols: vec![],
    };
    let df = ctx.register_json(
        "smash_open",
        "smash_open.json",
        json_options
    ).await?;

    // let df = ctx.register_json(
    //     "menu_open",
    //     "menu_open.json",
    //     json_options
    // ).await?;

    println!("Running SQL query...");
    let df = ctx.sql(
        "SELECT
            COUNT(DISTINCT device_id) num_devices,
            COUNT(DISTINCT session_id) num_sessions,
            COUNT(*) num_events,
            TO_TIMESTAMP_MILLIS(DATE_TRUNC('day', CAST(event_time * 1000000 AS timestamp))) AS date FROM smash_open
        WHERE
            -- after 09/01/2021
            event_time > 1630454400000
            -- before today
            AND CAST(event_time * 1000000 AS timestamp) < NOW()
        GROUP BY date ORDER BY date"
    ).await?;

    let results: Vec<RecordBatch> = df.collect().await?;
    use datafusion::arrow::util::pretty::pretty_format_batches;
    println!("{}", pretty_format_batches(&results)?);

    println!("Drawing chart...");
    draw_chart(results).unwrap();

    let df = ctx.sql("SELECT MAX(mod_version) FROM smash_open").await?;
    let results: Vec<RecordBatch> = df.collect().await?;
    println!("{}", pretty_format_batches(&results)?);

    Ok(())
}