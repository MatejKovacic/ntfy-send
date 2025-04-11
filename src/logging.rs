use std::fs::{OpenOptions, File};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;
use chrono::Local;
use log::{LevelFilter, Log, Metadata, Record};

const LOG_FILE: &str = "/var/lib/ntfy-send/ntfy-send.log";
const MAX_LOG_ENTRIES: usize = 500; // Set to 5 for only last 5 entries

struct FileLogger {
    file: File,
}

impl FileLogger {
    fn rotate_log() -> io::Result<()> {
        let path = Path::new(LOG_FILE);
        if !path.exists() {
            return Ok(());
        }

        // Read all lines
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        // Only rotate if over limit
        if lines.len() > MAX_LOG_ENTRIES {
            // Calculate how many lines to keep
            let skip_count = lines.len() - MAX_LOG_ENTRIES;
            
            // Write back trimmed log
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)?;
            
            for line in lines.into_iter().skip(skip_count) {
                writeln!(&mut file, "{}", line)?;
            }
        }

        Ok(())
    }
}

impl Log for FileLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S");
            let mut file = &self.file;
            let _ = writeln!(file, "{} [{}] - {}", now, record.level(), record.args());
        }
    }

    fn flush(&self) {
        let _ = self.file.sync_all();
    }
}

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure directory exists
    let log_path = Path::new(LOG_FILE);
    if let Some(parent) = log_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Rotate log before opening
    FileLogger::rotate_log()?;

    // Open or create log file
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)?;

    let logger = FileLogger { file };

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(LevelFilter::Info);

    Ok(())
}
