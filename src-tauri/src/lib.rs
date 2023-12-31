pub mod getters;
pub mod tables;
mod utils;

pub use utils::{FixedExpense, Recurrence, Subscription, TmpExpense, TmpSubscription};

pub struct Wrapper(pub Mutex<App>);

use std::{collections::HashMap, fs::File, io::Read, sync::Mutex};

use cached::proc_macro::cached;
use chrono::{Datelike, NaiveDate, Utc};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "margual56";
const APPLICATION: &str = "NixBucks";

pub fn format_money(amount: f32) -> String {
    if amount == 0.0 {
        // This is to avoid the whole "- 0.0" debacle, because of floating
        // point precision.
        "0.00 €".to_string()
    } else {
        format!("{:+.2} €", amount)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    pub initial_savings: f32,
    pub subscriptions: HashMap<Uuid, Subscription>,
    pub incomes: HashMap<Uuid, Subscription>,
    pub fixed_expenses: HashMap<Uuid, FixedExpense>,
    pub p_incomes: HashMap<Uuid, FixedExpense>,
    pub dismissed_ad: bool,
    pub lang: String,
}

impl Default for App {
    fn default() -> Self {
        if let Some(dir) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let mut path = match std::fs::File::open(dir.config_dir().join("config.json")) {
                Ok(p) => p,
                Err(e) => {
                    println!("Error while opening file: {}", e);
                    return Self {
                        initial_savings: 0.0,
                        subscriptions: HashMap::new(),
                        fixed_expenses: HashMap::new(),
                        incomes: HashMap::new(),
                        p_incomes: HashMap::new(),
                        dismissed_ad: false,
                        lang: String::from("en"),
                    };
                }
            };

            let mut buffer = String::new();

            path.read_to_string(&mut buffer).unwrap();

            serde_json::from_str::<Self>(&buffer)
                .unwrap_or(Self {
                    initial_savings: 0.0,
                    subscriptions: HashMap::new(),
                    fixed_expenses: HashMap::new(),
                    incomes: HashMap::new(),
                    p_incomes: HashMap::new(),
                    dismissed_ad: false,
                    lang: String::from("en"),
                })
                .update()
        } else {
            println!("Directory not found, returning default value");
            Self {
                initial_savings: 0.0,
                subscriptions: HashMap::new(),
                fixed_expenses: HashMap::new(),
                incomes: HashMap::new(),
                p_incomes: HashMap::new(),
                dismissed_ad: false,
                lang: String::from("en"),
            }
        }
    }
}

#[cached]
fn cost_to_year_end(subscriptions: Vec<Subscription>, expenses: Vec<FixedExpense>) -> f32 {
    let mut amount = 0.0;
    let year_end = NaiveDate::from_ymd_opt(Utc::now().year(), 12, 31).unwrap();

    for subscription in subscriptions {
        amount += subscription.cost_until(year_end);
    }

    for expense in expenses {
        if Utc::now().naive_utc().date() <= expense.date() && expense.date() <= year_end {
            amount += expense.cost();
        }
    }

    amount
}

