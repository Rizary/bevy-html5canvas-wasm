use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;
use log;
use std::rc::Rc;
use web_sys::{Window, Document, History};

thread_local! {
    static WINDOW: Rc<Window> = Rc::new(web_sys::window().unwrap_throw());
    static DOCUMENT: Document = WINDOW.with(|w| w.document().unwrap_throw());
    static HISTORY: History = WINDOW.with(|w| w.history().unwrap_throw());
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
  setup_logger();
  
  log::info!("creating bevy world");
  log::info!("starting run loop on window {:?}", WINDOW);

  //   let context = Context::new(WINDOW.clone());
  //   let ui_manager = GameUiManager::new(&context);
  //   let app_shared = Shared::new(App {
  //     context: context,
  //     ui_manager: ui_manager,
  //     fps_tracker: FpsTracker::new(),
  //   });
  
  //   register_on_error_listener(&window);
  //   register_on_visibility_change_listener(&window, app_shared.clone());
  //   register_input_listeners(&window, app_shared.clone());
  
  //   console::log_1(&JsValue::from_str("Starting run_loop(app_shared)!"));
  //   run_loop(app_shared);
  log::info!("starting run loop");
  Ok(())
}

cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
        fn setup_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            log::info!("rust logging enabled!!!");
            console_error_panic_hook::set_once();
        }
    } else {
        fn setup_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}