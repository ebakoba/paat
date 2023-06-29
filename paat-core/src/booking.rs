use anyhow::{anyhow, Result};
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    Element, Page,
};
use chrono::{Datelike, Month, NaiveDate};
use futures::StreamExt;

use crate::types::{event::Event, Direction};

const CHANGE_BUTTON_SELECTOR: &str = r#"body > app-root > app-ticket-checkout-success > app-ticket-layout > div.bg-science-blue-50 > div.container.relative.max-w-screen-lg.pt-8.pb-24.lg\:py-8 > section:nth-child(1) > div.mt-8.active-tickets.mx-2.lg\:mx-0 > app-ticket-detail > article > div > div.px-3.py-1.lg\:items-center.lg\:py-5.md\:px-8.lg\:flex.lg\:justify-between > div.flex.justify-between.items-center.mt-2.lg\:mt-0 > a.lg\:ml-2.flex.items-center.text-xs.btn.btn--secondary.btn--icon.btn--borderless.lg\:flex-none"#;

const DIRECTION_SELECT_SELECTOR: &str = r#"body > app-root > app-ticket-purchase > app-ticket-layout > div.bg-science-blue-50 > div.container.relative.pt-8.pb-5.xl\:pb-8 > div:nth-child(3) > app-ticket-purchase-searchbar > div > div > div.flex.flex-1.py-4.border-b.lg\:flex-none.border-midnight-blue-200.lg\:border-0.lg\:pr-8 > div > a"#;
const DIRECTION_SELECT_ITEM_START: &str = r#"body > app-root > app-ticket-purchase > app-ticket-layout > div.bg-science-blue-50 > div.container.relative.pt-8.pb-5.xl\:pb-8 > div:nth-child(3) > app-ticket-purchase-searchbar > div > div > div.flex.flex-1.py-4.border-b.lg\:flex-none.border-midnight-blue-200.lg\:border-0.lg\:pr-8 > div > app-ticket-route-picker > div"#;

const DATE_SELECT_SELECTOR: &str = r#"body > app-root > app-ticket-purchase > app-ticket-layout > div.bg-science-blue-50 > div.container.relative.pt-8.pb-5.xl\:pb-8 > div:nth-child(3) > app-ticket-purchase-searchbar > div > div > div.flex.items-center.py-4.border-b.lg\:border-b-0.lg\:flex-1.border-midnight-blue-200.lg\:px-8.lg\:border-l > div > div.flex.items-center > div > app-datepicker > input.lowercase.text-date.flatpickr-input.departure-select-date.ng-untouched.ng-pristine.ng-invalid.form-control.input"#;
const SELECTED_MONTH_SELECTOR: &str = r#"body > div > div.flatpickr-months > div > div > span"#;
const SELECTED_YEAR_SELECTOR: &str =
    r#"body > div > div.flatpickr-months > div > div > div > input"#;
const NEXT_MONTH_SELECTOR: &str =
    r#"body > div > div.flatpickr-months > span.flatpickr-next-month > a"#;
const BOOKING_DATE_SELECTOR: &str = r#"body > div > div.flatpickr-innerContainer > div > div.flatpickr-days > div > span.flatpickr-day:not(.prevMonthDay)"#;

const MAIN_BOOKING_URL: &str = r#"https://www.praamid.ee/portal/ticket/checkout/success;"#;
const LANGUAGE_URL: &str = r#"lang=et"#;

const BOOKING_BLOCK_SELECTOR: &str = r#"body > app-root > app-ticket-purchase > app-ticket-layout > div.bg-science-blue-50 > div.container.relative.pt-8.pb-5.xl\:pb-8 > div.mt-6 > div:nth-child(1) > section > app-event-selector > div"#;
const BOOKING_ROW_SELECTOR: &str = r#"body > app-root > app-ticket-purchase > app-ticket-layout > div.bg-science-blue-50 > div.container.relative.pt-8.pb-5.xl\:pb-8 > div.mt-6 > div:nth-child(1) > section > app-event-selector > div > div"#;
const DEPARTURE_TIME_SELECTOR: &str = r#"article > div.flex.justify-between.lg\:justify-start.lg\:content-center.lg\:self-center.p-2.sm\:px-8.lg\:pl-2.lg\:pr-0 > div.w-14.flex.items-center.pl-2.lg\:pl-0 > div"#;
const BOOKING_BUTTON_SELECTOR: &str = r#"article > div.flex.justify-between.lg\:justify-start.lg\:content-center.lg\:self-center.p-2.sm\:px-8.lg\:pl-2.lg\:pr-0 > button"#;

