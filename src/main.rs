mod game_pack;
mod script;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashMap;
use std::io::Result as IOResult;

fn draw_dialog(
    saying: &str,
    character: Option<String>,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font_name: &str,
    screen_size: Point,
) {
    const BOARDER_SIZE_X: i32 = 100;
    const BOARDER_SIZE_Y: i32 = 400;
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(font_name, 24).unwrap();
    let text_creator = canvas.texture_creator();

    let mut x = BOARDER_SIZE_X;
    let mut y = BOARDER_SIZE_Y;
    for word in saying.chars() {
        /* turn to a new line */
        if x > screen_size.x() - BOARDER_SIZE_X {
            x = BOARDER_SIZE_X;
            y += font.size_of_char(word).unwrap().1 as i32;
        }
        let texture = text_creator
            .create_texture_from_surface(
                font.render(&String::from(word))
                    .blended(Color::RGB(255, 255, 255))
                    .unwrap(),
            )
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Rect::new(x, y, texture.query().width, texture.query().height),
            )
            .unwrap();
        x += texture.query().width as i32;
    }

    /* draw character name */
    if character.is_some() {
        let font = ttf_context.load_font(font_name, 36).unwrap();
        let texture = text_creator
            .create_texture_from_surface(
                font.render(&character.unwrap())
                    .blended(Color::RGB(255, 255, 255))
                    .unwrap(),
            )
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Rect::new(
                    BOARDER_SIZE_X,
                    300,
                    texture.query().width,
                    texture.query().height,
                ),
            )
            .unwrap();
    }
    canvas.present();
}

fn draw_background(
    image_bytes: &[u8],
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    screen_size: Point,
) {
    let text_creator = canvas.texture_creator();
    let texture = text_creator.load_texture_bytes(image_bytes).unwrap();
    canvas
        .copy(
            &texture,
            None,
            Rect::new(0, 0, screen_size.x() as u32, screen_size.y() as u32),
        )
        .unwrap();
    canvas.present();
}

pub fn main() -> IOResult<()> {
    let mut current_background = String::new();
    let args: Vec<String> = std::env::args().collect();
    let pack_path = args[1].clone();
    let pack = game_pack::GamePack::open(&pack_path)?;
    let mut script = script::Script::new(pack);
    script.parse(&script.pack.get_config("start").unwrap())?;

    let config: HashMap<String, String> =
        serde_json::from_str(&std::fs::read_to_string("config.json").unwrap()).unwrap();

    let content = sdl2::init().unwrap();
    let video = content.video().unwrap();
    let window = video.window("31Gal", 800, 600).build().unwrap();
    let mut events = content.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::MouseButtonUp { .. } => {
                    let instruction = script.step().clone();
                    match instruction {
                        script::Instruction::Say { saying, character } => {
                            canvas.clear();
                            if current_background != "" {
                                let bytes = script.pack.get_resource(&current_background).unwrap();
                                draw_background(&bytes, &mut canvas, Point::new(800, 600));
                            }
                            draw_dialog(
                                &saying,
                                character.clone(),
                                &mut canvas,
                                &config["font"],
                                Point::new(800, 600),
                            );
                        }
                        script::Instruction::Scene { resource } => {
                            let bytes = script.pack.get_resource(&resource).unwrap();
                            draw_background(&bytes, &mut canvas, Point::new(800, 600));

                            current_background = resource.to_owned();
                        }
                    }
                }
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(10));
    }
    Ok(())
}
