#![allow(unused)]

mod draw;
mod state;

use futures::channel::mpsc::channel;
use futures::stream::StreamExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use web_sys;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
enum Error {
  WindowNotAvailable,
  DocumentNotAvailable,
  CanvasNotAvailable,
  Context2dNotAvailable
}

fn get_canvas() -> Result<web_sys::HtmlCanvasElement,Error> {
  let window = web_sys::window().ok_or(Error::WindowNotAvailable)?;
  let document = window.document().ok_or(Error::DocumentNotAvailable)?;
  document
    .get_element_by_id("canvas")
    .unwrap()
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|e| Error::CanvasNotAvailable)
}

fn get_context(
  canvas: &web_sys::HtmlCanvasElement
) -> Result<web_sys::CanvasRenderingContext2d,Error> {
  canvas
    .get_context("2d")
    .map_err(|e| Error::Context2dNotAvailable)?
    .ok_or(Error::Context2dNotAvailable)?
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .map_err(|e| Error::Context2dNotAvailable)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&JsValue::from_str("minesweeper!"));

    wasm_bindgen_futures::spawn_local(async move {
      let canvas = get_canvas().map_err(|e| JsValue::from_str(&format!("{e:?}"))).unwrap();
      let context = get_context(&canvas).unwrap();

      // channel to pass click events
      let (mut s, mut r) = channel::<(i32,i32)>(10);

      // when the user clicks the canvas, send the location into the channel
      let click_handler: Closure<dyn FnMut(web_sys::PointerEvent)>
        = Closure::new(move |evt: web_sys::PointerEvent| {
              s.try_send((evt.offset_x(), evt.offset_y())).unwrap();
          });
      canvas.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
      click_handler.forget();

      let mut game_state = state::GameState::new(20, 20);
      draw::resize_canvas(&canvas, &game_state);

      draw::draw_game(&context, &game_state);
             
      while let Some((x, y)) = r.next().await {
        let sx = (x as f64 / draw::SQUARE_SIZE) as usize;
        let sy = (y as f64 / draw::SQUARE_SIZE) as usize;
        web_sys::console::log_1(
          &JsValue::from_str(&format!("{x:?}, {y:?}, {sx:?}, {sy:?}")));

        game_state.click((sx, sy));
        draw::draw_game(&context, &game_state);
      }
      
    });

    Ok(())
}
