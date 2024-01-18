use chrono_tz::Tz;
use clap::Parser;
use colored::Colorize;
use cron::Schedule;
use similar::{ChangeTag, TextDiff};
use std::str::FromStr;

#[derive(Debug, Parser)]
#[clap(
  name = env!("CARGO_PKG_NAME"),
  version = env!("CARGO_PKG_VERSION"),
  author = env!("CARGO_PKG_AUTHORS"),
  about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct Args {
    #[clap(help = "cron expressions to test")]
    pub expression: String,
    #[clap(
        short,
        long,
        default_value = "UTC",
        help = "The time zone of the time of execution"
    )]
    pub timezone: String,
    #[clap(
        short,
        long,
        default_value_t = 5,
        help = "Number of execution times displayed"
    )]
    pub lines: usize,
    #[clap(short, long, help = "Colors the difference from the previous run time")]
    pub diff: bool,
}

pub fn print_error<E>(reason: E)
where
    E: std::fmt::Display,
{
    println!("{} {}", "error:".bold().red(), reason);
}

fn main() {
    let args = Args::parse();
    let timezone: Tz = match args.timezone.parse::<Tz>() {
        Ok(timezone) => timezone,
        Err(reason) => {
            print_error(reason);

            return;
        }
    };
    let schedule = match Schedule::from_str(&args.expression) {
        Ok(schedule) => schedule,
        Err(error) => {
            print_error(error);

            return;
        }
    };

    let mut prev: Option<String> = None;

    for datetime in schedule
        .upcoming(timezone)
        .take(args.lines)
        .map(|datetime| datetime.to_string())
    {
        if !args.diff || prev.is_none() {
            println!("{}", datetime);

            prev = Some(datetime);

            continue;
        }

        if let Some(ref prev) = prev {
            let diff = TextDiff::from_chars(prev, &datetime);
            let mut line = String::new();

            for change in diff.iter_all_changes() {
                let value = change.value();

                match change.tag() {
                    ChangeTag::Delete => {}
                    ChangeTag::Equal => line.push_str(value),
                    _ => line.push_str(&value.yellow().underline().to_string()),
                };
            }

            println!("{}", line);
        }
    }
}
