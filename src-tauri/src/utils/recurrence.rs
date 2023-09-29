use std::fmt::Display;

use cached::proc_macro::cached;
use chrono::{Datelike, Days, Months, NaiveDate};

use serde::{Deserialize, Serialize};

/// Returns the amount of days in a month.
/// This function is cached: It will only run once for each value you give it. Then, it caches the
/// result and returns it when you call it again with the same value.
/// # Arguments
/// - `m`: The month.
/// # Returns
/// - The amount of days in the month.
#[cached]
pub fn days_in_month(m: u8) -> u8 {
    match m {
        1 => 31,
        2 => 28, // TODO: Leap years
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}

/// A simplified version of the recurrence enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimpleRecurrence {
    Day,
    Month,
    Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Day {
    Normal(u8),
    Last,
}

impl Day {
    pub fn from_day(day: u8, month: u8) -> Self {
        let max_days = days_in_month(month);

        if day >= max_days {
            Self::Last
        } else {
            Self::Normal(day)
        }
    }

    pub fn get_day(&self, month: u8) -> u8 {
        let max_days = days_in_month(month);

        match self {
            Self::Normal(d) => {
                if *d > max_days {
                    max_days
                } else {
                    *d
                }
            }
            Self::Last => max_days,
        }
    }
}

/// A more complex recurrence enum. It stores the recurrence in a more complex way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Recurrence {
    /// Amount of days
    Day(u8),
    /// Day of the month, amount of months
    Month(Day, u8),
    /// Day of the month, month of the year, amount of years
    Year(Day, u8, u8),
}

impl Recurrence {
    /// Creates a recurrence from a simple recurrence.
    /// # Arguments
    /// - `value`: The simple recurrence.
    /// - `days`: The amount of days if it's a `Day` recurrence OR the day of the month otherwise.
    /// - `months`: The amount of months if it's a `Month` recurrence OR the month of the year otherwise.
    /// - `years`: The amount of years if it's a `Year` recurrence.
    ///
    pub fn from_simple_recurrence(
        value: SimpleRecurrence,
        days: u8,
        months: u8,
        years: u8,
    ) -> Self {
        match value {
            SimpleRecurrence::Day => Self::Day(days),
            SimpleRecurrence::Month => Self::Month(Day::from_day(days, months), months),
            SimpleRecurrence::Year => Self::Year(Day::from_day(days, months), months, years),
        }
    }

    pub fn times_in_a_year(self) -> Option<f32> {
        match self {
            Self::Day(days) => Some(365.0 / days as f32),
            Self::Month(_, months) => Some(12.0 / months as f32),
            Self::Year(_, _, years) => Some(1.0 / years as f32),
        }
    }
}

impl Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Day(days) => write!(f, "Every {} day(s)", days),
            Self::Month(day_r, months) => match day_r {
                Day::Normal(day) => write!(f, "Every {} month(s) on day {}", months, day),
                Day::Last => write!(f, "The last day of the month every {} month(s)", months),
            },
            Self::Year(day_r, month, years) => match day_r {
                Day::Normal(day) => write!(
                    f,
                    "Every {} year(s) on day {} of month {}",
                    years, day, month
                ),
                Day::Last => write!(f, "The last day of month {} every {} year(s)", month, years),
            },
        }
    }
}

/// Returns the amount of times the recurrence occurs between the two given dates.
/// This function is cached: It will only run once for each value you give it. Then, it caches the
/// result and returns it when you call it again with the same value.
/// # Arguments
/// - `recurrence`: The recurrence.
/// - `from`: The starting date.
/// - `to`: The target date.
/// # Returns
/// - The amount of times the recurrence occurs between the two given dates.
/// # Examples
/// ```
/// use chrono::NaiveDate;
/// use nix_bucks::{Recurrence, Day, times_until};
///
/// fn main() {
///    let start = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
///    let end = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
///
///    let recurrence = Recurrence::Month(Day::Normal(1), 1);
///    let times = times_until(recurrence, start, end);
///    assert_eq!(times, 14);
///
///    println!("{}", times);
/// }
/// ```
#[cached]
pub fn times_until(recurrence: Recurrence, from: NaiveDate, to: NaiveDate) -> u32 {
    match recurrence {
        Recurrence::Day(each_days) => {
            (to.signed_duration_since(from).num_days() as f32 / each_days as f32).trunc() as u32
        }
        Recurrence::Month(day, each_months) => {
            // Count the amount of times the day "day" has passed since today to the target date
            let mut start = from
                .clone()
                .with_day(day.get_day(from.month0() as u8 + 1) as u32)
                .unwrap();

            let mut times: u32 = 0;

            if start < from {
                start = start
                    .checked_add_months(Months::new(each_months as u32))
                    .unwrap();
            } else {
                times += 1;
            }

            let target = to
                .clone()
                .with_day(day.get_day(to.month0() as u8 + 1) as u32)
                .unwrap()
                .checked_add_days(Days::new(1))
                .unwrap();

            while target > start {
                times += 1;

                start = start
                    .checked_add_months(Months::new(each_months as u32))
                    .unwrap();
            }

            times
        }
        Recurrence::Year(day, month, each_years) => {
            // Count the amount of times the day "day" has passed since today to the target date
            let mut start = from
                .clone()
                .with_day(day.get_day(from.month0() as u8 + 1) as u32)
                .unwrap()
                .with_month(month as u32)
                .unwrap();

            let mut times: u32 = 0;

            if start < from {
                start = start
                    .checked_add_months(Months::new(each_years as u32 * 12))
                    .unwrap();
            } else {
                times += 1;
            }

            let target = to
                .clone()
                .with_day(day.get_day(to.month0() as u8 + 1) as u32)
                .unwrap()
                .checked_add_days(Days::new(1))
                .unwrap()
                .with_month(month as u32)
                .unwrap();

            while target > start {
                times += 1;

                start = start
                    .checked_add_months(Months::new(each_years as u32 * 12))
                    .unwrap();
            }

            times
        }
    }
}

impl Recurrence {
    /// Returns the string representation according to the language given.
    /// # Arguments
    /// - `lang`: The language.
    /// # Returns
    /// - The string representation according to the language given.
    pub fn to_simple_str(&self) -> &str {
        match self {
            Self::Day(_) => "Day",
            Self::Month(_, _) => "Month",
            Self::Year(_, _, _) => "Year",
        }
    }

    // Returns the string representation according to the language given.
    // # Arguments
    // - `lang`: The language.
    // # Returns
    // - The string representation according to the language given.
    // pub fn to_lang_str(&self, lang: &str) -> String {
    //     match self {
    //         Self::Day(days) => t!("recurrence.days", days: &format!("{}", days), lang),
    //         Self::Month(day, months) => {
    //             t!("recurrence.months", day: &format!("{}", day), months: &format!("{}", months), lang)
    //         }
    //         Self::Year(day, month, years) => {
    //             t!("recurrence.years", day: &format!("{}", day), month: &format!("{}", month), years: &format!("{}", years), lang)
    //         }
    //     }
    // }
}
