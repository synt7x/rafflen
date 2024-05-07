use sdl2::rect::Point;
use sdl2::render::Texture;
use crate::renderer::game_text;

use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use rand;

pub struct Dancer<'a> {
    pub position: Point,
    pub animation: bool,
    pub direction: f64,
    pub name: String,
    pub x: f64, pub y: f64,
    pub movement_interval: f64,
    pub animation_interval: f64,
    pub name_plate: Texture<'a>, pub name_outline: Texture<'a>,
    pub name_x: u32, pub name_y: u32,
}

impl Dancer<'_> {
    pub fn new<'a>(font: &mut Font, texture: &'a TextureCreator<WindowContext>, name: String) -> Dancer<'a> {
        let size = font.size_of(&name).unwrap();
        let (name_plate, name_outline) = game_text(font, texture, &name);

        Dancer {
            position: Point::new(0, 0),
            animation: false,
            direction: 0.0,
            name,
            x: 500.0, y: 300.0,
            movement_interval: 0.0,
            animation_interval: 0.0,
            name_plate, name_outline,
            name_x: size.0, name_y: size.1,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        let movement_speed = 1.5 * 10000.0;
        let player_size = 48.0;

        self.movement_interval -= delta_time * 60.0;
        self.animation_interval += delta_time * 60.0;

        if self.movement_interval < 0.0 {
            self.direction = rand::random::<f64>() * 360.0;
            self.movement_interval = rand::random::<f64>();
        }

        if self.animation_interval > 0.1 {
            self.animation = !self.animation;
            self.animation_interval = 0.0;
        }

        self.x += self.direction.cos() * delta_time * movement_speed;
        self.y += self.direction.sin() * delta_time * movement_speed;

        // prevent player from escaping the screen (0, 1024) (0, 600)
        if self.x < 0.0 {
            self.x = 0.0;
        } else if self.x > 1024.0 - player_size {
            self.x = 1024.0 - player_size;

            if self.movement_interval < 0.3 {
                self.movement_interval = 0.0;
            }
        }

        if self.y < 0.0 {
            self.y = 0.0;
        } else if self.y > 600.0 - player_size {
            self.y = 600.0 - player_size;

            if self.movement_interval < 0.5 {
                self.movement_interval = 0.0;
            }
        }

        self.position = Point::new(self.x as i32, self.y as i32);
    }
}