const CONTINUE_BUTTON_SELECTOR: &str = r#"#modal-ticket-content > footer > app-button"#;

async fn wait_for_element(page: &Page, selector: &str) -> Option<Element> {
    let timeout_in_seconds = 10;
    let current_time = std::time::Instant::now();
    loop {
        let element = page.find_element(selector).await;
        if let Ok(element) = element {
            if let Ok(_) = element.scroll_into_view().await {
                return Some(element);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
        if current_time.elapsed().as_secs() > timeout_in_seconds {
            return None;
        }
    }
}

async fn open_booking_change(page: &Page) -> Result<()> {
    let change_booking_button = wait_for_element(&page, CHANGE_BUTTON_SELECTOR).await;
    if let Some(change_booking_button) = change_booking_button {
        change_booking_button.click().await?;
    }
    Ok(())
}

fn parse_month(month_string_option: Option<String>) -> Option<Month> {
    if let Some(month_string) = month_string_option {
        return match month_string.to_lowercase().as_str() {
            "jaanuar" => Some(Month::January),
            "veebruar" => Some(Month::February),
            "märts" => Some(Month::March),
            "aprill" => Some(Month::April),
            "mai" => Some(Month::May),
            "juuni" => Some(Month::June),
            "juuli" => Some(Month::July),
            "august" => Some(Month::August),
            "september" => Some(Month::September),
            "oktoober" => Some(Month::October),
            "november" => Some(Month::November),
            "detsember" => Some(Month::December),
            _ => None,
        };
    }
    None
}

async fn move_to_year(page: &Page, desired_year: i32) -> Result<()> {
    let mut counter = 0;
    loop {
        if counter > 120 {
            return Err(anyhow!("Failed to move to desired year"));
        }
        let selected_year = wait_for_element(&page, SELECTED_YEAR_SELECTOR).await;
        if let Some(selected_year) = selected_year {
            let year_in_text = selected_year.property("value").await?;
            if let Some(year_in_text) = year_in_text {
                let year_in_text = year_in_text.as_str();
                if let Some(year_in_text) = year_in_text {
                    let year = year_in_text.parse::<i32>().map_err(|_| {
                        anyhow::anyhow!("Failed to parse year from text: {}", year_in_text)
                    })?;
                    if desired_year != year {
                        let next_month_button = wait_for_element(&page, NEXT_MONTH_SELECTOR).await;
                        if let Some(next_month_button) = next_month_button {
                            next_month_button.click().await?;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        counter += 1;
    }
    Ok(())
}

async fn move_to_month(page: &Page, desired_month: u32) -> Result<()> {
    let mut counter = 0;
    loop {
        if counter > 120 {
            return Err(anyhow!("Failed to move to desired month"));
        }
        let selected_month = wait_for_element(&page, SELECTED_MONTH_SELECTOR).await;
        if let Some(selected_month) = selected_month {
            let month_in_text = selected_month.inner_text().await?;
            let month = parse_month(month_in_text.clone()).ok_or_else(|| {
                anyhow::anyhow!(
                    "Failed to parse month from text: {}",
                    month_in_text.unwrap_or_default()
                )
            })?;
            if desired_month != month.number_from_month() {
                let next_month_button = wait_for_element(&page, NEXT_MONTH_SELECTOR).await;
                if let Some(next_month_button) = next_month_button {
                    next_month_button.click().await?;
                }
            } else {
                break;
            }
        }
        counter += 1;
    }
    Ok(())
}

async fn select_booking_date(page: &Page, desired_date: &NaiveDate) -> Result<()> {
    let dates = page.find_elements(BOOKING_DATE_SELECTOR).await?;
    for date in dates {
        let date_text = date.inner_text().await?;
        if let Some(date_text) = date_text {
            let date_text = date_text.trim();
            let date_number = date_text
                .parse::<u32>()
                .map_err(|_| anyhow::anyhow!("Failed to parse date from text: {}", date_text))?;
            if date_number == desired_date.day() {
                date.click().await?;
                break;
            }
        }
    }

    Ok(())
}

async fn select_desired_date(page: &Page, desired_date: &NaiveDate) -> Result<()> {
    let current_date = chrono::Local::today().naive_local();
    if desired_date < &current_date {
        return Err(anyhow::anyhow!("Cannot book into past"));
    }

    let select_date_button = wait_for_element(&page, DATE_SELECT_SELECTOR).await;
    if let Some(select_date_button) = select_date_button {
        select_date_button.click().await?;
    }

    move_to_year(page, desired_date.year()).await?;
    move_to_month(page, desired_date.month()).await?;
    select_booking_date(page, desired_date).await?;

    Ok(())
}

fn desired_direction_to_selector(desired_direction: &Direction) -> String {
    match desired_direction {
        Direction::HR => format!(
            "{} > div:nth-child(2) > div > a",
            DIRECTION_SELECT_ITEM_START
        ),
        Direction::RH => format!(
            "{} > div:nth-child(3) > div > a",
            DIRECTION_SELECT_ITEM_START
        ),
        Direction::KV => format!(
            "{} > div:nth-child(4) > div > a",
            DIRECTION_SELECT_ITEM_START
        ),
        Direction::VK => format!(
            "{} > div:nth-child(5) > div > a",
            DIRECTION_SELECT_ITEM_START
        ),
    }
}

async fn select_desired_direction(page: &Page, desired_direction: &Direction) -> Result<()> {
    let direction_select = wait_for_element(&page, DIRECTION_SELECT_SELECTOR).await;
    if let Some(direction_select) = direction_select {
        direction_select.click().await?;
    }

    let option_selector = desired_direction_to_selector(desired_direction);
    let direction_option = wait_for_element(&page, &option_selector).await;
    if let Some(direction_option) = direction_option {
        direction_option.click().await?;
    }

    Ok(())
}

async fn select_right_booking_type(page: &Page, event: &Event) -> Result<()> {
    wait_for_element(&page, BOOKING_BLOCK_SELECTOR).await;
    let booking_rows = page.find_elements(BOOKING_ROW_SELECTOR).await?;

    for element in booking_rows {
        let departure_time = element.find_element(DEPARTURE_TIME_SELECTOR).await?;
        let departure_time_text = departure_time.inner_text().await?;
        let event_timespan = event.to_string();
        if let Some(departure_time_text) = departure_time_text {
            let departure_time_text = departure_time_text.trim();
            if event_timespan.starts_with(departure_time_text) {
                let booking_button = element.find_element(BOOKING_BUTTON_SELECTOR).await?;
                booking_button.scroll_into_view().await?;
                booking_button.click().await?;
            }
        }
    }

    Ok(())
}

pub async fn change_booking(
    booking_id: &str,
    event: &Event,
    direction: &Direction,
    date: &NaiveDate,
) -> Result<()> {
    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .incognito()
            .with_head()
            .build()
            .map_err(|err| {
                anyhow::anyhow!(
                    "Failed to create browser with following configuration: {:?}",
                    err
                )
            })?,
    )
    .await?;

    let handle = tokio::task::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });

    let booking_url = format!(
        "{}bookingUid={};{}",
        MAIN_BOOKING_URL, booking_id, LANGUAGE_URL
    );

    let page = browser.new_page(&booking_url).await?;
    page.bring_to_front().await?;
    open_booking_change(&page).await?;
    select_desired_date(&page, date).await?;
    select_desired_direction(&page, direction).await?;
    select_right_booking_type(&page, &event).await?;

    let continue_button = wait_for_element(&page, CONTINUE_BUTTON_SELECTOR).await;
    if let Some(continue_button) = continue_button {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        continue_button.scroll_into_view().await?;
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
        continue_button.click().await?;
    }

    handle.await?;
    Ok(())
}
