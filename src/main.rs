//! sysmon-cli: A command-line system monitor written in Rust.

use clap::{App, Arg};
use crossbeam_channel::{unbounded, select};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use std::{thread, time};
use metrics::{collect_metrics, Metric};
use notifications::{send_notification, Notification};

const UPDATE_INTERVAL: time::Duration = time::Duration::from_secs(5);
const DEFAULT_METRICS: Vec<Metric> = vec![
    Metric::CpuUsage,
    Metric::MemoryUsage,
];

#[derive(Serialize, Deserialize)]
struct Config {
    metrics: Vec<Metric>,
    email: Option<Notification>,
}

fn load_config() -> Result<Config, String> {
    // Load configuration from file or environment variables
    // For simplicity, we'll assume a file-based config for now
    let config_path = "config.json";
    match std::fs::read_to_string(config_path) {
        Ok(config_str) => serde_json::from_str(&config_str),
        Err(e) => Err(format!("Failed to load config: {}", e)),
    }
}

fn main() {
    let matches = App::new("sysmon-cli")
        .version("1.0")
        .author("Your Name")
        .about("A command-line system monitor")
        .arg(
            Arg::with_name("config")
                .long("config")
                .help("Path to configuration file"),
        )
        .arg(
            Arg::with_name("metrics")
                .long("metrics")
                .help("Comma-separated list of metrics to collect")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("email")
                .long("email")
                .help("Email address to send notifications to")
                .takes_value(true),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config.json");
    let config = load_config().unwrap_or_else(|e| {
        eprintln!("Error loading config: {}", e);
        std::process::exit(1);
    });

    let metrics = matches
        .values_of("metrics")
        .map(|values| values.collect::<Vec<_>>())
        .unwrap_or_else(|| DEFAULT_METRICS.clone());

    let email = matches.value_of("email").map(|email| {
        let notification = Notification::from_str(email).unwrap_or_else(|e| {
            eprintln!("Error parsing email notification: {}", e);
            std::process::exit(1);
        });
        notification
    });

    let (tx, rx) = unbounded();
    let metrics_thread = thread::spawn(move || {
        loop {
            select! {
                _ = tx.send(collect_metrics(&metrics)) => {},
                _ = select! {
                    val = rx.recv() => {
                        match val {
                            Some(metrics) => collect_metrics(&metrics),
                            None => break,
                        }
                    },
                    _ = time::Duration::from_millis(1000) => {}
                }
            }
            thread::sleep(UPDATE_INTERVAL);
        }
    });

    let notification_thread = thread::spawn(move || loop {
        select! {
            val = rx.recv() => {
                match val {
                    Some(metrics) => send_notification(email, &metrics),
                    None => break,
                }
            },
            _ = time::Duration::from_millis(1000) => {}
        }
    });

    notification_thread.join().unwrap();
    metrics_thread.join().unwrap();
}