use crate::Wrapper;

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
