use log::*;
use std::cell::{Ref, RefCell, RefMut, UnsafeCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlBodyElement, KeyboardEvent};

#[derive(Debug)]
pub struct Body {
    pub events: Rc<RefCell<Vec<String>>>,
}

impl Body {
    pub fn new() -> Self {
        Self {
            events: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn pop(&mut self) -> String {
//         let mut vecs = self.events.borrow_mut();
 //       vecs.pop().unwrap()
        "qwe".to_string()
    }

    pub fn attach_events(&mut self, events: &mut Box<Vec<String>>) {
        let vec = Box::new(events);

        let handler = Box::new(|e: KeyboardEvent| {
//            vec.push("wer".to_string());
            debug!("event: {:?}", e.key_code());
        });
        
        let closure = Closure::new(handler as Box<dyn FnMut(_)>);

        window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("body")
            .unwrap()
            .dyn_into::<HtmlBodyElement>()
            .unwrap()
            .add_event_listener_with_callback("keydown", &closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();
    }
}
