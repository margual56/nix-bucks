use crate::Wrapper;

#[tauri::command]
pub fn table_subscriptions(app: tauri::State<Wrapper>) -> String {
    let mut output = String::new();

    for (uuid, subscription) in app.0.lock().unwrap().subscriptions.clone() {
        output += "<tr>";
        output += &format!("<td>{}</td>\n", subscription.name());

        output += &format!("<td>{:.2} €</td>\n", -subscription.cost());

        output += &format!("<td>{}</td>\n", subscription.recurrence());

        output += &format!(
            "<td><button class=\"delete-button\" data-uuid=\"{}\" data-table=\"table-subscriptions\" data-function=\"table_subscriptions\">
            <img src=\"/src/assets/icon-delete.svg\" alt=\"Delete\" width=\"17\" height=\"17\" />Delete</button>\n",
            uuid
        );
        output += "</tr>\n";
    }

    output
}

#[tauri::command]
pub fn table_fixed_expenses(app: tauri::State<Wrapper>) -> String {
    let mut output = String::new();

    for (uuid, expense) in app.0.lock().unwrap().fixed_expenses.clone() {
        output += "<tr>";
        output += &format!("<td>{}</td>\n", expense.name());

        output += &format!("<td>{:.2} €</td>\n", -expense.cost());

        output += &format!("<td>{}</td>\n", expense.date.to_string());

        output += &format!(
            "<td><button class=\"delete-button\" data-uuid=\"{}\" data-table=\"table-fixed-expenses\" data-function=\"table_fixed_expenses\">
            <img src=\"/src/assets/icon-delete.svg\" alt=\"Delete\" width=\"17\" height=\"17\" />
            Delete</button>\n",
            uuid
        );
        output += "</tr>\n";
    }

    output
}

#[tauri::command]
pub fn table_income(app: tauri::State<Wrapper>) -> String {
    let mut output = String::new();

    for (uuid, income) in app.0.lock().unwrap().incomes.clone() {
        output += "<tr>";
        output += &format!("<td>{}</td>\n", income.name());

        output += &format!("<td>{:.2} €</td>\n", income.cost());

        output += &format!("<td>{}</td>\n", income.recurrence());

        output += &format!(
            "<td><button class=\"delete-button\" data-uuid=\"{}\" data-table=\"table-income\" data-function=\"table_income\">
            <img src=\"/src/assets/icon-delete.svg\" alt=\"Delete\" width=\"17\" height=\"17\" />Delete</button>\n",
            uuid
        );
        output += "</tr>\n";
    }

    output
}

#[tauri::command]
pub fn table_punctual_income(app: tauri::State<Wrapper>) -> String {
    let mut output = String::new();

    for (uuid, p_income) in app.0.lock().unwrap().p_incomes.clone() {
        output += "<tr>";
        output += &format!("<td>{}</td>\n", p_income.name());

        output += &format!("<td>{:.2} €</td>\n", p_income.cost());

        output += &format!("<td>{}</td>\n", p_income.date());

        output += &format!(
            "<td><button class=\"delete-button\" data-uuid=\"{}\" data-table=\"table-punctual-income\" data-function=\"table_punctual_income\">
            <img src=\"/src/assets/icon-delete.svg\" alt=\"Delete\" width=\"17\" height=\"17\" />Delete</button>\n",
            uuid
        );
        output += "</tr>\n";
    }

    output
}
