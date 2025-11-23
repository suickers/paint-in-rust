use std::cell::RefCell;
use std::rc::Rc; 

use web_sys::{
	window,
	MouseEvent,
	HtmlCanvasElement,
	CanvasRenderingContext2d,
	Element,
	Document,
	DomRect,
};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;



pub fn get_document() -> Result<Document, JsValue> {
	window()
		.ok_or(JsValue::from_str("window value was None"))?
		.document()
		.ok_or(JsValue::from_str("Document value was None"))
}

pub fn get_canvas(
	document: &Document,
	element_id: &str
) -> Result<HtmlCanvasElement, JsValue> {

	let canvas = document
		.get_element_by_id(element_id)
		.ok_or(JsValue::from_str("Could not find element ID."))?;

	canvas
		.dyn_into::<HtmlCanvasElement>()
		.map_err(|_| JsValue::from_str("Failed casting into Html Canvas Element."))
}

pub fn get_context(
	canvas: &HtmlCanvasElement
) -> Result<CanvasRenderingContext2d, JsValue> {
	let context_object = canvas
		.get_context("2d")
		.map_err(|_| JsValue::from_str("Failed to get context"))?
		.ok_or(JsValue::from_str("Context was a None value"))?;

	context_object
		.dyn_into::<CanvasRenderingContext2d>()
		.map_err(|_| JsValue::from_str("Failed to cast into CanvasRenderingObject2d"))
		
}

pub fn get_canvas_field(
	canvas: &HtmlCanvasElement
) -> Result<DomRect, JsValue> {
	let field = canvas
		.dyn_ref::<Element>()
		.ok_or(JsValue::from_str("Failed to get Element ref"))?;

	Ok(field.get_bounding_client_rect())
}

pub fn on_mouse_down(
	canvas: &HtmlCanvasElement,
	context_mut: Rc<RefCell<CanvasRenderingContext2d>>,
	is_drawing: Rc<RefCell<bool>>,
	canvas_left: f64,
	canvas_top: f64
) {

	let ms_dn = Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
				
		let canvas_x = mouse_event.client_x() as f64 - canvas_left;
		let canvas_y = mouse_event.client_y() as f64 - canvas_top;

		*is_drawing.borrow_mut() = true;
		context_mut.borrow_mut().begin_path();
		context_mut.borrow_mut().move_to(canvas_x, canvas_y);

	})as Box<dyn FnMut(_)>);
	canvas.set_onmousedown(Some(ms_dn.as_ref().unchecked_ref()));
	ms_dn.forget();
}

pub fn on_mouse_move(
	canvas: &HtmlCanvasElement,
	context_mut: Rc<RefCell<CanvasRenderingContext2d>>,
	is_drawing: Rc<RefCell<bool>>,
	canvas_left: f64,
	canvas_top: f64
) {
	let ms_mv = Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
		
		let canvas_x = mouse_event.client_x() as f64 - canvas_left;
		let canvas_y = mouse_event.client_y() as f64 - canvas_top;
		let context_mut_borrow = context_mut.borrow_mut();
		
		if *is_drawing.borrow() {
			context_mut_borrow
				.line_to(canvas_x, canvas_y);
				
			context_mut_borrow
				.stroke();
		}
		
	})as Box<dyn FnMut(_)>);
	canvas.set_onmousemove(Some(ms_mv.as_ref().unchecked_ref()));
	ms_mv.forget();
}

pub fn on_mouse_up(
	canvas: &HtmlCanvasElement,
	is_drawing: Rc<RefCell<bool>>
) {
	let ms_up = Closure::wrap(Box::new(move |_: MouseEvent| {
			
		*is_drawing.borrow_mut() = false;
		
	})as Box<dyn FnMut(_)>);
	canvas.set_onmouseup(Some(ms_up.as_ref().unchecked_ref()));
	ms_up.forget();
}
