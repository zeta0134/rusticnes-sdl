extern crate image;
extern crate nfd;
extern crate sdl2;

extern crate rusticnes_core;
extern crate rusticnes_ui_common;

mod audio_window;
mod debugger_window;
mod game_window;
mod memory_window;
mod piano_roll_window;

use sdl2::audio::AudioSpecDesired;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::TextureAccess;
use std::env;
use std::fs::remove_file;

use rusticnes_ui_common::application::RuntimeState as RusticNesRuntimeState;
use rusticnes_ui_common::events;
use rusticnes_ui_common::panel::Panel;
use rusticnes_ui_common::test_window::TestWindow;
use rusticnes_ui_common::vram_window::VramWindow;

pub fn main() {
  let mut runtime_state = RusticNesRuntimeState::new();

  let sdl_context = sdl2::init().unwrap();
  let audio_subsystem = sdl_context.audio().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let mut event_pump = sdl_context.event_pump().unwrap();

  // Setup Audio output format and sample rate
  let desired_spec = AudioSpecDesired {
    freq: Some(44100),
    channels: Some(1),
    samples: Some(2048)
  };

  // Grab the active audio device and begin playback immediately. Until we fill the buffer, this will "play" silence:
  let device = audio_subsystem.open_queue::<i16, _>(None, &desired_spec).unwrap();
  device.clear();
  device.resume();

  // Setup the main window for rendering
  let sdl_game_window = video_subsystem.window("RusticNES NEW", (256 - 16) * 2, (240 - 16) * 2)
    .position(5, 40)
    .opengl()
    .build()
    .unwrap();
  let mut game_canvas = sdl_game_window.into_canvas().present_vsync().build().unwrap();
  game_canvas.set_draw_color(Color::RGB(0, 0, 0));
  game_canvas.clear();
  game_canvas.present();
  let game_screen_texture_creator = game_canvas.texture_creator();
  let mut game_screen_texture = game_screen_texture_creator.create_texture(PixelFormatEnum::RGBA8888, TextureAccess::Streaming, 256, 240).unwrap();
  let mut game_window = game_window::GameWindow::new();

  // Setup various debug windows
  let sdl_audio_window = video_subsystem.window("Audio Visualizer", 512, 384)
    .position(490, 40)
    .hidden()
    .opengl()
    .build()
    .unwrap();

  let mut audio_canvas = sdl_audio_window.into_canvas().build().unwrap();
  audio_canvas.set_draw_color(Color::RGB(0, 0, 0));
  audio_canvas.clear();
  audio_canvas.present();
  let audio_screen_texture_creator = audio_canvas.texture_creator();
  let mut audio_screen_texture = audio_screen_texture_creator.create_texture(PixelFormatEnum::ABGR8888, TextureAccess::Streaming, 256, 192).unwrap();
  let mut audio_window = audio_window::AudioWindow::new();

  let mut vram_window = VramWindow::new();
  let sdl_vram_window = video_subsystem.window(vram_window.title(), vram_window.active_canvas().width, vram_window.active_canvas().height)
    .position(490, 40)
    .opengl()
    .build()
    .unwrap();
  let mut vram_canvas = sdl_vram_window.into_canvas().build().unwrap();
  vram_canvas.set_draw_color(Color::RGB(0, 0, 0));
  vram_canvas.clear();
  vram_canvas.present();
  let vram_texture_creator = vram_canvas.texture_creator();
  let mut vram_screen_texture = vram_texture_creator.create_texture(PixelFormatEnum::ABGR8888, TextureAccess::Streaming, vram_window.active_canvas().width, vram_window.active_canvas().height).unwrap();

  let sdl_memory_window = video_subsystem.window("Memory Viewer", 360 * 2, 220 * 2)
    .position(490, 40)
    .hidden()
    .opengl()
    .build()
    .unwrap();

  let mut memory_canvas = sdl_memory_window.into_canvas().build().unwrap();
  memory_canvas.set_draw_color(Color::RGB(0, 0, 0));
  memory_canvas.clear();
  memory_canvas.present();
  let mut memory_window = memory_window::MemoryWindow::new();
  let memory_texture_creator = memory_canvas.texture_creator();
  let mut memory_screen_texture = memory_texture_creator.create_texture(PixelFormatEnum::ABGR8888, TextureAccess::Streaming, memory_window.buffer.width, memory_window.buffer.height).unwrap();

  let sdl_debugger_window = video_subsystem.window("Debugger", 512, 600)
    .position(490, 40)
    .hidden()
    .opengl()
    .build()
    .unwrap();

  let mut debugger_canvas = sdl_debugger_window.into_canvas().build().unwrap();
  debugger_canvas.set_draw_color(Color::RGB(0, 0, 0));
  debugger_canvas.clear();
  debugger_canvas.present();
  let mut debugger_window = debugger_window::DebuggerWindow::new();
  let debugger_texture_creator = debugger_canvas.texture_creator();
  let mut debugger_screen_texture = debugger_texture_creator.create_texture(PixelFormatEnum::ABGR8888, TextureAccess::Streaming, debugger_window.buffer.width, debugger_window.buffer.height).unwrap();

  let mut piano_roll_window = piano_roll_window::PianoRollWindow::new();
  let sdl_piano_roll_window = video_subsystem.window("Piano Roll", piano_roll_window.buffer.width, piano_roll_window.buffer.height)
    .position(490, 40)
    .hidden()
    .opengl()
    .build()
    .unwrap();
  let mut piano_roll_canvas = sdl_piano_roll_window.into_canvas().build().unwrap();
  piano_roll_canvas.set_draw_color(Color::RGB(0, 0, 0));
  piano_roll_canvas.clear();
  piano_roll_canvas.present();
  let piano_roll_texture_creator = piano_roll_canvas.texture_creator();
  let mut piano_roll_screen_texture = piano_roll_texture_creator.create_texture(PixelFormatEnum::ABGR8888, TextureAccess::Streaming, piano_roll_window.buffer.width, piano_roll_window.buffer.height).unwrap();

  let mut test_window = TestWindow::new();
  let sdl_test_window = video_subsystem.window(test_window.title(), test_window.active_canvas().width * 2, test_window.active_canvas().height * 2)
    .position(490, 40)
    .opengl()
    .build()
    .unwrap();
  let mut test_canvas = sdl_test_window.into_canvas().build().unwrap();
  test_canvas.set_draw_color(Color::RGB(0, 0, 0));
  test_canvas.clear();
  test_canvas.present();
  let test_texture_creator = test_canvas.texture_creator();
  let mut test_screen_texture = test_texture_creator.create_texture(PixelFormatEnum::ABGR8888, TextureAccess::Streaming, test_window.active_canvas().width, test_window.active_canvas().height).unwrap();

  let mut ctrl_mod = false;
  let mut trigger_resize = false;
  let mut dump_audio = false;

  let args: Vec<_> = env::args().collect();
  if args.len() > 1 {
      game_window.open_file(&mut runtime_state.nes, &args[1]);
  }

  'running: loop {
    if !game_window.shown {
      break 'running
    }

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} => {
          break 'running
        },
        _ => {
          if sdl_context.keyboard().focused_window_id().is_some() {
            let focused_window_id = sdl_context.keyboard().focused_window_id().unwrap();
            let application_focused = 
              audio_canvas.window().id() == focused_window_id ||
              debugger_canvas.window().id() == focused_window_id ||
              game_canvas.window().id() == focused_window_id ||
              memory_canvas.window().id() == focused_window_id ||
              piano_roll_canvas.window().id() == focused_window_id ||
              vram_canvas.window().id() == focused_window_id;

            if application_focused {
              match event {
                Event::KeyDown { keycode: Some(key), .. } => {
                  // Pass keydown events into sub-windows
                  game_window.handle_key_down(&mut runtime_state.nes, key);
                  // Handle global keydown events
                  if key == Keycode::LCtrl || key == Keycode::RCtrl {
                    ctrl_mod = true;
                  }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                  // Pass keyup events into sub-windows
                  audio_window.handle_key_up(&mut runtime_state.nes, key);
                  game_window.handle_key_up(&mut runtime_state.nes, key);
                  memory_window.handle_key_up(&mut runtime_state.nes, key);
                  // Handle global keydown events
                  if key == Keycode::LCtrl || key == Keycode::RCtrl {
                    ctrl_mod = false;
                  }
                  if ctrl_mod {
                    match key {
                      Keycode::Q => { break 'running },
                      Keycode::O => { game_window.open_file_dialog(&mut runtime_state.nes); ctrl_mod = false; },
                      _ => ()
                    }
                  } else {
                    // Previous implementation handled debug window showing / hiding here
                    match key {
                      /*Keycode::F1 => {
                        if !vram_window.shown {
                          vram_window.shown = true;
                          vram_canvas.window_mut().show();
                        } else {
                          vram_window.shown = false;
                          vram_canvas.window_mut().hide();
                        }
                      },*/
                      Keycode::F2 => {
                        if !audio_window.shown {
                          audio_window.shown = true;
                          audio_canvas.window_mut().show();
                        } else {
                          audio_window.shown = false;
                          audio_canvas.window_mut().hide();
                        }
                      },
                      Keycode::F3 => {
                        if !memory_window.shown {
                          memory_window.shown = true;
                          memory_canvas.window_mut().show();
                        } else {
                          memory_window.shown = false;
                          memory_canvas.window_mut().hide();
                        }
                      },
                      Keycode::F4 => {
                        if !debugger_window.shown {
                          debugger_window.shown = true;
                          debugger_canvas.window_mut().show();
                        } else {
                          debugger_window.shown = false;
                          debugger_canvas.window_mut().hide();
                        }
                      },
                      Keycode::F5 => {
                        if !piano_roll_window.shown {
                          piano_roll_window.shown = true;
                          piano_roll_canvas.window_mut().show();
                        } else {
                          piano_roll_window.shown = false;
                          piano_roll_canvas.window_mut().hide();
                        }
                      },
                      Keycode::Equals | Keycode::KpPlus | Keycode::Plus => {
                        if game_window.scale < 8 {
                          game_window.scale += 1;
                          trigger_resize = true;
                        }
                      },
                      Keycode::KpMinus | Keycode::Minus => {
                        if game_window.scale > 1 {
                          game_window.scale -= 1;
                          trigger_resize = true;
                        }
                      },
                      Keycode::KpMultiply=> {
                        game_window.display_overscan = !game_window.display_overscan;
                        trigger_resize = true;
                      },
                      Keycode::A => {
                        dump_audio = !dump_audio;
                        if dump_audio {
                          let _ = remove_file("audiodump.raw");
                          println!("Beginning audio dump...");
                        } else {
                          println!("Audio dump stopped.");
                        }
                      },
                      
                      _ => ()
                    }
                  }
                },
                Event::MouseButtonDown{ window_id: id, mouse_btn: MouseButton::Left, x: omx, y: omy, .. } if id == memory_canvas.window().id() => {
                    memory_window.handle_click(&mut runtime_state.nes, omx / 2, omy / 2);
                },
                Event::Window { window_id: id, win_event: WindowEvent::Close, .. } => {
                  if id == game_canvas.window().id() {
                    game_window.shown = false;
                    game_canvas.window_mut().hide();
                  }
                  if id == audio_canvas.window().id() {
                    audio_window.shown = false;
                    audio_canvas.window_mut().hide();
                  }
                  if id == debugger_canvas.window().id() {
                    debugger_window.shown = false;
                    debugger_canvas.window_mut().hide();
                  }
                  if id == memory_canvas.window().id() {
                    memory_window.shown = false;
                    memory_canvas.window_mut().hide();
                  }
                  /*
                  if id == vram_canvas.window().id() {
                    vram_window.shown = false;
                    vram_canvas.window_mut().hide();
                  }*/
                },
                _ => ()
              }
            }
          }
        }
      }
    }

    // Update windows
    if game_window.shown {
      game_window.update(&mut runtime_state.nes);

      if trigger_resize {
        trigger_resize = false;
        if game_window.display_overscan {
          let _ = game_canvas.window_mut().set_size(game_window.scale * 256, game_window.scale * 240);
        } else {
          let _ = game_canvas.window_mut().set_size(game_window.scale * (256 - 16), game_window.scale * (240 - 16));
        }
      }
    } else {
      // The main game window was closed! Exit the program.
      break 'running
    }
    if audio_window.shown {
      audio_window.update(&mut runtime_state.nes);
    }
    if debugger_window.shown {
      debugger_window.update(&mut runtime_state.nes);
    }
    if memory_window.shown {
      memory_window.update(&mut runtime_state.nes);
    }
    if piano_roll_window.shown {
      piano_roll_window.update(&mut runtime_state.nes);
    }
    //if vram_window.shown {
      //vram_window.update(&mut runtime_state.nes);
      vram_window.handle_event(&runtime_state, events::Event::Update);
    //}

    test_window.handle_event(&runtime_state, events::Event::Update);

    // Play Audio
    if runtime_state.nes.apu.buffer_full {
      let mut buffer = [0i16; 4096];
      for i in 0 .. 4096 {
        buffer[i] = runtime_state.nes.apu.output_buffer[i] as i16;
      }
      device.queue(&buffer);
      runtime_state.nes.apu.buffer_full = false;
      if dump_audio {
        runtime_state.nes.apu.dump_sample_buffer();
      }
    }

    // Draw all windows
    if audio_window.shown {
      audio_canvas.set_draw_color(Color::RGB(255, 255, 255));
      let _ = audio_screen_texture.update(None, &audio_window.buffer.buffer, 256 * 4);
      let _ = audio_canvas.copy(&audio_screen_texture, None, None);
      audio_canvas.present();
    }

    if debugger_window.shown {
      debugger_canvas.set_draw_color(Color::RGB(255, 255, 255));
      let _ = debugger_screen_texture.update(None, &debugger_window.buffer.buffer, (debugger_window.buffer.width * 4) as usize);
      let _ = debugger_canvas.copy(&debugger_screen_texture, None, None);
      debugger_canvas.present();
    }

    if piano_roll_window.shown {
      piano_roll_canvas.set_draw_color(Color::RGB(255, 255, 255));
      let _ = piano_roll_screen_texture.update(None, &piano_roll_window.buffer.buffer, (piano_roll_window.buffer.width * 4) as usize);
      let _ = piano_roll_canvas.copy(&piano_roll_screen_texture, None, None);
      piano_roll_canvas.present();
    }

    if memory_window.shown {
      memory_canvas.set_draw_color(Color::RGB(255, 255, 255));
      let _ = memory_screen_texture.update(None, &memory_window.buffer.buffer, (memory_window.buffer.width * 4) as usize);
      let _ = memory_canvas.copy(&memory_screen_texture, None, None);
      memory_canvas.present();
    }

    //if vram_window.shown {
      vram_window.handle_event(&runtime_state, events::Event::RequestFrame);
      vram_canvas.set_draw_color(Color::RGB(255, 255, 255));
      let _ = vram_screen_texture.update(None, &vram_window.active_canvas().buffer, (vram_window.active_canvas().width * 4) as usize);
      let _ = vram_canvas.copy(&vram_screen_texture, None, None);
      vram_canvas.present();
    //}

    test_window.handle_event(&runtime_state, events::Event::RequestFrame);
    test_canvas.set_draw_color(Color::RGB(255, 255, 255));
    let _ = test_screen_texture.update(None, &test_window.active_canvas().buffer, (test_window.active_canvas().width * 4) as usize);
    let _ = test_canvas.copy(&test_screen_texture, None, None);
    test_canvas.present();

    game_canvas.set_draw_color(Color::RGB(255, 255, 255));
    let _ = game_screen_texture.update(None, &game_window.screen_buffer, 256 * 4);
    if game_window.display_overscan {
      let _ = game_canvas.copy(&game_screen_texture, None, None);
    } else {
      let borderless_rectangle = Rect::new(8, 8, 256 - 16, 240 - 16);
      let _ = game_canvas.copy(&game_screen_texture, borderless_rectangle, None);
    }
    game_canvas.present();
  }
}

