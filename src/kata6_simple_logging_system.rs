// ============================================
// Kata 6: Simple Logging System
// Focus: Dependency Injection, Interface Segregation, Strategy Pattern
// ============================================

use std::fmt;
use chrono::{DateTime, Local};

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Log message
pub struct LogMessage {
    level: LogLevel,
    message: String,
    timestamp: DateTime<Local>,
}

impl LogMessage {
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        LogMessage {
            level,
            message: message.into(),
            timestamp: Local::now(),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "[{}] {} - {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.level,
            self.message
        )
    }
}

/// Logger trait
pub trait Logger: Send + Sync {
    fn log(&self, message: &LogMessage);
    fn minimum_level(&self) -> LogLevel;
}

/// Console logger
pub struct ConsoleLogger {
    min_level: LogLevel,
}

impl ConsoleLogger {
    pub fn new(min_level: LogLevel) -> Self {
        ConsoleLogger { min_level }
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, message: &LogMessage) {
        if message.level >= self.min_level {
            println!("{}", message.format());
        }
    }

    fn minimum_level(&self) -> LogLevel {
        self.min_level
    }
}

/// In-memory logger (for testing)
pub struct InMemoryLogger {
    min_level: LogLevel,
    messages: std::sync::Mutex<Vec<String>>,
}

impl InMemoryLogger {
    pub fn new(min_level: LogLevel) -> Self {
        InMemoryLogger {
            min_level,
            messages: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.lock().unwrap().clone()
    }
}

impl Logger for InMemoryLogger {
    fn log(&self, message: &LogMessage) {
        if message.level >= self.min_level {
            let mut messages = self.messages.lock().unwrap();
            messages.push(message.format());
        }
    }

    fn minimum_level(&self) -> LogLevel {
        self.min_level
    }
}

/// Multi-logger (Composite Pattern)
pub struct MultiLogger {
    loggers: Vec<Box<dyn Logger>>,
}

impl MultiLogger {
    pub fn new() -> Self {
        MultiLogger { loggers: Vec::new() }
    }

    pub fn add_logger(&mut self, logger: Box<dyn Logger>) {
        self.loggers.push(logger);
    }
}

impl Logger for MultiLogger {
    fn log(&self, message: &LogMessage) {
        for logger in &self.loggers {
            logger.log(message);
        }
    }

    fn minimum_level(&self) -> LogLevel {
        self.loggers
            .iter()
            .map(|l| l.minimum_level())
            .min()
            .unwrap_or(LogLevel::Error)
    }
}

/// Application (using dependency injection)
pub struct Application {
    logger: Box<dyn Logger>,
}

impl Application {
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Application { logger }
    }

    pub fn run(&self) {
        self.logger.log(&LogMessage::new(LogLevel::Info, "Application started"));
        // Application logic
        self.logger.log(&LogMessage::new(LogLevel::Info, "Application finished"));
    }

    pub fn process_data(&self, data: &str) -> Result<(), String> {
        self.logger.log(&LogMessage::new(LogLevel::Debug, format!("Processing: {}", data)));
        
        if data.is_empty() {
            self.logger.log(&LogMessage::new(LogLevel::Error, "Empty data received"));
            return Err("Empty data".to_string());
        }
        
        self.logger.log(&LogMessage::new(LogLevel::Info, "Data processed successfully"));
        Ok(())
    }
}

#[cfg(test)]
mod logger_tests {
    use super::*;

    #[test]
    fn test_in_memory_logger() {
        let logger = InMemoryLogger::new(LogLevel::Info);
        logger.log(&LogMessage::new(LogLevel::Debug, "Debug message")); // Should not be logged
        logger.log(&LogMessage::new(LogLevel::Info, "Info message"));
        logger.log(&LogMessage::new(LogLevel::Error, "Error message"));
        
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_application_with_logger() {
        let logger = Box::new(InMemoryLogger::new(LogLevel::Info));
        let app = Application::new(logger);
        
        app.run();
        // In actual tests, the log content can be verified
    }
}
