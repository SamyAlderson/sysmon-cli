//! Module for collecting and processing system metrics.
//!
//! This module provides functions for collecting CPU and memory usage metrics from the system.
//! It also includes utilities for processing and formatting these metrics.

use std::time::Duration;
use std::thread;
use crossbeam_channel::{unbounded, Receiver, Sender};

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

use clap::{Parser, Args};
use serde_json::json;

lazy_static! {
    /// Channel to send metrics to the notification module.
    static ref METRICS_CHANNEL: (Sender<Metrics>, Receiver<Metrics>) = unbounded();
}

/// Represents a system metric.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    /// Current CPU usage percentage.
    pub cpu_usage: f64,
    /// Current memory usage percentage.
    pub memory_usage: f64,
    /// Timestamp of when the metric was collected.
    pub timestamp: u64,
}

/// Collects CPU usage metrics from the system.
///
/// This function uses the `std::time::Duration` type to measure the time elapsed between two points.
/// It then calculates the CPU usage based on the time elapsed and the number of iterations.
///
/// Returns a `Metrics` struct containing the collected metric.
pub fn collect_cpu_usage() -> Metrics {
    let start = std::time::Instant::now();
    let iterations = 10_000_000;
    let mut cpu_usage = 0.0;

    for _ in 0..iterations {
        // Intentionally do some work to simulate CPU usage.
        let _ = thread::sleep(Duration::from_micros(10));
        cpu_usage += std::mem::size_of::<f64>() as f64 / 1024.0 / 1024.0 / 1024.0;
    }

    let elapsed = start.elapsed();
    let cpu_usage = (cpu_usage / iterations as f64) / (elapsed.as_nanos() as f64 / 1_000_000_000.0);

    Metrics {
        cpu_usage,
        memory_usage: 0.0,
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
    }
}

/// Collects memory usage metrics from the system.
///
/// This function uses the `std::process::Process` type to get the current memory usage of the process.
///
/// Returns a `Metrics` struct containing the collected metric.
pub fn collect_memory_usage() -> Metrics {
    let mem_usage = std::process::Process::current().memory_usage();
    let mem_percent = (mem_usage as f64 / 1024.0 / 1024.0 / 1024.0) / (8.0 * 1024.0 * 1024.0 * 1024.0) * 100.0;

    Metrics {
        cpu_usage: 0.0,
        memory_usage: mem_percent,
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
    }
}

/// Sends a set of metrics to the notification module.
///
/// This function takes a `Metrics` struct as input and sends it to the notification module via a channel.
pub fn send_metrics(metrics: Metrics) {
    METRICS_CHANNEL.0.send(metrics).unwrap();
}