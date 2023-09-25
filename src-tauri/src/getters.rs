use uuid::Uuid;

use crate::format_money;
use crate::FixedExpense;
use crate::Subscription;
use crate::Wrapper;

#[tauri::command]
pub fn get_savings(app: tauri::State<Wrapper>) -> String {
    format_money(app.0.lock().unwrap().initial_savings)
}

#[tauri::command]
pub fn monthly_cost(app: tauri::State<Wrapper>) -> String {
    format_money(-app.0.lock().unwrap().monthly_costs())
}

#[tauri::command]
pub fn yearly_cost(app: tauri::State<Wrapper>) -> String {
    format_money(-app.0.lock().unwrap().yearly_costs())
}

#[tauri::command]
pub fn eoy_cost(app: tauri::State<Wrapper>) -> String {
    format_money(-app.0.lock().unwrap().eoy_costs())
}

#[tauri::command]
pub fn eoy_income(app: tauri::State<Wrapper>) -> String {
    format_money(app.0.lock().unwrap().yearly_income())
}

#[tauri::command]
pub fn eoy_balance(app: tauri::State<Wrapper>) -> String {
    format_money(app.0.lock().unwrap().yearly_balance())
}

#[tauri::command]
pub fn eom_balance(app: tauri::State<Wrapper>) -> String {
    format_money(app.0.lock().unwrap().monthly_balance())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExportedSubscription {
    uuid: Uuid,
    name: String,
    cost: String,
    recurrence: String,
}

impl From<Subscription> for ExportedSubscription {
    fn from(value: Subscription) -> Self {
        ExportedSubscription {
            uuid: value.uuid(),
            name: String::from(value.name()),
            cost: format_money(value.cost()),
            recurrence: value.recurrence().to_string(),
        }
    }
}

#[tauri::command]
pub fn get_subscriptions(app: tauri::State<Wrapper>) -> Vec<ExportedSubscription> {
    let mut arr = app
        .0
        .lock()
        .unwrap()
        .subscriptions
        .clone()
        .into_values()
        .map(|s| ExportedSubscription {
            uuid: s.uuid(),
            name: String::from(s.name()),
            cost: format_money(-s.cost()),
            recurrence: s.recurrence().to_string(),
        })
        .collect::<Vec<ExportedSubscription>>();
    arr.sort_by(|a, b| a.name.cmp(&b.name));

    arr
}

#[tauri::command]
pub fn get_incomes(app: tauri::State<Wrapper>) -> Vec<ExportedSubscription> {
    let mut arr = app
        .0
        .lock()
        .unwrap()
        .incomes
        .clone()
        .into_values()
        .map(|s| ExportedSubscription {
            uuid: s.uuid(),
            name: String::from(s.name()),
            cost: format_money(s.cost()),
            recurrence: s.recurrence().to_string(),
        })
        .collect::<Vec<ExportedSubscription>>();
    arr.sort_by(|a, b| a.name.cmp(&b.name));

    arr
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExportedExpense {
    uuid: Uuid,
    name: String,
    cost: String,
    date: String,
}

impl From<FixedExpense> for ExportedExpense {
    fn from(value: FixedExpense) -> Self {
        ExportedExpense {
            uuid: value.uuid(),
            name: String::from(value.name()),
            cost: format_money(value.cost()),
            date: value.date.to_string(),
        }
    }
}

#[tauri::command]
pub fn get_punctual_incomes(app: tauri::State<Wrapper>) -> Vec<ExportedExpense> {
    let mut arr = app
        .0
        .lock()
        .unwrap()
        .p_incomes
        .clone()
        .into_values()
        .map(|i| ExportedExpense {
            uuid: i.uuid(),
            name: String::from(i.name()),
            cost: format_money(i.cost()),
            date: i.date().format("%d/%m/%Y").to_string(),
        })
        .collect::<Vec<ExportedExpense>>();

    arr.sort_by(|a, b| a.name.cmp(&b.name));

    arr
}

#[tauri::command]
pub fn get_punctual_expenses(app: tauri::State<Wrapper>) -> Vec<ExportedExpense> {
    let mut arr = app
        .0
        .lock()
        .unwrap()
        .fixed_expenses
        .clone()
        .into_values()
        .map(|i| ExportedExpense {
            uuid: i.uuid(),
            name: String::from(i.name()),
            cost: format_money(-i.cost()),
            date: i.date().format("%d/%m/%Y").to_string(),
        })
        .collect::<Vec<ExportedExpense>>();

    arr.sort_by(|a, b| a.name.cmp(&b.name));

    arr
}
