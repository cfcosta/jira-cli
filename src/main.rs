extern crate clap;
extern crate toml;
extern crate reqwest;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, SubCommand};
use log::{LogRecord, LogMetadata, LogLevelFilter};

mod issue;
mod config;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }
}

fn main() {
    let matches = App::new("jira")
        .version("0.1")
        .about("CLI client for JIRA issues")
        .author("Cainã Costa <me@cfcosta.com>")
        .arg(
            Arg::with_name("verbose")
            .help("Sets the level of verbosity")
            .long("verbose")
            .short("v")
            .multiple(true)
        )
        .subcommand(
            SubCommand::with_name("issue")
            .about("View an issue")
            .arg(
                Arg::with_name("issue_id")
                .help("The id of the issue to view")
                .index(1)
                .required(true)
            )
        )
        .subcommand(
            SubCommand::with_name("copy-cfg")
            .about("Copy the default config to config path")
        )
        .get_matches();

    let log_level = match matches.occurrences_of("verbose") {
        0 => LogLevelFilter::Off,
        1 => LogLevelFilter::Error,
        2 => LogLevelFilter::Warn,
        3 => LogLevelFilter::Info,
        4 | _ => LogLevelFilter::Debug
    };

    log::set_logger(|max_log_level| {
        max_log_level.set(log_level);
        Box::new(SimpleLogger)
    }).expect("Logger could not be initialized!");

    debug!("[Config] Log level: {:?}", log_level);

    let config = config::read();
    debug!("[Config] Loaded config: {:?}", config);

    match matches.subcommand() {
        ("issue", Some(matches)) => {
            let issue_id = matches.value_of("issue_id").unwrap();

            let path = format!("{}/rest/api/latest/issue/{}"
                               , config.host.hostname
                               , issue_id);
            let issue: issue::Issue = reqwest::get(&*path)
                .expect("Failed to fetch issue!")
                .json()
                .expect("Failed to deserialize issue!");

            debug!("[HTTP] Requested issue, got response: {:?}", issue);

            issue::print(issue);
        },
        ("copy-cfg", Some(_matches)) => {
            config::write_defaults();
        },
        _ => { unreachable!("We should not get here...") }
    }
}
