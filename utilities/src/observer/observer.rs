use crate::observer::event::Event;

pub trait Observer: Send + Sync {
    type Event: Event;

    fn update(&self, event: &Self::Event);
}
