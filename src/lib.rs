use std::{
	collections::HashMap,
	rc::Rc,
};
use wasm_bindgen::{
	prelude::*,
	JsValue,
	closure::Closure,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
	CanvasRenderingContext2d,
	Document,
	HtmlCanvasElement,
	HtmlElement,
	HtmlImageElement,
	Window,
};

use serde::Serialize;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Game
{
	_fps: u32,
	html_element: HtmlCanvasElement,
	canvas_context: CanvasRenderingContext2d,
	background: String,
	data: HashMap<String, Sprite>,
	window: Window,
	//document: Document,
	//body: HtmlElement
}

impl Default for Game {
	fn default() -> Self {
		Self::new()
	}
}

#[wasm_bindgen]
impl Game
{
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self
	{
		use web_sys::console;
		console::log_1(&"Hello World".into());

		let expect_window = "No global window".to_string();

		let window = web_sys::window().expect(&expect_window);
		let document = window.document().expect("should have a document on window");
		let body = document.body().expect("document should have a body");

		let (html_element, canvas_context) =
		Self::inicialize(document.clone(), body.clone()).unwrap();

		Self {
			html_element,
			canvas_context,
			background: String::from("white"),
			data: HashMap::new(),
			_fps: 60,
			window,
		}
	}

	fn inicialize(
		document: Document,
		body: HtmlElement,
	) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue>
	{
		let canvas: web_sys::HtmlCanvasElement = document
			.create_element("canvas")?
			.dyn_into()?;
        body.append_child(&canvas)?;

		let context = canvas
			.get_context("2d")?
			.unwrap();

		let context_2d: web_sys::CanvasRenderingContext2d =
		    context.dyn_into()?;

        Ok((canvas, context_2d))
    }

	pub async fn update(&mut self)
	{
		self.get_canvas_context().save();
		// background appears in the first layer
		Self::set_bg_color(&mut self.clone(), self.background.clone());
		// redraw sprites, sprites appears in the second layer
		let v = self.data.clone().into_values().collect();
		Self::reload_sprites(self, v).await;
	}

	pub async fn reload_sprites(&mut self, vec: Vec<Sprite>)
	{
		for mut sprite in vec
		{
			sprite.render();
		}
	}

	pub async fn force_update(&mut self)
	{
		Self::update(&mut self.clone()).await;
	}

	fn get_window_proportions(&self) -> Option<(f64, f64)>
	{Some((
		self.window.inner_width().ok()?.as_f64()?,
		self.window.inner_height().ok()?.as_f64()?,
	))}

	pub fn get_html_element(&mut self) -> HtmlCanvasElement
		{ self.html_element.to_owned() }

	pub fn get_canvas_context(&mut self) -> CanvasRenderingContext2d
		{ self.canvas_context.to_owned() }

	pub async fn resize_canvas(&mut self, aspect_ratio: f64)
	{
		let canvas = Self::get_html_element(self);
		let (old_width, old_height) = Self::get_window_proportions(self)
			.unwrap_or((0.0, 0.0));

		let (width, height);

		if old_width / old_height > aspect_ratio
		{
			width = (old_height * aspect_ratio).min(old_width);
			height = old_height;
		}
		else
		{
			width = old_width;
			height = (old_width / aspect_ratio).min(old_height);
		}

		canvas.set_width(width as u32);
		canvas.set_height(height as u32);

		let context = canvas
			.get_context("2d")
			.unwrap();

		let context_2d: web_sys::CanvasRenderingContext2d =
		context.expect("Context not found!").dyn_into().expect("error");

		context_2d.set_fill_style_str("black");
		context_2d.fill_rect(0.0, 0.0, width, height);

		Self::update(&mut self.clone()).await;
	}


    pub fn set_bg_color(&mut self, bg_color: String)
    {
        let element = Self::get_html_element(self);
        let context = Self::get_canvas_context(self);
        let (width, height) = (element.width() as f64, element.height() as f64);

        context.set_fill_style_str(bg_color.as_str());
        context.fill_rect(0.0, 0.0, width, height);

        self.background = bg_color;
    }

