use notify_rust::{Notification, Timeout};
use tokio::time;

use passport_scrape::get_urgent;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut last_time = time::Instant::now();

    loop {
        let next_time = last_time + time::Duration::from_secs(10);
        let mut interval = time::interval(next_time - last_time);

        last_time = time::Instant::now();
        interval.tick().await;

        match get_urgent().await {
            Ok(_) => {
                let message_body = {
                    let now = chrono::Local::now();
                    format!(
                        "[{}] Passport appointments available!",
                        now.format("%y-%m-%d %h:%m:%s")
                    )
                };
                println!("{message_body}");
                Notification::new()
                    .summary("Passport appointments")
                    .body(&message_body)
                    .timeout(Timeout::Never)
                    .show()
                    .map_err(|err| {
                        eprintln!("Error sending notification: {err:?}");
                        ()
                    })?;
            }
            Err(e) => {
                let now = chrono::Local::now();
                eprintln!(
                    "[{now}] Failure: {e:?}",
                    now = now.format("%Y-%m-%d %H:%M:%S")
                );
            }
        };

        interval.tick().await;
    }
}
