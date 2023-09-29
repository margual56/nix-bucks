mod fixed_expense;
mod recurrence;
mod subscription;

pub use fixed_expense::{FixedExpense, TmpExpense};
pub use recurrence::{times_until, Day, Recurrence, SimpleRecurrence};
pub use subscription::{Subscription, TmpSubscription};

mod tests {
    use chrono::NaiveDate;
    use once_cell::sync::Lazy;

    #[allow(unused)]
    use crate::utils::{
        recurrence::{Day, Recurrence},
        times_until,
    };

    #[allow(unused)]
    static START: Lazy<NaiveDate> = Lazy::new(|| NaiveDate::from_ymd_opt(2023, 5, 3).unwrap());

    #[allow(unused)]
    static TARGET: Lazy<NaiveDate> = Lazy::new(|| NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());

    #[test]
    fn test_times_until_days() {
        assert_eq!(times_until(Recurrence::Day(3), *START, *TARGET), 80);
        assert_eq!(times_until(Recurrence::Day(1), *START, *TARGET), 242);
    }

    #[test]
    fn test_times_until_months() {
        assert_eq!(
            times_until(Recurrence::Month(Day::Normal(1), 1), *START, *TARGET),
            7
        );
        assert_eq!(
            times_until(Recurrence::Month(Day::Normal(1), 2), *START, *TARGET),
            3
        );
        assert_eq!(
            times_until(Recurrence::Month(Day::Normal(1), 3), *START, *TARGET),
            2
        );
    }

    #[test]
    fn test_times_until_years() {
        let target = NaiveDate::from_ymd_opt(2033, 12, 31).unwrap();

        assert_eq!(
            times_until(Recurrence::Year(Day::Normal(1), 1, 1), *START, target),
            10
        );
        assert_eq!(
            times_until(Recurrence::Year(Day::Normal(1), 1, 2), *START, target),
            5
        );
        assert_eq!(
            times_until(Recurrence::Year(Day::Normal(1), 1, 3), *START, target),
            3
        );
    }
}
