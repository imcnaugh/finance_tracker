use crate::event::event::Event;

pub trait Observer: Send + Sync {
    type Event: Event;

    fn update(&self, event: &Self::Event);
}
