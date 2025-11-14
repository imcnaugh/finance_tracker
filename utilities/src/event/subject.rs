use crate::event::event::Event;
use crate::event::observer::Observer;

pub trait Subject<E: Event> {
    fn attach(&mut self, observer: Box<dyn Observer<Event = E>>);
    fn detach(&mut self, observer: &Box<dyn Observer<Event = E>>);
    fn notify(&self, event: &E);
}

pub struct SimpleSubject<E: Event> {
    observers: Vec<Box<dyn Observer<Event = E>>>,
}

impl<E: Event> SimpleSubject<E> {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}

impl<E: Event> Subject<E> for SimpleSubject<E> {
    fn attach(&mut self, observer: Box<dyn Observer<Event = E>>) {
        self.observers.push(observer);
    }

    fn detach(&mut self, observer: &Box<dyn Observer<Event = E>>) {
        self.observers.retain(|o| !std::ptr::eq(o, observer))
    }

    fn notify(&self, event: &E) {
        self.observers.iter().for_each(|o| o.update(event));
    }
}
