use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use datafusion::arrow::array::{Int64Array, TimestampMillisecondArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::datasource::file_format::{
    file_compression_type::FileCompressionType, options::NdJsonReadOptions,
};
use datafusion::prelude::*;
use plotters::prelude::*;
use std::sync::Arc;
use structopt::StructOpt;

const OUT_FILE_NAME: &str = "usage_metrics.svg";

#[derive(Debug, StructOpt)]
#[structopt(
    name = "analyze_metrics",
    about = "Analyze SSBU Training Modpack usage metrics"
)]
struct Opt {
    /// Input JSON file path (output from parse_firebase)
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,

    /// Output SVG file path
    #[structopt(parse(from_os_str), default_value = OUT_FILE_NAME)]
    output: std::path::PathBuf,

    /// Start date (YYYY-MM-DD format)
    #[structopt(long)]
    start_date: Option<String>,

    /// End date (YYYY-MM-DD format)
    #[structopt(long)]
    end_date: Option<String>,
}

fn parse_date_arg(date_str: &str) -> Result<i64> {
    let naive =
        NaiveDateTime::parse_from_str(&format!("{} 00:00:00", date_str), "%Y-%m-%d %H:%M:%S")
            .map_err(|e| anyhow::anyhow!("Failed to parse date {}: {}", date_str, e))?;
    Ok(Utc.from_utc_datetime(&naive).timestamp_millis())
}

fn timestamp_millis_to_datetime(ts: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(ts / 1000, 0).unwrap()
}

