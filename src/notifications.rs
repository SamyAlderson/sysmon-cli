// src/notifications.rs

// Import dependencies
use clap::{App, Arg};
use crossbeam::channel::Sender;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, error::Error, sync::Arc};

// Define a struct to store notification settings
#[derive(Debug, Deserialize, Serialize)]
struct NotificationSettings {
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    slack: Option<String>,
}

// Define a struct to store notification data
#[derive(Debug, Deserialize, Serialize)]
struct NotificationData {
    metric: String,
    threshold: f64,
    value: f64,
}

// Define a struct to manage notifications
pub struct Notifications {
    pub sender: Sender<String>,
    pub notification_settings: Arc<HashMap<String, NotificationSettings>>,
}

impl Notifications {
    // Create a new instance of the Notifications struct
    pub fn new(sender: Sender<String>, notification_settings: Arc<HashMap<String, NotificationSettings>>) -> Self {
        Self {
            sender,
            notification_settings,
        }
    }

    // Send a notification based on system metrics
    pub fn send_notification(&self, notification_data: NotificationData) -> Result<(), Box<dyn Error>> {
        // Get the notification settings for the current metric
        let notification_settings = self.notification_settings.get(&notification_data.metric).ok_or_else(|| {
            format!("No notification settings found for metric {}", notification_data.metric)
        })?;

        // Check if the metric value exceeds the threshold
        if notification_data.value > notification_settings.threshold {
            // Send a notification based on the selected notification method
            match notification_settings.email.as_ref() {
                Some(email) => self.send_email_notification(email, &notification_data),
                None => {
                    // Send a notification to Slack if no email is configured
                    self.send_slack_notification(notification_settings.slack.as_ref().ok_or_else(|| {
                        format!("No Slack channel configured for metric {}", notification_data.metric)
                    })?, &notification_data)
                }
            }
        }

        Ok(())
    }

    // Send an email notification
    fn send_email_notification(&self, email: &str, notification_data: &NotificationData) -> Result<(), Box<dyn Error>> {
        // Send an email notification using the sender channel
        self.sender.send(format!("Metric {} exceeded threshold: {} ({})", notification_data.metric, notification_data.threshold, notification_data.value))?;
        Ok(())
    }

    // Send a Slack notification
    fn send_slack_notification(&self, slack_channel: &str, notification_data: &NotificationData) -> Result<(), Box<dyn Error>> {
        // Send a Slack notification using the sender channel
        self.sender.send(format!("Metric {} exceeded threshold: {} ({})", notification_data.metric, notification_data.threshold, notification_data.value))?;
        Ok(())
    }
}

// Example usage:
fn main() {
    // Create a new instance of the Notifications struct
    let (sender, receiver) = crossbeam::channel::bounded(10);
    let notification_settings = Arc::new(HashMap::from([
        ("cpu_usage".to_string(), NotificationSettings {
            email: Some("example@example.com".to_string()),
            ..Default::default()
        }),
        ("memory_usage".to_string(), NotificationSettings {
            slack: Some("slack_channel".to_string()),
            ..Default::default()
        }),
    ]));

    let notifications = Notifications::new(sender, notification_settings);

    // Send a notification based on system metrics
    let notification_data = NotificationData {
        metric: "cpu_usage".to_string(),
        threshold: 50.0,
        value: 60.0,
    };
    notifications.send_notification(notification_data).unwrap();
}