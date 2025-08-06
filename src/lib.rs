// src/lib.rs
/*
 * Core library for BackupCopy
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
 
use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write};

 

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct BackupCopyProcessor {
    verbose: bool,
    processed_count: usize,
}

impl BackupCopyProcessor {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    pub fn process(&mut self, data: &str) -> Result<ProcessResult> {
        if self.verbose {
            debug!("Processing data of length: {}", data.len());
        }

        // Simulate processing
        self.processed_count += 1;
        
        let result = ProcessResult {
            success: true,
            message: format!("Successfully processed item #{}", self.processed_count),
            data: Some(serde_json::json!({
                "length": data.len(),
                "processed_at": chrono::Utc::now().to_rfc3339(),
                "item_number": self.processed_count
            })),
        };

        Ok(result)
    }

    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}

/// Main processing function
pub fn run(verbose: bool, input: Option<String>, output: Option<String>) -> Result<()> {
    // Read from input file or stdin
    let mut input_data = String::new();

    match input {
        Some(path) => {
            if verbose {
                println!("Reading from input file: {}", path);
            }
            let mut file = File::open(path)?;
            file.read_to_string(&mut input_data)?;
        }
        None => {
            if verbose {
                println!("Reading from standard input...");
            }
            io::stdin().read_to_string(&mut input_data)?;
        }
    }

    // Write to output file or stdout
    match output {
        Some(path) => {
            if verbose {
                println!("Writing to output file: {}", path);
            }
            let mut file = File::create(path)?;
            file.write_all(input_data.as_bytes())?;
        }
        None => {
            if verbose {
                println!("Writing to standard output...");
            }
            println!("{}", input_data);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = BackupCopyProcessor::new(true);
        assert_eq!(processor.verbose, true);
        assert_eq!(processor.processed_count, 0);
    }

    #[test]
    fn test_data_processing() {
        let mut processor = BackupCopyProcessor::new(false);
        let result = processor.process("test data").unwrap();
        
        assert!(result.success);
        assert_eq!(processor.processed_count, 1);
    }

    #[test]
    fn test_run_function() {
        // Test the main run function
        let result = run(false, None, None);
        assert!(result.is_ok());
    }
}
