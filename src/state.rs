use tokio::sync::{futures::Notified, Notify};

pub struct AppState {
    notifier: Notify,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            notifier: Notify::new(),
        }
    }

    pub fn notify_waiters(&self) {
        self.notifier.notify_waiters();
    }

    pub fn notify_one(&self) {
        self.notifier.notify_one()
    }

    pub fn notified(&self) -> Notified<'_> {
        self.notifier.notified()
    }
}
