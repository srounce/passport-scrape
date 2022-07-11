use notify_rust::{Notification, Timeout};
use reqwest;
use scraper::{Html, Selector};

const BUSY_MESSAGE: &str = "Sorry, there are no available appointments";

#[derive(Debug)]
pub enum ScrapeError {
    Unknown(String),

    InvalidBody,
    InvalidPageSelector,

    NotificationFailure(String),

    AppointmentsUnavailable,
}

pub async fn get_urgent() -> Result<(), ScrapeError> {
    let response = reqwest::get("https://www.passport.service.gov.uk/urgent")
        .await
        .map_err(|err| ScrapeError::Unknown(err.to_string()))?;

    eprintln!("Response status: {}", response.status());

    let document = async {
        let body = response
            .text()
            .await
            .map_err(|_| ScrapeError::InvalidBody)?;

        Ok(Html::parse_document(&body))
    }
    .await?;

    let h1_selector = Selector::parse("h1").map_err(|_| ScrapeError::InvalidPageSelector)?;

    let heading_message = document
        .select(&h1_selector)
        .map(|heading| heading.text().collect::<String>())
        .collect::<String>();

    if heading_message == BUSY_MESSAGE {
        return Err(ScrapeError::AppointmentsUnavailable);
    }

    {
        let message_body = {
            let now = chrono::Local::now();
            format!(
                "{}: Passport appointments available!",
                now.format("%y-%m-%d %h:%m:%s")
            )
        };
        Notification::new()
            .summary("Passport appointments")
            .body(&message_body)
            .timeout(Timeout::Never)
            .show()
            .map_err(|err| ScrapeError::NotificationFailure(err.to_string()))?;
    }

    return Ok(());
}
