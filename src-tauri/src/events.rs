pub enum AppEvent {
    UpdateUserSettings,
    RefreshGame,
}

impl From<AppEvent> for &'static str {
    fn from(event: AppEvent) -> Self {
        match event {
            AppEvent::UpdateUserSettings => "update_user_settings",
            AppEvent::RefreshGame => "refresh_game",
        }
    }
}
