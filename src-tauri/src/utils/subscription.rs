use std::hash::Hash;

use chrono::{NaiveDate, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{times_until, Recurrence, SimpleRecurrence};

/// A temporary subscription is a subscription that is not yet saved.
/// It's used to create a new subscription.
#[derive(Clone, Serialize, Deserialize)]
pub struct TmpSubscription {
    pub name: String,
    pub cost: f32,
    pub recurrence: SimpleRecurrence,
    pub days: u8,
    pub months: u8,
    pub years: u8,
}

impl Default for TmpSubscription {
    fn default() -> Self {
        Self {
            name: String::new(),
            cost: 10.0,
            recurrence: SimpleRecurrence::Month,
            days: 1,
            months: 1,
            years: 1,
        }
    }
}

impl From<TmpSubscription> for Subscription {
    fn from(val: TmpSubscription) -> Self {
        Subscription::new(
            val.name.to_string(),
            val.cost,
            Recurrence::from_simple_recurrence(val.recurrence, val.days, val.months, val.years),
        )
    }
}

impl TmpSubscription {
    pub fn to_subscription(self, uuid: Uuid) -> Subscription {
        Subscription {
            uuid,
            name: self.name,
            cost: OrderedFloat(self.cost),
            recurrence: Recurrence::from_simple_recurrence(self.recurrence, self.days, self.months, self.years),
        }
    }
}

/// A subscription is a recurrent expense.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Subscription {
    uuid: Uuid,
    name: String,
    cost: OrderedFloat<f32>,
    recurrence: Recurrence,
}

impl Subscription {
    /// Create a new subscription.
    /// # Arguments
    /// - `name`: The name of the subscription.
    /// - `cost`: The cost of the subscription.
    /// - `recurrence`: The recurrence of the subscription.
    /// # Returns
    /// - A new subscription.
    /// # Examples
    /// ```
    /// use nix_bucks::{Subscription, Recurrence};
    /// use chrono::{Utc, NaiveDate};
    ///
    /// pub fn main() {
    ///    let subscription = Subscription::new(
    ///        String::from("My new subscription"),
    ///        123.0,
    ///        Recurrence::Month(1, 1)
    ///    );
    ///
    ///    println!("{:?}", subscription);
    /// }
    /// ```
    pub fn new(name: String, cost: f32, recurrence: Recurrence) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            cost: OrderedFloat(cost),
            recurrence,
        }
    }

    /// Returns the uuid
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the cost
    pub fn cost(&self) -> f32 {
        self.cost.0
    }

    /// Converts the cost to a positive value.
    pub fn positive(self) -> Self {
        let mut other = self.clone();
        
        other.cost.0 = other.cost.0.abs();

        other
    }

    /// Converts the cost to a negative value.
    pub fn negative(self) -> Self {
        let mut other = self.clone();
        
        other.cost.0 = -other.cost.0.abs();

        other
    }

    /// Returns the recurrence
    pub fn recurrence(&self) -> Recurrence {
        self.recurrence
    }

    /// Calculates the cost from today until the given date.
    /// # Arguments
    /// - `to`: The date until the cost should be calculated.
    /// # Returns
    /// - The cost from today until the given date.
    pub fn cost_until(&self, to: NaiveDate) -> f32 {
        let times = times_until(self.recurrence, Utc::now().naive_utc().date(), to);

        self.cost.0 * times as f32
    }

    /// Calculates the cost per year.
    /// # Returns
    /// - The cost per year.
    pub fn cost_per_year(&self) -> f32 {
        let times = match self.recurrence {
            Recurrence::Day(each_days) => 365 / each_days as u32,
            Recurrence::Month(_, each_months) => 12 / each_months as u32,
            Recurrence::Year(_, _, each_years) => 1 / each_years as u32,
        };

        self.cost.0 * times as f32
    }

    /// Calculates the cost per month.
    /// # Returns
    /// - The cost per month
    pub fn cost_per_month(&self) -> f32 {
        let times = match self.recurrence {
            Recurrence::Day(each_days) => 30 / each_days as u32,
            Recurrence::Month(_, each_months) => 1 / each_months as u32,
            Recurrence::Year(_, _, each_years) => 1 / (each_years * 12) as u32,
        };

        self.cost.0 * times as f32
    }
}
