extern crate reqwest;

mod synonym;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

use log::{debug, error, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    filter::threshold::ThresholdFilter,
};
use crate::synonym::thesaurus::Thesaurus;
use crate::synonym::Finder;

fn init_log() -> Result<(), Box<dyn std::error::Error>>  {
    let level = log::LevelFilter::Debug;
    let file_path = "app.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        .build(file_path)
        .unwrap();

    // Log [level] level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    debug!("Configure log");
    init_log()?;

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Path arg not found");
        error!("Path arg not found");
        return Ok(())
    }

    let path= args[1].as_str();

    debug!("Opening file");
    let input = File::open(path)?;
    let buffered = BufReader::new(input);


    debug!("Reading file");
    for line in buffered.lines() {
        let word = line.unwrap();
        debug!("Searching synonyms for {}", word);

        let q = Thesaurus::new_query(&word);
        let synonyms = q.find_synonyms()?;
        debug!("Sinonimos: {:?}", synonyms);

    }

    debug!("Finish");

    Ok(())

}
