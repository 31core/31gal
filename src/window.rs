use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{game_pack::GamePack, Content};

pub fn redraw(
    pack: &mut GamePack,
    canvas: &mut Canvas<Window>,
    content: &mut Content,
    font_name: &str,
) {
    canvas.clear();
    if let Ok(bytes) = pack.get_resource(&content.scene) {
        draw_background(&bytes, canvas);
    }

    draw_dialog(
        &content.saying,
        content.character.clone(),
        canvas,
        font_name,
    );

    canvas.present();
}

pub fn draw_dialog(
    saying: &str,
    character: Option<String>,
    canvas: &mut Canvas<Window>,
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
}

pub fn draw_background(image_bytes: &[u8], canvas: &mut Canvas<Window>) {
    let text_creator = canvas.texture_creator();
    let texture = text_creator.load_texture_bytes(image_bytes).unwrap();
    canvas
        .copy(
            &texture,
            None,
            Rect::new(0, 0, canvas.window().size().0, canvas.window().size().1),
        )
        .unwrap();
}
