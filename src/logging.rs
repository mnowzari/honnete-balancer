use std::error::Error;
use std::collections::HashMap;
use crate::env::EnvLogLevel;

// Custom logging library to handle logs
// 
// We grab log level from our config YAML file's env level
// In the YAML file, we can also configure the details of an Elasticsearch instance.


// LogHandler should establish several things:
// a. The log level
// Both of these should be found in the config YAML
pub struct LogHandler {
    log_level: EnvLogLevel,
}

impl LogHandler {
    pub fn new(env_level: EnvLogLevel) -> Result<LogHandler, Box<dyn Error>> {
        Ok(LogHandler {
            log_level: env_level,
        })
    }

    pub fn log() -> Result<(), Box<dyn Error>> {

        Ok(())
    }
}

