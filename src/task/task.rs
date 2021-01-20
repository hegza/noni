use crate::Time;

use super::{
    display_settings::DisplaySettings, flag::Flag, tag::Tag, trigger_rule::TriggerRule,
    triggerings::Triggerings,
};
use serde::{Deserialize, Serialize};

/// This is the main data-structure of the program
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    /// Main display text. Might get truncated for display.
    summary: String,
    /// More details, perhaps multi-line if necessary.
    details: String,
    /// How often and when the event's triggered.
    trigger_rule: TriggerRule,
    /// Temporary adjustments
    override_next_trigger: Time,
    /// Things pertaining to how to show this task on screen.
    display_settings: DisplaySettings,
    /// User-editable comments
    comments: Vec<String>,
    /// Temporary comments that vanish after all triggerings are completed.
    post_it_notes: Vec<String>,
    /// Tags for searching
    tags: Vec<Tag>,
    /// Flags added by system.
    flags: Vec<Flag>,
    /// Triggerings and their completions. We serialize this last to avoid cluttering screen-space when editing the event file manually.
    triggerings: Triggerings,
}
