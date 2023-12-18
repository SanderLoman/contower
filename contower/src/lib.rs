#![deny(unsafe_code)]

use clap::{App, Arg};
use git_version::git_version;
use slog::{o, Drain, Level, Logger, Record};
use target_info::Target;

// A custom Drain filter that allows logging only for specific log levels.
struct RangeLevelFilter<D> {
    levels: Vec<Level>, // List of log levels to allow
    drain: D,           // The underlying Drain
}

// Implement the Drain trait for RangeLevelFilter
impl<D> Drain for RangeLevelFilter<D>
where
    D: Drain<Ok = ()>,
{
    type Ok = ();
    type Err = D::Err;

    // Log the record if its level is in the allowed list; otherwise, do nothing.
    fn log(&self, record: &Record, values: &slog::OwnedKVList) -> Result<Self::Ok, Self::Err> {
        if self.levels.contains(&record.level()) {
            self.drain.log(record, values)?;
            Ok(())
        } else {
            Ok(())
        }
    }
}

// Parse the verbosity level from command line arguments and return a vector of allowed log levels.
pub fn parse_verbosity() -> Vec<Level> {
    let matches = App::new("Wagmi")
        .version("1.0")
        .author("Sander Feitsma")
        .about("Wagmi, brah")
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let verbosity = matches.occurrences_of("verbosity");

    // Map the verbosity level to a vector of slog::Level
    match verbosity {
        1 => vec![Level::Info],
        2 => vec![Level::Info, Level::Warning],
        3 => vec![Level::Info, Level::Warning, Level::Error],
        4 => vec![Level::Info, Level::Warning, Level::Error, Level::Critical],
        5 => vec![
            Level::Info,
            Level::Warning,
            Level::Error,
            Level::Critical,
            Level::Debug,
        ],
        6 => vec![
            Level::Info,
            Level::Warning,
            Level::Error,
            Level::Critical,
            Level::Debug,
            Level::Trace,
        ],
        _ => vec![Level::Info, Level::Warning, Level::Error, Level::Critical],
    }
}

// Create and return a Logger configured with the given log levels.
pub fn create_logger(levels: Vec<Level>) -> Logger {
    // Create a Drain for stdout
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    // Wrap the Drain in a RangeLevelFilter
    let drain = RangeLevelFilter { levels, drain };

    // Create and return the root Logger
    Logger::root(drain.fuse(), o!())
}

/// Returns the current version of this build of ConTower.
///
/// A plus-sign (`+`) is appended to the git commit if the tree is dirty (not commited).
/// Commit hash is omitted if the sources don't include git information.
///
/// ## Example
///
/// `ConTower/v0.1.0-67da032*`
pub const VERSION: &str = git_version!(
    args = [
        "--always",
        "--dirty=*",
        "--abbrev=7",
        // NOTE: using --match instead of --exclude for compatibility with old Git
        "--match=thiswillnevermatchlol"
    ],
    prefix = "ConTower/v0.1.0-",
    fallback = "ConTower/v0.1.0-"
);

/// Returns `VERSION`, but with platform information appended to the end.
///
/// ## Example
///
/// `ConTower/v0.1.0-67da032*/x86_64-linux`
pub fn version_with_platform() -> String {
    format!("{}/{}-{}", VERSION, Target::arch(), Target::os())
}