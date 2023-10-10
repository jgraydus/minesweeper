use web_sys;
use crate::state::GameState;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

pub const SQUARE_SIZE: f64 = 30.0;

fn calculate_canvas_size(game_state: &GameState) -> (f64, f64) {
    let xmax = game_state.width as f64 * SQUARE_SIZE + 1.0;
    let ymax = game_state.height as f64 * SQUARE_SIZE + 1.0;
    (xmax, ymax)
}

pub fn resize_canvas(canvas: &web_sys::HtmlCanvasElement, game_state: &GameState) {
    let (xmax, ymax) = calculate_canvas_size(game_state);
    canvas.set_height(ymax as u32);
    canvas.set_width(xmax as u32);
}

pub fn draw_game(context: &web_sys::CanvasRenderingContext2d, game_state: &GameState) {
    let (xmax, ymax) = calculate_canvas_size(game_state);

    // clear
    context.begin_path();
    context.move_to(0.0, 0.0);
    context.line_to(xmax, 0.0);
    context.line_to(xmax, ymax);
    context.line_to(0.0, ymax);
    context.line_to(0.0, 0.0);
    context.set_fill_style(&JsValue::from_str("white"));
    context.fill();

    // border
    context.begin_path();
    context.move_to(1.0, 1.0);
    context.line_to(1.0, ymax);
    context.line_to(xmax, ymax);
    context.line_to(xmax, 1.0);
    context.line_to(1.0, 1.0);
    context.set_stroke_style(&JsValue::from_str("black"));
    context.stroke();

    // vertical grid lines
    for v in 1..game_state.width {
      context.begin_path();
      let x = v as f64 * SQUARE_SIZE;
      context.move_to(x, 0.0);
      context.line_to(x, ymax);
      context.set_stroke_style(&JsValue::from_str("black"));
      context.stroke();
    }

    // horizontal grid lines
    for h in 1..game_state.width {
      context.begin_path();
      let y = h as f64 * SQUARE_SIZE;
      context.move_to(0.0, y);
      context.line_to(xmax, y); 
      context.set_stroke_style(&JsValue::from_str("black"));
      context.stroke();
    }

    for x in 0..game_state.width {
      for y in 0..game_state.height {
        let x0 = x as f64 * SQUARE_SIZE + 2.0;
        let x1 = x0 + SQUARE_SIZE - 4.0;
        let y0 = y as f64 * SQUARE_SIZE + 2.0;
        let y1 = y0 + SQUARE_SIZE - 4.0;
        if game_state.revealed_squares.contains(&(x,y)) {
          context.set_font("20pt sans-serif");
          context.set_fill_style(&JsValue::from_str("black"));
          let n = game_state.neighboring_bombs.get(&(x,y)).unwrap();
          if *n != 0 {
            context.fill_text(&format!("{}",n), x0 + 6.0, y0 + 22.0).unwrap();
          }
        } else {
          context.begin_path();
          context.move_to(x0,y0);
          context.line_to(x1, y0);
          context.line_to(x1,y1);
          context.line_to(x0,y1);
          context.line_to(x0, y0);
          context.set_fill_style(&JsValue::from_str("gray"));
          context.fill();
        }
      }
    }
}

