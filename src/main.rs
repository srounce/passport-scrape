use std::process;

use tokio;
use reqwest;
use scraper::{Html, Selector};
use notify_rust::Notification;

const BUSY_MESSAGE: &str = "Sorry, there are no available appointments";

enum ScrapeError {
    Unknown(String),

    InvalidBody,
    InvalidPageSelector,

    NotificationFailure(String),

    AppointmentsUnavailable,
}

#[tokio::main]
async fn main() {
    match get_urgent().await {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    };
}

async fn get_urgent() -> Result<(), ScrapeError> {
    let response = reqwest::get("https://www.passport.service.gov.uk/urgent")
        .await
        .map_err(|err| ScrapeError::Unknown(err.to_string()))?;

    println!("Response status: {}", response.status());

    let document = async {
        let body = response.text()
            .await
            .map_err(|_| ScrapeError::InvalidBody)?;

        Ok(Html::parse_document(&body))
    }.await?;

    let h1_selector = Selector::parse("h1")
        .map_err(|_| ScrapeError::InvalidPageSelector)?;
    
    let heading_message = document
        .select(&h1_selector)
        .map(|heading| heading.text().collect::<String>())
        .collect::<String>();

    if heading_message == BUSY_MESSAGE {
        return Err(ScrapeError::AppointmentsUnavailable)
    }

    Notification::new()
        .summary("Passport appointments")
        .body("Passport appointments available!")
        .timeout(0)
        .show()
        .map_err(|err| ScrapeError::NotificationFailure(err.to_string()))?;

    return Ok(())
}
