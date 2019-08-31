pub enum AppAction {
    Task,
    Confirmation,
    Notification
}
pub fn inspect(app_action: AppAction) -> String {
    match app_action {
        AppAction::Task => format!("Task"),
        AppAction::Confirmation => format!("Confirmation"),
        AppAction::Notification => format!("Notification")
    }
}
