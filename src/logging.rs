use std::error::Error;
use std::collections::HashMap;
use crate::env::EnvLogLevel;

// Custom logging library to handle logs
// 
// We grab log level from our config YAML file's env level
// In the YAML file, we can also configure the details of an Elasticsearch instance.
// This way, we can make a best-effort to dump logs into ES
// There are probably better ways to do this via established libs, but I want to try my hand at it.


// LogHandler should establish several things:
// a. The log level
// b. Are we dumping to ES?
// Both of these should be found in the config YAML
pub struct LogHandler {
    dump_to_elasticsearch: bool, // indicates if we should use write to an Elasticsearch instance
    log_level: EnvLogLevel,
}

impl LogHandler {
    pub fn new(env_level: EnvLogLevel) -> Result<LogHandler, Box<dyn Error>> {
        Ok(LogHandler {
            dump_to_elasticsearch: false,
            log_level: env_level,
        })
    }

    pub fn log() -> Result<(), Box<dyn Error>> {

        Ok(())
    }
}

