use gargoyle::Schedule;
use gargoyle_web_monitor::WebAvailability;
use gargoyle_email_notifier::{Email, Mailbox, Address};
use gargoyle_feed_monitor::WebFeedUpdate;

use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};

use std::{fs::File, thread::sleep, time::Duration};

fn main() {
    simplelog::CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("gargoyle.log").unwrap()
        ),
    ]).unwrap();

    let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");

    let smtp_from = std::env::var("SMTP_FROM").expect("SMTP_FROM not set");
    let (smtp_from_u, smtp_from_d) = smtp_from.split_once('@').expect("Invalid email address");

    let smtp_to = std::env::var("SMTP_TO").expect("SMTP_TO not set");
    let (smtp_to_u, smtp_to_d) = smtp_to.split_once('@').expect("Invalid email address");

    let smtp_relay = std::env::var("SMTP_RELAY").expect("SMTP_RELAY not set");
    let http_url = std::env::var("HTTP_URL").expect("HTTP_URL not set");

    let feed_url = std::env::var("FEED_URL").expect("FEED_URL not set");

    let schedule_delay = std::env::var("SCHEDULE_DELAY_SECS")
        .expect("SCHEDULE_DELAY_SECS not set")
        .parse::<u64>()
        .expect("Invalid SCHEDULE_DELAY_SECS");

    let mail_notifier = Email {
        from: Mailbox::new(
            Some("The Gargoyle".into()),
            Address::new(smtp_from_u, smtp_from_d).expect("Invalid email address")
        ),
        to: Mailbox::new(
            Some("Admin".into()),
            Address::new(smtp_to_u, smtp_to_d).expect("Invalid email address")
        ),
        relay: smtp_relay,
        smtp_username,
        smtp_password,
    };

    let mut web_monitor = WebAvailability::new(&http_url);

    let mut feed_monitor = WebFeedUpdate::new(&feed_url);

    let mut scheduler = Schedule::default();
    scheduler.add(
        &format!("The Gargoyle has detected that {http_url} has gone down"),
        &format!("The Gargoyle has detected that {http_url} has recovered"),
        Duration::from_secs(schedule_delay),
        &mut web_monitor,
        &mail_notifier,
    );
    scheduler.add(
        &format!("The Gargoyle got an update from {feed_url}"),
        &format!("The Gargoyle got an update from {feed_url}"),
        Duration::from_secs(schedule_delay),
        &mut feed_monitor,
        &mail_notifier,
    );

    loop {
        scheduler.run();
        sleep(Duration::from_millis(100));
    }
}

