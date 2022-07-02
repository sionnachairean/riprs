extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::VideoSubsystem;
//use sdl2::event::Event;
//use sdl2::gfx::primitives::DrawRenderer;
//use sdl2::keyboard::Keycode;
//use std::time::Duration;

pub struct CommandHandler {
    sdl_context: Sdl,
    video_system: VideoSubsystem,
    canvas: Option<WindowCanvas>,
    bg_color: Option<Color>,
    line_color: Option<Color>,
    cursor: (u32, u32),
}

impl CommandHandler {
    fn viewport(&mut self, x0: u32, y0: u32, x1: u32, y1: u32) {
        let width = x1 - x0;
        let height = y1 - y0;
        let sdl_window = Some(
            self.video_system
                .window("RIPrs", width, height)
                .position_centered()
                .build()
                .unwrap(),
        );
        self.canvas = Some(sdl_window.unwrap().into_canvas().build().unwrap());
    }
    fn reset_windows(&mut self) {
        self.viewport(0, 0, 640, 350);
        match self.canvas {
            Some(ref mut canv) => {
                canv.set_draw_color(self.bg_color.clone().unwrap());
                canv.clear();
                canv.present();
            }
            None => {}
        }
    }
    fn erase_view(&mut self) {
        match self.canvas {
            Some(ref mut canv) => {
                canv.set_draw_color(self.bg_color.clone().unwrap());
                canv.clear();
                canv.present();
            }
            None => {}
        }
    }
    //fn rip_color(&mut self) {}
}

pub fn init_sdl() -> CommandHandler {
    let sdl_context = sdl2::init().unwrap();
    let video_system = sdl_context.video().unwrap();

    let handler = CommandHandler {
        sdl_context,
        video_system,
        canvas: None,
        bg_color: None,
        line_color: None,
        cursor: (0, 0),
    };
    handler
}
