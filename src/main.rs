mod game_pack;
mod script;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::collections::HashMap;
use std::io::Result as IOResult;

use script::Instruction;

#[derive(Default)]
struct Content {
    character: Option<String>,
    saying: String,
    scene: String,
}

fn draw_dialog(
    saying: &str,
    character: Option<String>,
    canvas: &mut Canvas<sdl2::video::Window>,
    font_name: &str,
) {
    const BOARDER_SIZE_PERCENT_X: f32 = 0.1;
    const BOARDER_SIZE_PERCENT_Y: f32 = 0.8;
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(font_name, 24).unwrap();
    let text_creator = canvas.texture_creator();

    let mut x = (BOARDER_SIZE_PERCENT_X * canvas.window().size().0 as f32) as u32;
    let mut y = (BOARDER_SIZE_PERCENT_Y * canvas.window().size().1 as f32) as u32;
    /* draw saying */
    for word in saying.chars() {
        /* turn to a new line */
        if x > canvas.window().size().0
            - (BOARDER_SIZE_PERCENT_X * canvas.window().size().0 as f32) as u32
        {
            x = (BOARDER_SIZE_PERCENT_X * canvas.window().size().0 as f32) as u32;
            y += font.size_of_char(word).unwrap().1;
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
                Rect::new(
                    x as i32,
                    y as i32,
                    texture.query().width,
                    texture.query().height,
                ),
            )
            .unwrap();
        x += texture.query().width;
    }

    /* draw character name */
    if let Some(character) = character {
        let font = ttf_context.load_font(font_name, 36).unwrap();
        let texture = text_creator
            .create_texture_from_surface(
                font.render(&character)
                    .blended(Color::RGB(255, 255, 255))
                    .unwrap(),
            )
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Rect::new(
                    (BOARDER_SIZE_PERCENT_X * canvas.window().size().0 as f32) as i32,
                    (BOARDER_SIZE_PERCENT_Y * canvas.window().size().1 as f32) as i32,
                    texture.query().width,
                    texture.query().height,
                ),
            )
            .unwrap();
    }
    canvas.present();
}

fn draw_background(image_bytes: &[u8], canvas: &mut Canvas<sdl2::video::Window>) {
    let text_creator = canvas.texture_creator();
    let texture = text_creator.load_texture_bytes(image_bytes).unwrap();
    canvas
        .copy(
            &texture,
            None,
            Rect::new(0, 0, canvas.window().size().0, canvas.window().size().1),
        )
        .unwrap();
    canvas.present();
}

pub fn main() -> IOResult<()> {
    let args: Vec<String> = std::env::args().collect();
    let pack_path = args[1].clone();
    let pack = game_pack::GamePack::open(&pack_path)?;
    let mut script = script::Script::new(pack);
    script.parse(
        &script
            .pack
            .get_config("start")
            .expect("'start' does not defined in package.json."),
    )?;

    let config: HashMap<String, String> =
        serde_json::from_str(&std::fs::read_to_string("config.json")?)?;

    let content = sdl2::init().unwrap();
    let video = content.video().unwrap();
    let window = video.window("31Gal", 800, 600).resizable().build().unwrap();
    let mut events = content.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut content = Content::default();

    if let Some(title) = script.pack.get_config("title") {
        canvas.window_mut().set_title(&title).unwrap();
    }

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::MouseButtonUp { .. } => {
                    let instruction = script.step().clone();
                    match instruction {
                        Instruction::Say { saying, character } => {
                            canvas.clear();
                            if !content.scene.is_empty() {
                                let bytes = script.pack.get_resource(&content.scene)?;
                                draw_background(&bytes, &mut canvas);
                            }
                            draw_dialog(
                                &saying,
                                character.clone(),
                                &mut canvas,
                                &config
                                    .get("font")
                                    .expect("'font' is not defined in config.json"),
                            );

                            content.saying = saying;
                            content.character = character;
                        }
                        Instruction::Scene { resource } => {
                            let bytes = script.pack.get_resource(&resource)?;
                            draw_background(&bytes, &mut canvas);

                            content.scene = resource;
                        }
                        Instruction::Switch { label } => {
                            let step = script.get_label(&label);
                            script.switch_to(step.expect(&format!("'{}' is not defined", label)));
                        }
                        _ => {}
                    }
                }
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(_x, _y),
                    ..
                } => {
                    canvas.clear();
                    if let Ok(bytes) = script.pack.get_resource(&content.scene) {
                        draw_background(&bytes, &mut canvas);
                    }

                    draw_dialog(
                        &content.saying,
                        content.character.clone(),
                        &mut canvas,
                        &config
                            .get("font")
                            .expect("'font' is not defined in config.json"),
                    );
                }
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(10));
    }
    Ok(())
}