	pub async fn set_bg_image(&mut self, path: String)
	{
		let context = Self::get_canvas_context(self);
		let (width, height) = Self::get_window_proportions(self)
			.unwrap_or((0.0, 0.0));

		let image = Rc::new(HtmlImageElement::new().unwrap());
		image.set_src(&path);

		let onload_promise = js_sys::Promise::new(&mut |resolve, _reject| {
			let closure = Closure::wrap(Box::new(move || {
				resolve.call0(&JsValue::NULL).unwrap();
			}) as Box<dyn FnMut()>);

			image.set_onload(Some(closure.as_ref().unchecked_ref()));
			closure.forget();
		});

		let future = JsFuture::from(onload_promise);
		future.await.unwrap();

		context.translate(width / 2.0, height / 2.0).unwrap();
		context.draw_image_with_html_image_element_and_dw_and_dh(
			&image,
			-width / 2.0,
			-height / 2.0,
			width,
			height
		).unwrap();

		self.background = path;
	}

	pub fn new_image(
		&mut self,
		id: String,
		path: String,
	)
	{
		let mut sprite = Sprite::new(
			SpriteType::Image{ path },
			self.get_canvas_context(),
		);

		sprite.render();
		self.data.insert(id, sprite);
	}

	pub async fn new_text(
		&mut self,
		value: String,
		color: String,
		font: String,
	)// -> Result<Sprite, JsValue>
	{
		let font_with_size = format!("{}px {}", 16.0, font.clone());

		let font_face = format!("{} {}", 16.0, font);
		let load_promise = js_sys::Promise::resolve(&JsValue::from_str(&font_face));

		let future = JsFuture::from(load_promise);
		future.await.unwrap();

		let context = self.get_canvas_context();
		context.set_font(&font_with_size);
		context.set_fill_style_str(&color);

		let text = SpriteType::Text{
			value,
			color,
			font: font_with_size.clone(),
		};

		let mut sprite = Sprite::new(
			text,
			self.get_canvas_context(),
		);

		sprite.render();
		//Ok(sprite);
	}

	pub fn get_canvas_size(&mut self) -> Vec<u32>
	{
		let canvas = self.get_html_element();
		vec![canvas.width(), canvas.height()]
	}
}

#[derive(Clone, Serialize)]
pub enum SpriteType {
	Image{
		path: String,
	},
	Text{
		value: String,
		color: String,
		font: String,
	},
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Point
{
	pub x: f64,
	pub y: f64,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Sprite
{
	r#type: SpriteType,
	context: CanvasRenderingContext2d,
	pub position: Point,
	pub rotation: f64,
	pub size: f64,
}

impl Sprite
{
	pub fn new(
		r#type: SpriteType,
		context: CanvasRenderingContext2d,
	) -> Self
	{
		Self {
			r#type,
			position: Point { x: 0.0, y: 0.0 },
			rotation: 0.0,
			size: 1.0,
			context,
		}
	}

	pub fn render(&mut self)
	{
		match self.r#type.clone()
		{
			SpriteType::Image{ path } =>
				Self::render_texture(self, path.clone()),
			SpriteType::Text{ value, font, color } =>
				Self::render_text(self, value, font, color),
		}
	}

	fn render_texture(&mut self, path: String)
	{
		let context = self.clone().context;

		let image = Rc::new(HtmlImageElement::new().unwrap());
		let image_clone = image.clone();

		image.set_src(&path);
		let (img_h, img_w, dx, dy, size, angle) = (
			image.height() as f64,
			image.width() as f64,
			self.position.x,
			self.position.y,
			self.rotation,
			self.size,
		);

		let width = size * img_w / img_h;

		let closure = Closure::wrap(Box::new(move ||
		{
			context.save();
			context.translate(dx + width / 2.0, dy + size / 2.0).unwrap();
			context.rotate(angle.to_radians()).unwrap();
			context.draw_image_with_html_image_element_and_dw_and_dh(
				&image_clone,
				-width / 2.0,
				-size / 2.0,
				width,
				size
			).unwrap();

			context.restore();
		}) as Box<dyn FnMut()>);

		image.set_onload(Some(closure.as_ref().unchecked_ref()));
		closure.forget();
	}

	fn render_text(&mut self, value: String, font: String, color: String)
	{
		let context = self.to_owned().context;

		context.set_font(format!("{}px {}", self.size, font).as_str());
		context.set_fill_style_str(color.as_str());

		context.begin_path();
		context.fill_text(&value, self.position.x, self.position.y).unwrap();
	}
}
