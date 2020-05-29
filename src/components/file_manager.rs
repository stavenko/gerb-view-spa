use yew::prelude::*;
use yew::agent::{ Bridge};
use crate::redux::*;
use crate::components::drop_zone::DropZone;
use crate::components::file_list::FileList;

pub struct FileManager {
  _updater: Box<dyn Bridge<Redux>>,
  is_new_files_dragged: bool,
}

pub enum Message {
  NewState(State)
}


impl Component for FileManager {
  type Message = Message;
  type Properties = ();
  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(|s: State| Message::NewState(s));

    FileManager {
      is_new_files_dragged: false,
      _updater: Redux::bridge(callback),
    }
  }

  fn update(&mut self, m: Self::Message) -> bool { 
    match m {
      Message::NewState(state) => {
        let needs_update = self.is_new_files_dragged != state.is_something_dragged;
        self.is_new_files_dragged = state.is_something_dragged;
        needs_update
      }
    }
  }


  fn change(&mut self, _: Self::Properties) -> bool { false }

  fn view(&self) -> Html {
    if self.is_new_files_dragged {
      html! {<DropZone /> }
    } else {
      html! {<FileList /> }
    }
  }
}

