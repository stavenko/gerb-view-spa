use yew::prelude::*;
use yew::agent::Bridge;
use crate::redux::*;

pub struct FileList {
  // redux: Dispatcher<Redux>,
  _updater: Box<dyn Bridge<Redux>>,
  _component: ComponentLink<Self>,
}

pub enum Message {
  NewState(State)
}

impl Component for FileList {
  type Message = Message;
  type Properties = ();
  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(|s: State| Message::NewState(s));

    FileList {
      _updater: Redux::bridge(callback),
      _component: link,
    }
  }

  fn update(&mut self, _: Self::Message) -> bool { 
    false
  }

  fn change(&mut self, _: Self::Properties) -> bool { false }

  fn view(&self) -> Html {
    html! {
      <span> {"hey, I am file list"} </span>
    }
  }
}
