pub use log::{error, info, warn};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use owo_colors::OwoColorize;

struct TrainingModpackLogger;

impl log::Log for TrainingModpackLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => {
                    println!(
                        "[TrainingModpack] [{}] {}",
                        record.level().red(),
                        record.args()
                    );
                }
                Level::Warn => {
                    println!(
                        "[TrainingModpack] [{}] {}",
                        record.level().yellow(),
                        record.args()
                    );
                }
                Level::Info => {
                    println!(
                        "[TrainingModpack] [{}] {}",
                        record.level().cyan(),
                        record.args()
                    );
                }
                _ => {
                    println!("[TrainingModpack] [{}] {}", record.level(), record.args());
                }
            };
        }
    }

    fn flush(&self) {}
}

static LOGGER: TrainingModpackLogger = TrainingModpackLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
