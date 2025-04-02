use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct MenuItem {
    pub label: Option<String>,
    pub disabled: Option<bool>,
    pub shortcut: Option<String>,
    pub event: Option<String>,
    pub payload: Option<String>,
    pub subitems: Option<Vec<MenuItem>>,
    pub icon: Option<MenuItemIcon>,
    pub checked: Option<bool>,
    pub is_separator: Option<bool>,
}

#[derive(Clone, Deserialize)]
pub struct MenuItemIcon {
    pub path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            label: None,
            disabled: Some(false),
            shortcut: None,
            event: None,
            payload: None,
            subitems: None,
            icon: None,
            checked: Some(false),
            is_separator: Some(false),
        }
    }
}

// 👇 Add this block below your existing code
impl MenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: Some(label.to_string()),
            ..Default::default()
        }
    }

    pub fn separator() -> Self {
        Self {
            is_separator: Some(true),
            ..Default::default()
        }
    }

    // Add this method to support the with_id functionality
    pub fn with_id(mut self, id: &str) -> Self {
        // Store the ID in the event field, or create a new id field if needed
        self.event = Some(id.to_string());
        self
    }
}

