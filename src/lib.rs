use std::cell::RefCell;
use std::rc::Rc; 

use wasm_bindgen::prelude::*;

mod canvas;
use crate::canvas::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
	console_error_panic_hook::set_once();
	
	let document = get_document()?;	
	let canvas = get_canvas(&document, "canvas")?;
	let context = get_context(&canvas)?;
	let field = get_canvas_field(&canvas)?;
		
	let canvas_left = field.left();
	let canvas_top = field.top();

	let context_mut = Rc::new(RefCell::new(context));
	let is_drawing = Rc::new(RefCell::new(false));

	let reset_btn = create_button("reset", &document)?;
	
	attach_reset_button_handler(&reset_btn, context_mut.clone());

	on_mouse_down(
		&canvas,
		context_mut.clone(),
		is_drawing.clone(),
		canvas_left,
		canvas_top
	);	

	on_mouse_move(
		&canvas,
		context_mut.clone(),
		is_drawing.clone(),
		canvas_left,
		canvas_top
	);

	on_mouse_up(
		&canvas,
		is_drawing.clone()
	);

	
	Ok(())
}
