use crate::screen::{TextBufferRect, VGAChar, CharacterColor, TextBufferInterface};

use super::UIElement;

pub struct ScrollBox {
    pub rect: TextBufferRect,
    width: usize,
    height: usize,
    internal_buffer: Vec<(VGAChar, CharacterColor)>,
    dirty_regions: Vec<TextBufferRect>,

    scroll: (isize, isize),
    bounded: bool
}

impl ScrollBox {
    pub fn new(rect: TextBufferRect, size: (usize, usize)) -> Self {
        Self {
            rect,
            width: size.0,
            height: size.1,
            internal_buffer: vec![(VGAChar(b' '), CharacterColor::Gray); size.0 * size.1],
            dirty_regions: vec![TextBufferRect::new(0, 0, size.0, size.1)],
            scroll: (0, 0),
            bounded: false
        }
    }

    pub fn scroll_horizontal(&mut self, amt: isize) {
        let backup = self.scroll;
        if !self.bounded {
            self.scroll.0 += amt;
        }
        else {
            self.scroll.0 = (self.scroll.0 + amt).max(0).min(self.width as isize - self.rect.width as isize);
        }

        if self.scroll != backup {  
            self.dirty_regions = vec![TextBufferRect::new(0, 0, self.width, self.height)];
        }
    }

    pub fn scroll_vertical(&mut self, amt: isize) {
        let backup = self.scroll;
        if !self.bounded {
            self.scroll.1 += amt;
        }
        else {
            self.scroll.1 = (self.scroll.1 + amt).max(0).min(self.height as isize - self.rect.height as isize);
        }

        if self.scroll != backup {  
            self.dirty_regions = vec![TextBufferRect::new(0, 0, self.width, self.height)];
        }
    }
}

impl UIElement for ScrollBox {
    fn take_dirty(&mut self) -> bool {
        !self.dirty_regions.is_empty()
    }

    fn ui_draw(&mut self, screen: &mut impl crate::screen::TextBufferInterface) {
        self.dirty_regions.clear();

        screen.clear_rect(self.rect);

        self.scroll_horizontal(0);
        self.scroll_vertical(0);

        for (y_index, y) in (self.rect.y..self.rect.bottom()).enumerate() {
            let y_index = y_index as isize + self.scroll.1;
            if let Some(i) = self.index_of((0, y_index as isize).into())
            {
                if self.scroll.0 >= 0 {
                    screen.write_data((self.rect.x + self.scroll.0, y).into(), &self.internal_buffer[i..(i+self.rect.width-self.scroll.0 as usize).min(i + self.width)]);
                }
                else {
                    screen.write_data((self.rect.x, y).into(), &self.internal_buffer[(i + (-self.scroll.0) as usize)..(i+self.rect.width+(-self.scroll.0) as usize).min(i + self.width)]);
                }
            }
        }
    }

    fn clear_last(&self, _screen: &mut impl crate::screen::TextBufferInterface) {
        // Nothing needs to be done here since refresh already draws over the entire screen
    }
}

impl TextBufferInterface for ScrollBox {
    fn add_dirty_rect(&mut self, rect: TextBufferRect) {
        self.dirty_regions.push(rect);
    }

    fn index_of(&self, pos: crate::screen::TextBufferPos) -> Option<usize> {
        if 0 <= pos.x && pos.x < self.width as isize && 0 <= pos.y && pos.y < self.height as isize {
            Some(pos.x as usize + pos.y as usize * self.width)
        }
        else {
            None
        }
    }

    fn inner_data_buffer(&self) -> &[(VGAChar, CharacterColor)] {
        &self.internal_buffer
    }

    fn inner_data_buffer_mut(&mut self) -> &mut [(VGAChar, CharacterColor)] {
        &mut self.internal_buffer
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}