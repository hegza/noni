use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Settings {
    /// Delete one-shot events after they've been completed.
    delete_one_shot_events: bool,
}
