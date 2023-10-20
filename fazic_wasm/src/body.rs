use log::*;
use std::cell::{Ref, RefCell, RefMut, UnsafeCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlBodyElement, Event};

#[derive(Debug)]
pub struct Body {
    pub events: Vec<String>
}

impl Body {
    pub fn new() -> Self {
        Self {
            events: vec![]
        }
    }

    pub fn pop(&mut self) -> String {
        self.events.pop().unwrap()
    }

    pub fn attach_events(&mut self) {
        let handler = Closure::wrap(Box::new(|e: Event| {
            //self.events.push("wer".to_string());
            debug!("event: {:?}", e);
        }) as Box<dyn FnMut(_)>);

        window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("body")
            .unwrap()
            .dyn_into::<HtmlBodyElement>()
            .unwrap()
            .add_event_listener_with_callback("keydown", &handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();
    }
}
