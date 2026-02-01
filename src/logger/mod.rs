use core::*;
use colored::*;

pub fn info(module: &str, message: &str) {
    println!("[INFO] [{module}]: {message}")
}

pub fn warn(module: &str, message: &str, warn_level: i32) {
    println!("[{}] [{module}]: {message}", "WARN".yellow())
}

pub fn err(module: &str, message: &str, err_level: i32) {
    println!("[{}] [{module}]: {message}", "ERR".red())
}