async fn analyze_data(
    input_path: &str,
    output_path: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<()> {
    // Parse date arguments
    let date_filter = match (start_date, end_date) {
        (Some(start), Some(end)) => {
            let start_ts = parse_date_arg(start)?;
            let end_ts = parse_date_arg(end)? + 86400000; // Add one day to include the end date
            format!(
                "WHERE event_time >= {} AND event_time < {}",
                start_ts, end_ts
            )
        }
        (Some(start), None) => {
            let start_ts = parse_date_arg(start)?;
            format!("WHERE event_time >= {}", start_ts)
        }
        (None, Some(end)) => {
            let end_ts = parse_date_arg(end)? + 86400000;
            format!("WHERE event_time < {}", end_ts)
        }
        (None, None) => String::from(""),
    };

    // Create a session context
    let ctx = SessionContext::new();

    // Register the JSON file as a table
    ctx.register_json(
        "smash_open",
        input_path,
        NdJsonReadOptions {
            schema: Some(&Arc::new(Schema::new(vec![
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
            file_compression_type: FileCompressionType::UNCOMPRESSED,
            file_sort_order: vec![],
            infinite: false,
            table_partition_cols: vec![],
        },
    )
    .await?;

    // Query to get daily metrics
    let query = format!(
        "WITH base_stats AS (
            SELECT 
                TO_TIMESTAMP_MILLIS(CAST(event_time / 86400000 * 86400000 AS bigint)) AS date,
                device_id,
                session_id,
                event_time
            FROM smash_open
            {}
        ),
        daily_stats AS (
            SELECT 
                date,
                COUNT(DISTINCT device_id) AS num_devices,
                COUNT(DISTINCT session_id) AS num_sessions,
                COUNT(*) AS num_events
            FROM base_stats
            GROUP BY date
        )
        SELECT 
            num_devices,
            num_sessions,
            num_events,
            date
        FROM daily_stats
        ORDER BY date",
        date_filter
    );

    let df = ctx.sql(&query).await?;

    let results = df.collect().await?;

    // Create the visualization
    let root = SVGBackend::new(output_path, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Calculate y-axis max value with some padding
    let (y_max, min_date, max_date) = {
        let mut max_value = 0f64;
        let mut min_ts = i64::MAX;
        let mut max_ts = i64::MIN;

        for batch in &results {
            for row in 0..batch.num_rows() {
                let devices = batch
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .value(row) as f64;
                let sessions = batch
                    .column(1)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .value(row);
                let ts = batch
                    .column(3)
                    .as_any()
                    .downcast_ref::<TimestampMillisecondArray>()
                    .unwrap()
                    .value(row);

                max_value = max_value.max(devices).max(sessions as f64);
                min_ts = min_ts.min(ts);
                max_ts = max_ts.max(ts);
            }
        }
        // Add 10% padding to the max value
        (
            (max_value * 1.1).ceil(),
            timestamp_millis_to_datetime(min_ts),
            timestamp_millis_to_datetime(max_ts),
        )
    };

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Training Modpack Usage Metrics",
            ("sans-serif", 50).into_font(),
        )
        .margin(15)
        .margin_top(30)
        .x_label_area_size(80)
        .y_label_area_size(60)
        .build_cartesian_2d(min_date..max_date, 0f64..y_max)?;

    // Configure the mesh
    chart
        .configure_mesh()
        .disable_mesh() // Remove default grid
        .bold_line_style(WHITE.mix(0.3)) // Subtle grid lines
        .light_line_style(WHITE.mix(0.1))
        .axis_style(BLACK.mix(0.7)) // Softer axis lines
        .x_labels(12)
        .y_labels(10)
        .x_label_formatter(&|x| x.format("%Y-%m-%d").to_string())
        .y_label_formatter(&|y| {
            if *y >= 1000.0 {
                format!("{:.1}k", y / 1000.0)
            } else {
                format!("{:.0}", y)
            }
        })
        .x_label_style(
            ("sans-serif", 15)
                .into_font()
                .transform(FontTransform::Rotate90),
        )
        .y_label_style(("sans-serif", 15).into_font())
        .draw()?;

    // Draw horizontal grid lines
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_axis()
        .disable_x_axis()
        .y_desc("Number of Users/Sessions")
        .draw()?;

    let mut device_data = Vec::new();
    let mut session_data = Vec::new();

    for batch in &results {
        for row in 0..batch.num_rows() {
            let ts = batch
                .column(3)
                .as_any()
                .downcast_ref::<TimestampMillisecondArray>()
                .unwrap()
                .value(row);
            let devices = batch
                .column(0)
                .as_any()
                .downcast_ref::<Int64Array>()
                .unwrap()
                .value(row);
            let sessions = batch
                .column(1)
                .as_any()
                .downcast_ref::<Int64Array>()
                .unwrap()
                .value(row);

            device_data.push((timestamp_millis_to_datetime(ts), devices as f64));
            session_data.push((timestamp_millis_to_datetime(ts), sessions as f64));
        }
    }

    // Define custom colors
    const DEVICE_COLOR: RGBColor = RGBColor(46, 125, 190); // Blue
    const SESSION_COLOR: RGBColor = RGBColor(55, 166, 155); // Teal

    // Draw the data series with enhanced styling
    chart
        .draw_series(LineSeries::new(device_data.clone(), &DEVICE_COLOR))?
        .label("Unique Devices")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &DEVICE_COLOR));

    // Add points over the device line
    chart.draw_series(
        device_data
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 3, DEVICE_COLOR.filled())),
    )?;

    chart
        .draw_series(LineSeries::new(session_data.clone(), &SESSION_COLOR))?
        .label("Unique Sessions")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &SESSION_COLOR));

    // Add points over the session line
    chart.draw_series(
        session_data
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 3, SESSION_COLOR.filled())),
    )?;

    // Configure and draw the legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK.mix(0.5))
        .position(SeriesLabelPosition::UpperRight)
        .margin(15)
        .legend_area_size(35)
        .draw()?;

    // Ensure the drawing is saved
    root.present()?;

    println!("Chart saved to {}", output_path);
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    analyze_data(
        opt.input.to_str().unwrap(),
        opt.output.to_str().unwrap(),
        opt.start_date.as_deref(),
        opt.end_date.as_deref(),
    )
    .await?;

    println!(
        "Analysis complete! Output saved to {}",
        opt.output.display()
    );
    Ok(())
}
