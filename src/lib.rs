// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-arch-fm: arch fm component model.
//!
//! Real implementation: arch_fm_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchFmItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchFmError {
    EmptyArchFmItemName,
    DuplicateArchFmItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ArchFmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyArchFmItemName => write!(f, "arch_fm_item name must not be empty"),
            Self::DuplicateArchFmItem(id) => write!(f, "duplicate arch_fm_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ArchFmError {}

pub struct ArchFm {
    pub name: String,
    arch_fm_items: BTreeMap<String, ArchFmItem>,
    events: BTreeMap<String, Event>,
}

impl ArchFm {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arch_fm_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_arch_fm_item(&mut self, item: ArchFmItem) -> Result<(), ArchFmError> {
        if item.name.trim().is_empty() {
            return Err(ArchFmError::EmptyArchFmItemName);
        }
        let id = item.id.clone();
        if self.arch_fm_items.contains_key(&id) {
            return Err(ArchFmError::DuplicateArchFmItem(id.clone()));
        }
        self.arch_fm_items.insert(id, item);
        Ok(())
    }

    pub fn arch_fm_item_count(&self) -> usize {
        self.arch_fm_items.len()
    }

    pub fn get_arch_fm_item(&self, id: &str) -> Option<&ArchFmItem> {
        self.arch_fm_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ArchFmError> {
        if event.subject.trim().is_empty() {
            return Err(ArchFmError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_arch_fm_item(id: &str, name: &str) -> ArchFmItem {
        ArchFmItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ArchFm::new("ArchFm");
        assert_eq!(m.name, "ArchFm");
        assert_eq!(m.arch_fm_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_arch_fm_item() {
        let mut m = ArchFm::new("ArchFm");
        m.add_arch_fm_item(sample_arch_fm_item("a1", "Alpha")).unwrap();
        assert_eq!(m.arch_fm_item_count(), 1);
        assert!(m.get_arch_fm_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ArchFm::new("ArchFm");
        assert!(m.add_arch_fm_item(sample_arch_fm_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ArchFm::new("ArchFm");
        m.add_arch_fm_item(sample_arch_fm_item("x", "One")).unwrap();
        assert!(m.add_arch_fm_item(sample_arch_fm_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ArchFm::new("ArchFm");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
