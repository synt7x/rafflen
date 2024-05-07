use crate::dancers;

use sdl2::image::LoadTexture;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

use sdl2::ttf::Font;
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::render::Texture;

pub struct RenderContext<'a> {
    pub canvas: Canvas<Window>,
    pub texture: TextureCreator<WindowContext>,
    pub font: Font<'a, 'a>,
    pub waiting: bool,
    pub frame: u64,
}

pub struct RenderTextures<'a> {
    pub dancer: Texture<'a>,
    pub dancer_flipped: Texture<'a>,
    pub background: Texture<'a>,
}

impl RenderTextures<'_> {
    pub fn new<'a>(context: &'a TextureCreator<WindowContext>) -> RenderTextures<'a> {
        let dancer = context.load_texture("assets/dancer.png").unwrap();
        let dancer_flipped = context.load_texture("assets/dancer_flipped.png").unwrap();
        let background = context.load_texture("assets/background.png").unwrap();

        RenderTextures {
            dancer,
            dancer_flipped,
            background
        }

    }
}

pub fn render(context: &mut RenderContext,  dancers: &mut Vec<dancers::Dancer>, textures: &mut RenderTextures, delta_time: f64) {
    let delta_time_string = format!("{:.0} fps", 1.0 / (delta_time * 60.0));
    context.canvas.copy(&textures.background, None, Some(Rect::new(0, 0, 600, 600))).unwrap();
    context.canvas.copy(&textures.background, None, Some(Rect::new(600, 0, 600, 600))).unwrap();

    for dancer in dancers.iter_mut() {
        dancer.update(delta_time);
        let bounds = Some(Rect::new(if dancer.animation {128} else {0} , 0, 128, 128));
        let sprite = if dancer.direction.cos().is_sign_negative() {&textures.dancer_flipped} else {&textures.dancer};
        
        context.canvas.copy(sprite, bounds, Some(Rect::new(dancer.position.x, dancer.position.y, 48, 48))).unwrap();
        context.canvas.copy(&dancer.name_outline, None, Some(Rect::new(dancer.position.x + 21 - (dancer.name_x / 2) as i32, dancer.position.y - 27, dancer.name_x + 5, dancer.name_y + 6))).unwrap();
        context.canvas.copy(&dancer.name_plate, None, Some(Rect::new(dancer.position.x + 24 - (dancer.name_x / 2) as i32, dancer.position.y - 24, dancer.name_x, dancer.name_y))).unwrap();
    }

    debug_text(context, &delta_time_string, Point::new(5, 5));

    let width = (500.0 + (context.frame as f64 / 200.0).sin() * 50.0) as u32;
    let height = (50.0 + (context.frame as f64 / 200.0).sin() * 10.0) as u32;

    if context.waiting {
        render_outlined_text(context, "PRESS ENTER TO START", Rect::new(512 - (width as i32 / 2), 300 - (height as i32 / 2), width, height), 3, Color::RGBA(255, 255, 255, 255), Color::RGBA(12, 12, 12, 255)).unwrap();
    }
    context.frame += 1;
}

pub fn text(context: &mut RenderContext, string: &str, target: Rect, color: Color) -> Result<(), String> {
    context.font.set_outline_width(0);
    let surface = context.font
        .render(string)
        .blended(color)
        .map_err(|e| e.to_string())?;

    let texture = context.texture
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    context.canvas.copy(&texture, None, Some(target))?;
    return Ok(());
}

pub fn outlined_text<'a>(font: &mut Font, texture: &'a TextureCreator<WindowContext>, string: &str, outline_width: u16, color: Color, outline_color: Color) -> Result<(Texture<'a>, Texture<'a>), String> {
    font.set_outline_width(0);
    let surface = font
        .render(string)
        .blended(color)
        .map_err(|e| e.to_string())?;

    let text_texture = texture
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    font.set_outline_width(outline_width);
    let outline = font
        .render(string)
        .blended(outline_color)
        .map_err(|e| e.to_string())?;

    let outline_texture = texture
        .create_texture_from_surface(&outline)
        .map_err(|e| e.to_string())?;

    return Ok((text_texture, outline_texture));
}

pub fn render_outlined_text(context: &mut RenderContext, string: &str, target: Rect, outline_width: u16, color: Color, outline_color: Color) -> Result<(), String> {
    context.font.set_outline_width(0);
    let surface = context.font
        .render(string)
        .blended(color)
        .map_err(|e| e.to_string())?;

    let texture = context.texture
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    context.font.set_outline_width(outline_width);
    let outline = context.font
        .render(string)
        .blended(outline_color)
        .map_err(|e| e.to_string())?;

    let outline_texture = context.texture
        .create_texture_from_surface(&outline)
        .map_err(|e| e.to_string())?;

    context.canvas.copy(&outline_texture, None, Some(Rect::new(target.x - 3, target.y - 3, target.width() + 5, target.height() + 6))).unwrap();
    context.canvas.copy(&texture, None, Some(target)).unwrap();

    return Ok(());
}

pub fn debug_text(context: &mut RenderContext, string: &str, target: Point) -> (u32, u32) {
    let size = context.font.size_of(string).unwrap();
    let target = Rect::new(target.x, target.y, size.0, size.1);

    render_outlined_text(context, string, target, 3, Color::RGBA(255, 255, 0, 255), Color::RGBA(12, 12, 12, 255)).unwrap();
    return (target.width(), target.height());
}

pub fn game_text<'a>(font: &mut Font, texture: &'a TextureCreator<WindowContext>, string: &str) -> (Texture<'a>, Texture<'a>) {
    return outlined_text(font, texture, string, 3, Color::RGBA(255, 255, 255, 255), Color::RGBA(12, 12, 12, 255)).unwrap();
}