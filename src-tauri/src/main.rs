// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nix_bucks::{App, Wrapper, getters::*, tables::*, TmpSubscription};
use std::sync::Mutex;
use uuid::Uuid;

#[tauri::command]
fn delete_uuid(app: tauri::State<Wrapper>, uuid_str: String) {
    let uuid = Uuid::parse_str(&uuid_str).unwrap();
    let mut my_app = app.0.lock().unwrap();

    *my_app = my_app.clone().remove_from_uuid(uuid);

    my_app.save_data();
}

#[tauri::command]
fn add_subscription(app: tauri::State<Wrapper>, tmp: TmpSubscription) {
    let mut mut_app = app.0.lock().unwrap();

    *mut_app = mut_app.clone().add_subscription(tmp).update();
    
    mut_app.save_data();
}

fn main() {
    tauri::Builder::default()
        .manage(Wrapper(Mutex::new(App::default())))
        .invoke_handler(tauri::generate_handler![
            get_savings,
            get_subscriptions,
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
            add_subscription
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
