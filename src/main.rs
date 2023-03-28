use std::{fs::File, time::Instant, u32};

use game_loop::game_loop;
use log::{error, info};
use pixels::{Error, Pixels, SurfaceTexture};
use renderer::Renderer;
use ui::UI;
use winit::{
    dpi::LogicalSize, event::VirtualKeyCode, event_loop::EventLoop, window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

pub mod renderer;
pub mod ui;

const SCALE: usize = 4;
const WIDTH: usize = 1280 / SCALE;
const HEIGHT: usize = 720 / SCALE;

pub struct Game {
    renderer: Renderer,
    ui: UI,
    input: WinitInputHelper,
    pixels: Pixels,
}

impl Game {
    pub fn new(renderer: Renderer, ui: UI, input: WinitInputHelper, pixels: Pixels) -> Self {
        Self {
            renderer,
            ui,
            input,
            pixels,
        }
    }

    pub fn update(&mut self) {
        if self.input.key_held(VirtualKeyCode::W) {
            self.renderer.add_position(0.08);
        }
        if self.input.key_held(VirtualKeyCode::S) {
            self.renderer.add_position(-0.08);
        }
        if self.input.key_held(VirtualKeyCode::A) {
            self.renderer.add_rotation(0.05);
        }
        if self.input.key_held(VirtualKeyCode::D) {
            self.renderer.add_rotation(-0.05);
        }
    }
}

fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    let event_loop = EventLoop::new();
    let input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scale_size =
            LogicalSize::new(WIDTH as f64 * SCALE as f64, HEIGHT as f64 * SCALE as f64);
        WindowBuilder::new()
            .with_title("wolfenstien")
            .with_inner_size(scale_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("assets/map.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];

    let mut map = Vec::new();
    for chunk in bytes.chunks_exact(4) {
        map.push(u32::from_be_bytes(chunk.try_into().unwrap()));
    }
    let renderer = Renderer::new(map, info.width as usize, info.height as usize);
    let ui = UI;
    let game = Game::new(renderer, ui, input, pixels);

    game_loop(
        event_loop,
        window,
        game,
        60,
        0.1,
        move |g| {
            g.game.update();
        },
        move |g| {
            let start = Instant::now();

            // render current image onto framebuffer
            g.game.renderer.render(g.game.pixels.get_frame_mut());

            // render ui
            //g.game.ui.render(g.game.pixels.get_frame_mut());

            // display framebuffer
            if let Err(err) = g.game.pixels.render() {
                error!("pixels.render() failed: {err}");
                g.exit();
            }

            let end = Instant::now();
            let frametime = end - start;
            let fps = 1000 / frametime.as_millis();
            info!("FPS/MSPF: {fps}/{frametime:?}");
        },
        |g, event| {
            if g.game.input.update(event) {
                // Close events
                if g.game.input.key_pressed(VirtualKeyCode::Escape)
                    || g.game.input.close_requested()
                    || g.game.input.destroyed()
                {
                    g.exit();
                    return;
                }

                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    if let Err(err) = g.game.pixels.resize_surface(size.width, size.height) {
                        error!("pixels.resize_surface() failed: {err}");
                        g.exit();
                    }
                }
            }
        },
    );
}
