// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-user-daemons: user daemons component model.
//!
//! Real implementation: daemons, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Daemon {
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
pub enum UserDaemonsError {
    EmptyDaemonName,
    DuplicateDaemon(String),
    EmptyEventSubject,
}

impl fmt::Display for UserDaemonsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDaemonName => write!(f, "daemon name must not be empty"),
            Self::DuplicateDaemon(id) => write!(f, "duplicate daemon {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for UserDaemonsError {}

pub struct UserDaemons {
    pub name: String,
    daemons: BTreeMap<String, Daemon>,
    events: BTreeMap<String, Event>,
}

impl UserDaemons {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            daemons: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_daemon(&mut self, item: Daemon) -> Result<(), UserDaemonsError> {
        if item.name.trim().is_empty() {
            return Err(UserDaemonsError::EmptyDaemonName);
        }
        let id = item.id.clone();
        if self.daemons.contains_key(&id) {
            return Err(UserDaemonsError::DuplicateDaemon(id.clone()));
        }
        self.daemons.insert(id, item);
        Ok(())
    }

    pub fn daemon_count(&self) -> usize {
        self.daemons.len()
    }

    pub fn get_daemon(&self, id: &str) -> Option<&Daemon> {
        self.daemons.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), UserDaemonsError> {
        if event.subject.trim().is_empty() {
            return Err(UserDaemonsError::EmptyEventSubject);
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

    fn sample_daemon(id: &str, name: &str) -> Daemon {
        Daemon {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = UserDaemons::new("UserDaemons");
        assert_eq!(m.name, "UserDaemons");
        assert_eq!(m.daemon_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_daemon() {
        let mut m = UserDaemons::new("UserDaemons");
        m.add_daemon(sample_daemon("a1", "Alpha")).unwrap();
        assert_eq!(m.daemon_count(), 1);
        assert!(m.get_daemon("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = UserDaemons::new("UserDaemons");
        assert!(m.add_daemon(sample_daemon("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = UserDaemons::new("UserDaemons");
        m.add_daemon(sample_daemon("x", "One")).unwrap();
        assert!(m.add_daemon(sample_daemon("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = UserDaemons::new("UserDaemons");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
