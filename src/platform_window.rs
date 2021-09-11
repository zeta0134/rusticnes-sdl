use rusticnes_ui_common::panel::Panel;

use sdl2::pixels::Color;
use sdl2::VideoSubsystem;
use sdl2::render::WindowCanvas;

pub struct PlatformWindow {
  pub panel: Box<dyn Panel>,
  pub canvas: WindowCanvas,
}

use sdl2::pixels::PixelFormatEnum;
fn pixel_enum_as_string(pixel_enum: PixelFormatEnum) -> String {
  // This is very, very stupid
  match pixel_enum {
    PixelFormatEnum::Unknown => "Unknown",
    PixelFormatEnum::Index1LSB => "Index1LSB",
    PixelFormatEnum::Index1MSB => "Index1MSB",
    PixelFormatEnum::Index4LSB => "Index4LSB",
    PixelFormatEnum::Index4MSB => "Index4MSB",
    PixelFormatEnum::Index8 => "Index8",
    PixelFormatEnum::RGB332 => "RGB332",
    PixelFormatEnum::RGB444 => "RGB444",
    PixelFormatEnum::RGB555 => "RGB555",
    PixelFormatEnum::BGR555 => "BGR555",
    PixelFormatEnum::ARGB4444 => "ARGB4444",
    PixelFormatEnum::RGBA4444 => "RGBA4444",
    PixelFormatEnum::ABGR4444 => "ABGR4444",
    PixelFormatEnum::BGRA4444 => "BGRA4444",
    PixelFormatEnum::ARGB1555 => "ARGB1555",
    PixelFormatEnum::RGBA5551 => "RGBA5551",
    PixelFormatEnum::ABGR1555 => "ABGR1555",
    PixelFormatEnum::BGRA5551 => "BGRA5551",
    PixelFormatEnum::RGB565 => "RGB565",
    PixelFormatEnum::BGR565 => "BGR565",
    PixelFormatEnum::RGB24 => "RGB24",
    PixelFormatEnum::BGR24 => "BGR24",
    PixelFormatEnum::RGB888 => "RGB888",
    PixelFormatEnum::RGBX8888 => "RGBX8888",
    PixelFormatEnum::BGR888 => "BGR888",
    PixelFormatEnum::BGRX8888 => "BGRX8888",
    PixelFormatEnum::ARGB8888 => "ARGB8888",
    PixelFormatEnum::RGBA8888 => "RGBA8888",
    PixelFormatEnum::ABGR8888 => "ABGR8888",
    PixelFormatEnum::BGRA8888 => "BGRA8888",
    PixelFormatEnum::ARGB2101010 => "ARGB2101010",
    PixelFormatEnum::YV12 => "YV12",
    PixelFormatEnum::IYUV => "IYUV",
    PixelFormatEnum::YUY2 => "YUY2",
    PixelFormatEnum::UYVY => "UYVY",
    PixelFormatEnum::YVYU => "YVYU",
  }.to_string()
}

impl<'a> PlatformWindow {
  pub fn from_panel(video_subsystem: &'a VideoSubsystem, panel: Box<dyn Panel>) -> PlatformWindow {
    let width = panel.active_canvas().width * panel.scale_factor();
    let height = panel.active_canvas().height * panel.scale_factor();
    let sdl_window = video_subsystem.window(panel.title(), width, height)
      .position(490, 40)
      .opengl()
      .hidden()
      .build()
      .unwrap();
    let mut sdl_canvas = sdl_window.into_canvas().present_vsync().build().unwrap();

    if panel.title() == "RusticNES" {
      let debug_info = sdl_canvas.info();
      println!("Renderer: {}", debug_info.name);
      for format in debug_info.texture_formats {
        println!("Format: {}", pixel_enum_as_string(format));
      }
    }

    sdl_canvas.set_draw_color(Color::RGB(0, 0, 0));
    sdl_canvas.clear();
    sdl_canvas.present();

    return PlatformWindow {
      panel: panel,
      canvas: sdl_canvas,
    }
  }

  pub fn size(&self) -> (u32, u32) {
    let px = self.panel.active_canvas().width * self.panel.scale_factor();
    let py = self.panel.active_canvas().height * self.panel.scale_factor();
    return (px, py);
  }

  pub fn needs_resize(&self) -> bool {
    let (wx, wy) = self.canvas.window().size();
    let (px, py) = self.size();
    return (wx != px) || (wy != py);
  }
}