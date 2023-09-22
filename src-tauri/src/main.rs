// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nix_bucks::{App, Wrapper, getters::*, tables::*, TmpSubscription, Subscription};
use std::sync::Mutex;
use uuid::Uuid;

#[tauri::command]
fn delete_uuid(app: tauri::State<Wrapper>, uuid: Uuid) {
    let mut my_app = app.0.lock().unwrap();

    *my_app = my_app.clone().remove_from_uuid(uuid).update();

    my_app.save_data();
}

#[tauri::command]
fn add_subscription(app: tauri::State<Wrapper>, tmp: TmpSubscription) -> Subscription {
    let mut mut_app = app.0.lock().unwrap();

    *mut_app = mut_app.clone();
    
    let subscription = mut_app.add_subscription(tmp);
    
    mut_app.update().save_data();

    subscription
}

#[tauri::command]
fn add_income(app: tauri::State<Wrapper>, tmp: TmpSubscription) -> Subscription {
    let mut mut_app = app.0.lock().unwrap();

    *mut_app = mut_app.clone();

    let subscription = mut_app.add_income(tmp);
    
    mut_app.update().save_data();

    subscription
}

fn main() {
    tauri::Builder::default()
        .manage(Wrapper(Mutex::new(App::default())))
        .invoke_handler(tauri::generate_handler![
            get_savings,
            get_subscriptions,
            get_incomes,
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
            add_income
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
