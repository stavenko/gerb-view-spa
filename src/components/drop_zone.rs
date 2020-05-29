use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew::agent::{Dispatcher, Dispatched };
use crate::redux::*;
use web_sys::{ console, DragEvent};

pub struct DropZone {
  redux: Dispatcher<Redux>,
  component: ComponentLink<Self>,
}

pub enum Message {
  DragStop(DragEvent)
}

impl Component for DropZone {
  type Message = Message;
  type Properties = ();
  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    DropZone {
      redux: Redux::dispatcher(),
      component: link,
    }
  }

  fn update(&mut self, m: Self::Message) -> bool { 
    match m {
      Message::DragStop(_evt) => {
        console::log_1(&JsValue::from_str("ok"));
        self.redux.send(Actions::ApplicationDrageStopped);
      }
    }
    false
  }

  fn rendered(&mut self, _first_render: bool) {
  }

  fn change(&mut self, _: Self::Properties) -> bool { false }

  fn view(&self) -> Html {
    let ondrop_cb =self.component.callback(|evt: DragEvent| {
      evt.prevent_default();
      Message::DragStop(evt)
    }); 


    html! {
      <span ondrop=ondrop_cb>{"drop here"}</span>
    }
  }
}

