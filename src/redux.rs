use serde::{Serialize, Deserialize};
use yew::worker::{Context, HandlerId, AgentLink, Agent};
use std::collections::HashSet;
use web_sys::console;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Actions {
  ApplicationDragStarted,
  ApplicationDrageStopped
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State{
  pub is_something_dragged: bool,
  amount_of_drag_enter: u32
}


pub struct Redux {
  link: AgentLink<Self>,
  subscribers: HashSet<HandlerId>,
  state: Box<State>
               
}

impl Redux {
  fn propagate_state(&self) {
    for sub in self.subscribers.iter() {
      self.link.respond(*sub, *self.state.clone())
    }
  }

  fn update_state(&mut self, msg: Actions, _id: HandlerId) {
    match msg {
      Actions::ApplicationDragStarted => {
        console::log_1(&JsValue::from_str(">>>>"));

        let state = State { 
          amount_of_drag_enter: self.state.amount_of_drag_enter + 1,
          is_something_dragged: true, 
          ..*self.state
        };
        self.state = Box::new(state);
      }
      Actions::ApplicationDrageStopped => {
        console::log_1(&JsValue::from_str("<<<<"));
        let amount_of_drag_enter = self.state.amount_of_drag_enter - 1;
        if amount_of_drag_enter == 0 {
          self.state = Box::new( State {
            is_something_dragged: false,
            amount_of_drag_enter: 0,
            ..*self.state
          });
        } else {
          self.state = Box::new(State{ amount_of_drag_enter, ..*self.state});
        }
      }
    }
  }
}

impl Agent for Redux {
  type Reach = Context;
  type Message = ();
  type Input = Actions;
  type Output = State;

  fn create(link: AgentLink<Self>) -> Self {
    Redux {
      link,
      subscribers: HashSet::new(),
      state: Box::new(
      State {
        is_something_dragged: false,
        amount_of_drag_enter: 0
      })
    }
  }

  fn update(&mut self, _: Self::Message) { }

  fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
    self.update_state(msg, id);
    self.propagate_state();
  }

  fn connected(&mut self, id: HandlerId) {
    self.subscribers.insert(id);
  }

  fn disconnected(&mut self, id: HandlerId) {
    self.subscribers.remove(&id);
  }

}