impl App {
    /// Saves the data to the config file. It uses the [`directories::ProjectDirs`](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html) struct to find the config folder with:
    /// - QUALIFIER: "com"
    /// - ORGANIZATION: "margual56"
    /// - APPLICATION: "NixBucks"
    ///
    /// And appends "config.json" to the path. Then, it overwrites the file with the serialized data.
    pub fn save_data(&self) {
        if let Some(dir) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            if !dir.config_dir().exists() {
                std::fs::create_dir_all(dir.config_dir()).unwrap();
            }

            let path = File::create(dir.config_dir().join("config.json")).unwrap();

            serde_json::to_writer_pretty(path, self).unwrap();
        }
    }

    /// Updates the app by removing the expired subscriptions and incomes and adding the amounts to the "initial amount".
    pub fn update(&self) -> Self {
        let mut app = self.clone();

        let today = Utc::now().date_naive();

        for (uuid, expense) in self.fixed_expenses.clone() {
            if today > expense.date {
                app.initial_savings -= expense.cost();
                app.remove_expense(uuid);
            }
        }

        for (uuid, income) in self.p_incomes.clone() {
            if today > income.date {
                app.initial_savings += income.cost();
                app.remove_punctual_income(&uuid);
            }
        }

        app.save_data();

        app.clone()
    }

    pub fn remove_from_uuid(self, uuid: Uuid) -> Self {
        if self.subscriptions.contains_key(&uuid) {
            let mut subs = self.subscriptions.clone();
            subs.remove_entry(&uuid);

            let app = App {
                subscriptions: subs,
                ..self
            };

            return app;
        } else if self.fixed_expenses.contains_key(&uuid) {
            let mut fexp = self.fixed_expenses.clone();
            fexp.remove_entry(&uuid);

            let app = App {
                fixed_expenses: fexp,
                ..self
            };

            return app;
        } else if self.incomes.contains_key(&uuid) {
            let mut incomes = self.incomes.clone();
            incomes.remove_entry(&uuid);

            let app = App { incomes, ..self };

            return app;
        } else if self.p_incomes.contains_key(&uuid) {
            let mut p_incomes = self.p_incomes.clone();
            p_incomes.remove_entry(&uuid);

            let app = App { p_incomes, ..self };

            return app;
        } else {
            return self;
        }
    }

    pub fn add_subscription(&mut self, tmp: TmpSubscription) -> Subscription {
        let uuid = Uuid::new_v4();
        let subscription: Subscription = tmp.to_subscription(uuid);

        self.subscriptions.insert(uuid, subscription.clone());

        subscription
    }

    pub fn add_income(&mut self, tmp: TmpSubscription) -> Subscription {
        let uuid = Uuid::new_v4();

        let subscription: Subscription = tmp.to_subscription(uuid);

        self.incomes.insert(uuid, subscription.clone());

        subscription
    }

    pub fn add_p_expense(&mut self, tmp: TmpExpense) -> FixedExpense {
        let p_expense: FixedExpense = FixedExpense::new(
            tmp.name,
            tmp.cost,
            NaiveDate::parse_from_str(&tmp.date, "%d/%m/%Y").unwrap(),
        );

        self.fixed_expenses
            .insert(p_expense.uuid(), p_expense.clone());

        p_expense
    }

    pub fn add_p_income(&mut self, tmp: TmpExpense) -> FixedExpense {
        let p_income: FixedExpense = FixedExpense::new(
            tmp.name,
            tmp.cost,
            NaiveDate::parse_from_str(&tmp.date, "%d/%m/%Y").unwrap(),
        );

        self.p_incomes.insert(p_income.uuid(), p_income.clone());

        p_income
    }

    /// Removes an expense.
    /// # Arguments
    /// - `uuid`: The UUID of the expense to remove.
    pub fn remove_expense(&mut self, uuid: Uuid) {
        self.fixed_expenses.remove(&uuid);
    }

    /// Remove a punctual income.
    /// # Arguments
    /// - `uuid`: The UUID of the income to remove.
    pub fn remove_punctual_income(&mut self, uuid: &Uuid) {
        self.p_incomes.remove(uuid);
    }

    /// Returns the total cost of all subscriptions in a whole year.
    pub fn yearly_costs(&self) -> f32 {
        let mut cost = 0.0;

        for subscription in self.subscriptions.clone().into_values() {
            cost += subscription.cost()
                * subscription
                    .recurrence()
                    .times_in_a_year()
                    .unwrap_or(1.0 / 12.0);
        }

        for expense in self.fixed_expenses.clone().into_values() {
            if expense.date().year_ce().1 == Utc::now().year_ce().1 {
                cost += expense.cost();
            }
        }

        cost
    }

    pub fn eoy_costs(&self) -> f32 {
        cost_to_year_end(
            self.subscriptions.clone().into_values().collect(),
            self.fixed_expenses.clone().into_values().collect(),
        )
    }

    /// Returns the total cost of all subscriptions in a month.
    pub fn monthly_costs(&self) -> f32 {
        let mut amount = 0.0;

        for subscription in self.subscriptions.values() {
            amount += subscription.cost_per_month();
        }

        amount
    }

    /// Returns the balance at the end of each month (all income streams - all subscriptions).
    pub fn monthly_balance(&self) -> f32 {
        let mut amount = 0.0;

        for income in self.incomes.values() {
            amount += income.cost_per_month();
        }

        for subscription in self.subscriptions.values() {
            amount -= subscription.cost_per_month();
        }

        amount
    }

    pub fn yearly_income(&self) -> f32 {
        cost_to_year_end(
            self.incomes.clone().into_values().collect(),
            self.p_incomes.clone().into_values().collect(),
        )
    }

    pub fn yearly_balance(&self) -> f32 {
        self.yearly_income() - self.eoy_costs()
    }
}
