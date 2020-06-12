use std::collections::HashMap;
use std::string::String;
use std::ops::Index;
use yew::utils::*;
use crate::rand::rand;
pub mod css_rules;

pub type Selector = String;
pub type Rule = String;
pub type ClassName = String;
pub type Rules = HashMap<Selector, Rule>;

pub struct ComponentClasses {
  component_name: String,
  classes: HashMap<ClassName, Rules>,
  name_map: HashMap<ClassName, String>
}

impl ComponentClasses {
  pub fn new(component_name: &str) -> Self {
    ComponentClasses {
      classes: HashMap::new(),
      component_name: String::from(component_name),
      name_map: HashMap::new()
    }
  }

  pub fn add_class(&mut self, name: &str, rules: Rules) -> &mut ComponentClasses {
    let class = String::from(name);
    self.classes.insert(class.clone(), rules);
    self.name_map.insert(class.clone(), self.create_name(&class));
    self
  }

  fn create_name(&self, class: &String ) -> String {
    let mut rand_string = String::new();

    for _ in 1..5 {
      let v = (rand() * 16.0).floor() as u8;
      rand_string += &format!("{:x}", v);
    }
    self.component_name.clone() + "-" + class + "-" + &rand_string
  }

  fn prepare_content(&self) -> String {
    let mut css_string = String::new();
    for (class, rules) in self.classes.iter() {
      let actual_class = self.name_map[class].clone();
      css_string += &format!(".{}", actual_class);
      css_string += " {\n";
      for (rule, value) in rules {
        css_string += &format!("{}: {}; \n", rule, value);
      }
      css_string += "}\n";
    }
    css_string
  }

  pub fn populate(&self) {
    let style_el = match document().get_element_by_id(&self.component_name) {
      Some(style_el) => style_el,
      None => {
        let style_el = document().create_element("style").unwrap();
        style_el.set_id(&self.component_name);
        match document().body() {
          Some(body) => {
            body.append_child(&style_el).expect("Could not append child");
          }
          None => {}
        };
        style_el
      }
    };

    style_el.set_text_content(Some(&self.prepare_content()));
  }

  pub fn cls(&self, cls: &str) -> String {
    self.name_map[cls].clone()
  }
}
