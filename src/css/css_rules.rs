
#[macro_export]
macro_rules! css_rule {
  ($($f: tt)*) => {
    {
      let mut left = Vec::<String>::new();
      let mut right = Vec::<String>::new();
      let mut selector_finished = false;
      let mut result = HashMap::<String, String>::new();
      $(
        let token: &str = stringify!($f);
        match token {
          ":" => {selector_finished = true},
            
          ";" => {
            selector_finished = false;
            let selector = left.iter().fold(String::new(), |acc, s| acc + s);
            let rule = right.iter().fold(String::new(), |acc, s| acc + s);
            result.insert(selector, rule);
            left.clear();
            right.clear();
          },
          _ => {
            if !selector_finished {
              left.push(String::from(token))
            }else {
              right.push(String::from(token))
            }
          },
        }
      )*
      result
    }
  }
}


