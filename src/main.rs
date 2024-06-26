mod game_pack;
mod script;
mod window;

use clap::Parser;
use sdl2::event::Event;

use std::collections::HashMap;

#[derive(Parser)]
#[command(version)]
struct Args {
    game_pack: String,
    #[arg(short, long, default_value_t = 800)]
    weight: u32,
    #[arg(short = 'H', long, default_value_t = 600)]
    height: u32,
}

#[derive(Default)]
struct Content {
    character: Option<String>,
    /// Current saying text.
    saying: String,
    /// Current scene file string.
    scene: String,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut pack = game_pack::GamePack::open(&args.game_pack)?;
    let mut script = script::Script::new();
    script.parse(
        &pack
            .get_config("start")
            .expect("'start' does not defined in package.json."),
        &mut pack,
    )?;

    let config: HashMap<String, String> =
        serde_json::from_str(&std::fs::read_to_string("config.json")?)?;

    let content = sdl2::init().unwrap();
    let video = content.video().unwrap();
    let window = video
        .window("31Gal", args.weight, args.height)
        .resizable()
        .build()?;
    let mut events = content.event_pump().unwrap();
    let mut canvas = window.into_canvas().build()?;

    let mut content = Content::default();
    let font_name = config
        .get("font")
        .expect("'font' is not defined in config.json");

    if let Some(title) = pack.get_config("title") {
        canvas.window_mut().set_title(&title)?;
    }

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::MouseButtonUp { .. } => {
                    script.execute_script(&mut pack, &mut content, &mut canvas, font_name)?;
                }
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(_x, _y),
                    ..
                } => {
                    window::redraw(&mut pack, &mut canvas, &mut content, font_name)?;
                }
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(10));
    }
    Ok(())
}
