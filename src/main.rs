use battery::{Manager, State};
use notify_rust::Notification;
use std::{thread, time::Duration};

fn main() {
    // Create a battery manager
    let manager = Manager::new().expect("Failed to initialize battery manager");

    loop {
        // Get all batteries
        let batteries = manager.batteries().expect("Failed to get batteries");

        for maybe_battery in batteries {
            if let Ok(battery) = maybe_battery {
                // Check the battery's state and percentage
                let percentage = battery.state_of_charge().value * 100.0;
                let state = battery.state();

                if percentage < 15.0 && state != State::Charging {
                    // Send a notification if below 10% and not charging
                    Notification::new()
                        .summary("Low Battery")
                        .body("Battery level is below 10%. Please plug in your charger.")
                        .timeout(5000) // Notification timeout in milliseconds
                        .show()
                        .expect("Failed to send notification");
                }
            }
        }

        // Sleep for a minute before checking again
        thread::sleep(Duration::from_secs(60));
    }
}
