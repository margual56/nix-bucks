// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nix_bucks::{getters::*, tables::*, App, TmpExpense, TmpSubscription, Wrapper};
use std::sync::Mutex;
use uuid::Uuid;

#[tauri::command]
fn delete_uuid(app: tauri::State<Wrapper>, uuid: Uuid) {
    let mut my_app = app.0.lock().unwrap();

    *my_app = my_app.clone().remove_from_uuid(uuid).update();

    my_app.save_data();
}

#[tauri::command]
fn add_subscription(app: tauri::State<Wrapper>, tmp: TmpSubscription) -> ExportedSubscription {
    let mut mut_app = app.0.lock().unwrap();

    *mut_app = mut_app.clone();

    let subscription = mut_app.add_subscription(tmp);

    mut_app.update().save_data();

    subscription.negative().into()
}

#[tauri::command]
fn add_income(app: tauri::State<Wrapper>, tmp: TmpSubscription) -> ExportedSubscription {
    let mut mut_app = app.0.lock().unwrap();

    *mut_app = mut_app.clone();

    let subscription = mut_app.add_income(tmp);

    mut_app.update().save_data();

    subscription.positive().into()
}

#[tauri::command]
fn add_punctual_expense(app: tauri::State<Wrapper>, tmp: TmpExpense) -> ExportedExpense {
    let mut mut_app = app.0.lock().unwrap();

    *mut_app = mut_app.clone();

    let p_expense = mut_app.add_p_expense(tmp);

    mut_app.update().save_data();

    p_expense.negative().into()
}

#[tauri::command]
fn add_punctual_income(app: tauri::State<Wrapper>, tmp: TmpExpense) -> ExportedExpense {
    let mut mut_app = app.0.lock().unwrap();

    *mut_app = mut_app.clone();

    let p_income = mut_app.add_p_income(tmp);

    mut_app.update().save_data();

    p_income.positive().into()
}

fn main() {
    tauri::Builder::default()
        .manage(Wrapper(Mutex::new(App::default())))
        .invoke_handler(tauri::generate_handler![
            get_savings,
            get_subscriptions,
            get_incomes,
            get_punctual_incomes,
            get_punctual_expenses,
            monthly_cost,
            eoy_cost,
            eoy_income,
            eoy_balance,
            eom_balance,
            table_subscriptions,
            table_fixed_expenses,
            table_income,
            table_punctual_income,
            delete_uuid,
            add_subscription,
            add_income,
            add_punctual_expense,
            add_punctual_income
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
