//! Utility functions to get current week and season.
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc, Weekday};
use chrono_tz::Tz;
use chrono_tz::US::Eastern;

/// Trait used to abstract time for testing purposes.
trait Clock {
    /// Return the current time of the clock.
    fn now(&self) -> DateTime<Tz>;
}

/// EST time clock.
struct RealClock;

impl Clock for RealClock {
    /// Current time EST.
    fn now(&self) -> DateTime<Tz> {
        Utc::now().with_timezone(&Eastern)
    }
}

/// Private function to calculate current season.
fn get_current_season_internal(roster: Option<bool>, clock: &impl Clock) -> i32 {
    let roster = match roster {
        Some(r) => r,
        None => false,
    };

    let now = clock.now();
    let year = now.year();

    if roster {
        // Roster logic: current year after March 15, otherwise previous year.
        let march_15 = Eastern.with_ymd_and_hms(year, 3, 15, 0, 0, 0).unwrap();
        return if now >= march_15 { year } else { year - 1 };
    } else {
        // Season logic: current year after Thursday following Labor Day.
        // Labor day is the first Monday in september.
        let labor_day = NaiveDate::from_weekday_of_month_opt(year, 9, Weekday::Mon, 1).unwrap();

        let n_thursday = if labor_day.day() >= 5 { 2 } else { 1 };

        let season_start =
            NaiveDate::from_weekday_of_month_opt(year, 9, Weekday::Thu, n_thursday).unwrap();
        let season_start = Eastern
            .from_local_datetime(&season_start.and_hms_opt(0, 0, 0).unwrap())
            .unwrap();

        return if now >= season_start { year } else { year - 1 };
    }
}

/// Gets the current season applying the approriate EST clock.
///
/// This is a wrapper for the corresponding private function.
/// It is simple enough to not need testing whereas the underlying
/// function can be tested with custom clocks which impl Clock.
///
/// # Arguments
///
/// * `roster`  -   If true apply roster logic (see below). If false or none apply season logic.
///
/// Roster logic: current year after March 15, otherwise previous year.
/// Season logic: current year after Thursday following Labor Day, otherwise previous year.
///
/// # Examples
///
/// ```
/// use nflreadrs::utils::get_current_season;
///
/// let roster = Some(true);
/// let current_season = get_current_season(roster);
/// # assert!(current_season >= 2025);
/// ```

pub fn get_current_season(roster: Option<bool>) -> i32 {
    get_current_season_internal(roster, &RealClock)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_current_season_tests {
        use super::*;

        struct FakeClock {
            year: i32,
            month: u32,
            day: u32,
        }

        impl Clock for FakeClock {
            fn now(&self) -> DateTime<Tz> {
                let day = NaiveDate::from_ymd_opt(self.year, self.month, self.day).unwrap();
                Eastern
                    .from_local_datetime(&day.and_hms_opt(23, 35, 45).unwrap())
                    .unwrap()
            }
        }

        #[test]
        fn test_roster_true_various_dates() {
            let cases = [
                // (year, month, day, expected season)
                (2025, 3, 14, 2024),
                (2025, 3, 15, 2025),
                (2027, 3, 1, 2026),
                (2027, 5, 4, 2027),
            ];

            let roster: Option<bool> = Some(true);

            for (year, month, day, exp) in cases {
                let fake_clock = FakeClock { year, month, day };
                assert_eq!(get_current_season_internal(roster, &fake_clock), exp);
            }
        }

        #[test]
        fn test_roster_false_various_dates() {
            let cases = [
                // (year, month, day, expected season)
                (2025, 9, 3, 2024),
                (2025, 9, 4, 2025),
                (2027, 9, 8, 2026),
                (2027, 9, 9, 2027),
            ];

            let roster: Option<bool> = Some(false);

            for (year, month, day, exp) in cases {
                let fake_clock = FakeClock { year, month, day };
                assert_eq!(get_current_season_internal(roster, &fake_clock), exp);
            }
        }

        #[test]
        fn test_roster_none_various_dates() {
            let cases = [
                // (year, month, day, expected season)
                (2025, 9, 3, 2024),
                (2025, 9, 4, 2025),
                (2027, 9, 8, 2026),
                (2027, 9, 9, 2027),
            ];

            let roster: Option<bool> = None;

            for (year, month, day, exp) in cases {
                let fake_clock = FakeClock { year, month, day };
                assert_eq!(get_current_season_internal(roster, &fake_clock), exp);
            }
        }
    }
}
