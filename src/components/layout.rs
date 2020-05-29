use yew::prelude::*;
use crate::components::file_manager::FileManager;

pub struct Layout {
}

impl Component for Layout {
  type Message = ();
  type Properties = ();
  fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Layout {
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <main>
        <header>{"header"} </header>
        <aside>
          <FileManager />
        </aside>
        <main> {"main screen"} </main>
      </main>
    }
  }
}

