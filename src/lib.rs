pub mod redux;
pub mod components;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::utils::{document, window};
use yew::agent::Dispatched;
use web_sys::console;
use web_sys::{ Event };
use redux::{Actions, Redux};
use components::layout::*;


fn prevent_default(evt: Event) {
  evt.prevent_default();
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello fucker!"));
  

    let on_start = Closure::wrap(
      Box::new(
        move || Redux::dispatcher().send(Actions::ApplicationDragStarted)
      ) as Box<dyn FnMut()>
    );
    let on_end = Closure::wrap(
      Box::new(
        move || {
          console::log_1(&JsValue::from_str("======"));
          Redux::dispatcher().send(Actions::ApplicationDrageStopped)
        }
      ) as Box<dyn FnMut()>
    );

    let prevent_default_closure = Closure::wrap(Box::new(move |evt: Event| prevent_default(evt)) as Box<dyn FnMut(_)>);

    for evt in vec!["dragleave", "drop", "dragenter", "dragover"] {
      console::log_1(&JsValue::from_str(evt));
      document().add_event_listener_with_callback(
        evt, 
        prevent_default_closure.as_ref().unchecked_ref()
        )?;
      window().add_event_listener_with_callback(evt, prevent_default_closure.as_ref().unchecked_ref())?;
    }

    window().add_event_listener_with_callback("dragleave", on_end.as_ref().unchecked_ref())?;
    window().add_event_listener_with_callback("dragend", on_end.as_ref().unchecked_ref())?;
    window().add_event_listener_with_callback("drop", on_end.as_ref().unchecked_ref())?;
    window().add_event_listener_with_callback("dragenter", on_start.as_ref().unchecked_ref())?;
    prevent_default_closure.forget();
    on_start.forget();
    on_end.forget();


    App::<Layout>::new().mount_to_body();

    Ok(())
}
