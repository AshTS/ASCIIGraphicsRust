use crate::screen::{VGAChar, TextBufferRect, CharacterColor};

use super::UIElement;

pub struct TextBox {
    rect: TextBufferRect,
    internal_buffer: Vec<(VGAChar, CharacterColor)>,
    dirty: bool,
    cursor_x: usize,
    draw_color: CharacterColor,
}

impl TextBox {
    pub fn new(rect: TextBufferRect) -> Self {
        Self {
            rect,
            internal_buffer: vec![(VGAChar(b' '), CharacterColor::Gray); rect.area()],
            dirty: true,
            cursor_x: 0,
            draw_color: CharacterColor::White
        }
    }

    fn line(&self, i: usize) -> &[(VGAChar, CharacterColor)] {
        &self.internal_buffer[i * self.rect.width..(i + 1) * self.rect.width]
    }

    fn scroll(&mut self) {
        for x in 0..self.rect.width {
            for y in 0..self.rect.height - 1 {
                self.internal_buffer[y * self.rect.width + x] = self.internal_buffer[(y + 1) * self.rect.width + x];
            }
            self.internal_buffer[(self.rect.height - 1) * self.rect.width + x] = (VGAChar(b' '), CharacterColor::Gray);
        }
    }

    pub fn place_char(&mut self, c: char) {
        if c == '\n' || self.cursor_x >= self.rect.width {
            self.cursor_x = 0;
            self.scroll();
        }

        if c != '\n' {
            let vga: VGAChar = c.try_into().unwrap();

            self.internal_buffer[(self.rect.height - 1) * self.rect.width + self.cursor_x] = (vga, self.draw_color);
            self.cursor_x += 1;
        }
        self.dirty = true;
    }

    pub fn place_string(&mut self, s: &str) {
        for c in s.chars() {
            self.place_char(c);
        }
    }

    pub fn clear_color(&mut self) {
        self.draw_color = CharacterColor::White;
    }

    pub fn set_color(&mut self, c: CharacterColor) {
        self.draw_color = c;
    }

    pub fn update_rect(&mut self, rect: TextBufferRect) {
        let mut new_internal_buffer = vec![(VGAChar(b' '), CharacterColor::Gray); rect.area()];

        let line_width = rect.width;
        let last_line = rect.height * line_width;

        for y in 0..self.rect.height {
            if last_line >= (self.rect.height - y) * line_width {
                for (i, c) in self.line(y).iter().enumerate() {
                    if i < rect.width {
                        new_internal_buffer[last_line - (self.rect.height - y) * line_width + i] = *c;
                    }
                }
            }
        }

        self.rect = rect;
        self.internal_buffer = new_internal_buffer;
    }
}

impl UIElement for TextBox {
    fn take_dirty(&mut self) -> bool {
        let dirty = self.dirty;
        self.dirty = false;
        dirty
    }

    fn ui_draw(&mut self, screen: &mut impl crate::screen::TextBufferInterface) {
        for y in 0..self.rect.height {
            screen.write_data((self.rect.x, self.rect.y + y as isize).into(), self.line(y));
        }
    }

    fn clear_last(&self, screen: &mut impl crate::screen::TextBufferInterface) {
        screen.clear_rect(self.rect)
    }
}

impl std::fmt::Write for TextBox {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.place_string(s);
        Ok(())
    }
}