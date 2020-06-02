use yew::prelude::*;
use crate::components::file_manager::FileManager;
use crate::css::{ComponentClasses};
use crate::css_rule;
use std::collections::HashMap;

pub struct Layout {
  classes: ComponentClasses

}
#[derive(Properties, Clone)]
pub struct Props {
}

impl Component for Layout {
  type Message = ();
  type Properties = ();
  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    let mut cc = ComponentClasses::new("layout");
    cc.add_class("main-layout", css_rule! {
      width: 800px;
      margin: auto;
      grid-template-columns: repeat(4, 1fr);
      grid-gap: 10px;
      display: grid;
    })
    .add_class("section-layout", css_rule! {
      background-color: #ffaaaa;
      grid-column-start: 2;
      grid-column-end: -1;
    })
    .add_class("aside-layout", css_rule! {
      grid-column: 1;
      background-color: #fafffa;
    })
    .add_class("header-layout", css_rule! {
      grid-column: 1 / -1;
      background-color: #fafaffk;
    });

    cc.populate();
    
    Layout {
      classes: cc
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
      <main class=self.classes.cls("main-layout")>
        <header class=self.classes.cls("header-layout")>{"header"} </header>
        <aside class=self.classes.cls("aside-layout")>
          <FileManager />
        </aside>
        <main class=self.classes.cls("section-layout")> {"main screen"} </main>
      </main>
    }
  }
}

