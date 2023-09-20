use crate::Wrapper;
use crate::format_money;

#[tauri::command]
pub fn get_savings(app: tauri::State<Wrapper>) -> f32 {
    app.0.lock().unwrap().initial_savings
}

#[tauri::command]
pub fn montly_cost(app: tauri::State<Wrapper>) -> f32 {
    -app.0.lock().unwrap().monthly_costs()
}

#[tauri::command]
pub fn eoy_cost(app: tauri::State<Wrapper>) -> f32 {
    -app.0.lock().unwrap().yearly_costs()
}

#[tauri::command]
pub fn eoy_income(app: tauri::State<Wrapper>) -> f32 {
    app.0.lock().unwrap().yearly_income()
}

#[tauri::command]
pub fn eoy_balance(app: tauri::State<Wrapper>) -> f32 {
    app.0.lock().unwrap().yearly_balance()
}

#[tauri::command]
pub fn eom_balance(app: tauri::State<Wrapper>) -> f32 {
    app.0.lock().unwrap().monthly_balance()
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExportedSubscription {
    name: String,
    cost: String,
    recurrence: String,
}

#[tauri::command]
pub fn get_subscriptions(app: tauri::State<Wrapper>) -> Vec<ExportedSubscription> {
    app.0.lock().unwrap().subscriptions.clone()
        .into_values()
        .map(|s|
             ExportedSubscription {
                 name: String::from(s.name()),
                 cost: format_money(-s.cost()),
                 recurrence: s.recurrence().to_string(),
             }
        ).collect()
}
