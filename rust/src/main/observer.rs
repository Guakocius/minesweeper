use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Load,
    Save,
}

pub type Subscriber = fn();

#[derive(Default)]
pub struct Observer {
    events: HashMap<Event, Vec<Subscriber>>,
}

impl Observer {
    pub fn subscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|&s| s != listener);
    }

    pub fn notify(&self, event_type: Event) {
        let listeners = self.events.get(&event_type).unwrap();
        for listener in listeners {
            //listener
        }
    }
}