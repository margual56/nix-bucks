use chrono::{NaiveDate, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TmpExpense {
    pub name: String,
    pub cost: f32,

    pub date: String,
}

/// A fixed expense is an expense that is not recurrent.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct FixedExpense {
    uuid: Uuid,
    pub name: String,
    pub cost: OrderedFloat<f32>,

    pub date: NaiveDate,
}

impl Default for FixedExpense {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: String::new(),
            cost: OrderedFloat(0.0),
            date: Utc::now().naive_utc().date(),
        }
    }
}

impl FixedExpense {
    /// Create a new fixed expense.
    /// # Arguments
    /// - `name`: The name of the fixed expense.
    /// - `cost`: The cost of the fixed expense.
    /// - `date`: The date of the fixed expense.
    /// # Returns
    /// - A new fixed expense.
    /// # Examples
    /// ```
    /// use chrono::{Utc, NaiveDate};
    /// use nix_bucks::FixedExpense;
    ///
    /// pub fn main() {
    ///    let fixed_expense = FixedExpense::new(
    ///        String::from("My new fixed expense"),
    ///        123.0,
    ///        Utc::now().naive_utc().date()
    ///     );
    ///
    ///     println!("{:?}", fixed_expense);
    /// }
    /// ```
    pub fn new(name: String, cost: f32, date: NaiveDate) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            cost: OrderedFloat(cost),
            date,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the cost.
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

    /// Returns the date
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// Returns the uuid.
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}
