use asciiengine::character_map::CharacterMap;
use asciiengine::ui::{self, UIElement, MenuSettings};

use asciiengine::interface::GameInterface;
use asciiengine::screen::{TextBufferScreen, TextAlign, TextBufferRect, TextView, TextBufferInterface, Drawable, CharacterColor};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

pub const PIXEL_SCALE: usize = 1;
pub const INITIAL_SIZE: (usize, usize) = (640, 480);

fn main() -> Result<(), String> {
    let mut interface = GameInterface::new(INITIAL_SIZE).unwrap();
    let texture_creator = interface.canvas.texture_creator();

    let mut character_map = CharacterMap::from_file("assets/codepage.bmp", &texture_creator, (9, 16), (8, 8), 32)?;
    let mut text_buffer = TextBufferScreen::new(INITIAL_SIZE.0/9/PIXEL_SCALE, INITIAL_SIZE.1/16/PIXEL_SCALE);
    let mut running = true;
    
    let mut i = 0;

    let mut redraw_all = true;

    let mut inventory = ui::GraphicalMenu::new(vec!["I0", "Item1", "Itsdfgem2", "Item3", "Iteadsfm4", "Item5", "Item6", "Item7"], (8, 0).into(), 
        ui::MenuSettings::new().horizontal(0).align(TextAlign::Center).fix_selection().hide_others().wrapping());

    let mut scrollbox = ui::ScrollBox::new((0, 0, 20, 20).into(), (50, 50));

    for y in 0..50 {
        scrollbox.write_string((0, y).into(), &format!("This is line number {}, it will contain many characters so as to show how scroll menus work", y), asciiengine::screen::CharacterColor::BrightWhite).unwrap();
    }

    while running {
        for event in interface.event_pump.poll_iter() {
            match event {
                Event::MouseMotion { .. } => {},
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                    let (cw, ch) = character_map.character_size();
                    text_buffer.resize_buffer(width as usize / PIXEL_SCALE / cw, height as usize / PIXEL_SCALE / ch);
                    redraw_all = true;
                }

                Event::KeyDown { keycode: Some(Keycode::A), .. } => { inventory.prev() }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { inventory.next() }
                // Event::KeyDown { keycode: Some(Keycode::A), .. } => { i -= 1; redraw_all = true; }
                // Event::KeyDown { keycode: Some(Keycode::D), .. } => { i += 1; redraw_all = true; }

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => { scrollbox.scroll_horizontal(-1) }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => { scrollbox.scroll_horizontal(1) }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => { scrollbox.scroll_vertical(-1) }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => { scrollbox.scroll_vertical(1) }
                _ => {}
            }
        }

        let width = text_buffer.width();
        let height = text_buffer.height();

        let pane0: TextBufferRect = (0, 0, (width as isize / 2 + i) as usize, height * 3 / 4).into();
        let pane1: TextBufferRect = (0, height as isize * 3 / 4, (width as isize / 2 + i) as usize, (height + 3) / 4).into();
        let pane2: TextBufferRect = ((width as isize) / 2 + i, 0, ((width + 1) as isize / 2 - i) as usize, height /2 ).into();
        let pane3: TextBufferRect = ((width as isize) / 2 + i, height as isize / 2, ((width + 1) as isize / 2 - i) as usize, (height + 1) / 2).into();

        if redraw_all {
            text_buffer.clear_rect(text_buffer.screen_rect());
            ui::draw_box(pane0, &mut text_buffer, Some(("Pane0 A Stupidly Long Name", TextAlign::Right)));
            ui::draw_box(pane1, &mut text_buffer, Some(("Pane1", TextAlign::Center)));
            ui::draw_box(pane2, &mut text_buffer, Some(("Pane2", TextAlign::Center)));
            ui::draw_box(pane3, &mut text_buffer, Some(("Pane3", TextAlign::Center)));
        }

        if redraw_all || inventory.take_dirty() || scrollbox.take_dirty() {
            let mut view = TextView::new(pane2.interior(), &mut text_buffer);
            scrollbox.rect.width = pane2.interior().width;
            scrollbox.rect.height = pane2.interior().height;
            scrollbox.ui_draw(&mut view);

            
            let mut view = TextView::new(pane3.interior(), &mut text_buffer);
            inventory.clear_last(&mut view);
            inventory.ui_draw(&mut view);
            redraw_all = false;
        }
        
        text_buffer.write_to_canvas(&mut interface.canvas, &mut character_map, PIXEL_SCALE)?;
    }

    Ok(())
}