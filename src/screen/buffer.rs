use super::*;

use sdl2::render::Canvas;

use crate::character_map::CharacterMap;

/// Trait to allow generic usage of text views and the buffer display
pub trait TextBufferInterface {
    /// Get the rect for the entire screen
    fn screen_rect(&self) -> TextBufferRect {
        TextBufferRect { x: 0, y: 0, width: self.width(), height: self.height() }
    }

    /// Insert a new dirty text buffer rect
    fn add_dirty_rect(&mut self, rect: TextBufferRect);

    /// Get the index of a character at the given position
    fn index_of(&self, pos: TextBufferPos) -> Option<usize>;

    /// Get access to the inner data buffer (if in a text view, this is the one at the lowest level)
    fn inner_data_buffer(&self) -> &[(VGAChar, CharacterColor)];

    /// Get access to the inner data buffer (if in a text view, this is the one at the lowest level)
    fn inner_data_buffer_mut(&mut self) -> &mut [(VGAChar, CharacterColor)];

    /// Get a mutable reference to the character at the given position
    fn inner_mut_char(&mut self, pos: TextBufferPos) -> Option<&mut (VGAChar, CharacterColor)> {
        let i = self.index_of(pos)?;
        Some(&mut self.inner_data_buffer_mut()[i])
    }

    /// Get a mutable reference to the character at the given position, try to avoid using this and prefer the instructions which write larger sections of the text buffer at once
    fn mut_char(&mut self, pos: TextBufferPos) -> Option<&mut (VGAChar, CharacterColor)> {
        self.add_dirty_rect(TextBufferRect::new(pos.x, pos.y, 1, 1));
        let i = self.index_of(pos)?;
        Some(&mut self.inner_data_buffer_mut()[i])
    }

    /// Get the character at the given position
    fn char_ref(&self, pos: TextBufferPos) -> Option<&(VGAChar, CharacterColor)> {
        let i = self.index_of(pos)?;
        Some(&self.inner_data_buffer()[i])
    }

    /// Clear a rectangle
    fn clear_rect(&mut self, rect: TextBufferRect) {
        for x in rect.x..rect.right() {
            for y in rect.y..rect.bottom() {
                if let Some(c) = self.inner_mut_char(TextBufferPos{x, y}) {
                    *c = (VGAChar(b' '), CharacterColor::Gray);
                }
            }
        }

        self.add_dirty_rect(rect);
    }

    /// Write text to the display at a specific location, takes in a slice of VGAChar's and a color for the text
    fn write_text(&mut self, pos: TextBufferPos, text: &[VGAChar], color: CharacterColor) -> TextBufferRect {
        for (i, c) in text.iter().enumerate() {
            if let Some(cref) = self.inner_mut_char(TextBufferPos{x: pos.x + i as isize, y: pos.y}) {
                *cref = (*c, color);
            }
        }

        let rect = TextBufferRect::new(pos.x, pos.y, text.len(), 1);
        self.add_dirty_rect(rect);
        rect
    }

    /// Write text to the display at a specific location respecting text alignment, takes in a slice of VGAChar's and a color for the text
    fn write_text_align(&mut self, pos: TextBufferPos, text: &[VGAChar], color: CharacterColor, align: TextAlign) -> TextBufferRect {
        self.write_text(align.align_text(pos, text.len()), text, color)
    }

    /// Write a string to the display at a specific location. Takes in an &str instead of a slice of VGAChar's. This function attempts to perform the conversion to VGAChar's, returning an error if the conversion cannot be performed
    fn write_string(&mut self, pos: TextBufferPos, text: &str, color: CharacterColor) -> Result<TextBufferRect, char> {
        for (i, c) in text.chars().enumerate() {
            if let Some(cref) = self.inner_mut_char(TextBufferPos{x: pos.x + i as isize, y: pos.y}) {
                *cref = (c.try_into()?, color);
            }
        }

        let rect = TextBufferRect::new(pos.x, pos.y, text.len(), 1);
        self.add_dirty_rect(rect);

        Ok(rect)
    }

    /// Write a string to the display at a specific location respecting text alignment. Takes in an &str instead of a slice of VGAChar's. This function attempts to perform the conversion to VGAChar's, returning an error if the conversion cannot be performed
    fn write_string_align(&mut self, pos: TextBufferPos, text: &str, color: CharacterColor, align: TextAlign) -> Result<TextBufferRect, char> {
        self.write_string(align.align_text(pos, text.len()), text, color)
    }

    /// Write a line of pre colored text to the display at the given position, takes in a slice of (VGAChar, CharacterColor) pairs. This is preferable for staticly allocated text
    fn write_data(&mut self, pos: TextBufferPos, text: &[(VGAChar, CharacterColor)]) -> TextBufferRect {
        for (i, c) in text.iter().enumerate() {
            if let Some(cref) = self.inner_mut_char(TextBufferPos{x: pos.x + i as isize, y: pos.y}) {
                *cref = *c;
            }
        }

        let rect = TextBufferRect::new(pos.x, pos.y, text.len(), 1);
        self.add_dirty_rect(rect);

        rect
    }

    /// Write a line of pre colored text to the display at the given position respecting text alignment, takes in a slice of (VGAChar, CharacterColor) pairs. This is preferable for staticly allocated text
    fn write_data_align(&mut self, pos: TextBufferPos, text: &[(VGAChar, CharacterColor)], align: TextAlign) -> TextBufferRect {
        self.write_data(align.align_text(pos, text.len()), text)
    }

    /// Get the width of the text buffer
    fn width(&self) -> usize;

    /// Get the height of the text buffer
    fn height(&self) -> usize;
}

/// Text buffer representing the entire screen to be displayed
#[derive(Clone)]
pub struct TextBufferScreen {
    width: usize,
    height: usize, 
    data: Vec<(VGAChar, CharacterColor)>,
    dirty_regions: Vec<TextBufferRect>,
    clear_optimization_heuristic: bool,
}

impl TextBufferScreen {
    /// Construct a new, empty text buffer
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![(VGAChar(b' '), CharacterColor::Gray); width * height],
            dirty_regions: vec![TextBufferRect::new(0, 0, width, height)],
            clear_optimization_heuristic: true
        }
    }

    /// Get the rect for the entire screen
    pub const fn screen_rect(&self) -> TextBufferRect {
        TextBufferRect { x: 0, y: 0, width: self.width(), height: self.height() }
    }

    /// Insert a new dirty text buffer rect
    pub fn add_dirty_rect(&mut self, rect: TextBufferRect) {
        for old_rect in self.dirty_regions.iter_mut() {
            if old_rect.contains(&rect) {
                *old_rect = rect;
                return;
            }
            else if rect.contains(old_rect) {
                return;
            }
        }

        self.dirty_regions.push(rect);
    }

    /// Get the index of a character at the given position
    const fn index_of(&self, pos: TextBufferPos) -> Option<usize> {
        if 0 <= pos.x && pos.x < self.width as isize && 0 <= pos.y && pos.y < self.height as isize {
            Some(pos.x as usize + pos.y as usize * self.width)
        }
        else {
            None
        }
    }

    /// Get a mutable reference to the character at the given position
    fn inner_mut_char(&mut self, pos: TextBufferPos) -> Option<&mut (VGAChar, CharacterColor)> {
        let i = self.index_of(pos)?;
        Some(&mut self.data[i])
    }

    /// Get a mutable reference to the character at the given position, try to avoid using this and prefer the instructions which write larger sections of the text buffer at once
    pub fn mut_char(&mut self, pos: TextBufferPos) -> Option<&mut (VGAChar, CharacterColor)> {
        self.add_dirty_rect(TextBufferRect::new(pos.x, pos.y, 1, 1));
        let i = self.index_of(pos)?;
        Some(&mut self.data[i])
    }

    /// Get the character at the given position
    pub fn char_ref(&self, pos: TextBufferPos) -> Option<&(VGAChar, CharacterColor)> {
        let i = self.index_of(pos)?;
        Some(&self.data[i])
    }

    /// Clear a rectangle
    pub fn clear_rect(&mut self, rect: TextBufferRect) {
        for x in rect.x..rect.right() {
            for y in rect.y..rect.bottom() {
                if let Some(c) = self.inner_mut_char(TextBufferPos{x, y}) {
                    *c = (VGAChar(b' '), CharacterColor::Gray);
                }
            }
        }

        if self.screen_rect().area() <= rect.area() * 4 {
            self.clear_optimization_heuristic = true;
        }

        self.add_dirty_rect(rect);
    }

    /// Write text to the display at a specific location, takes in a slice of VGAChar's and a color for the text
    pub fn write_text(&mut self, pos: TextBufferPos, text: &[VGAChar], color: CharacterColor) -> TextBufferRect {
        for (i, c) in text.iter().enumerate() {
            if let Some(cref) = self.inner_mut_char(TextBufferPos{x: pos.x + i as isize, y: pos.y}) {
                *cref = (*c, color);
            }
        }

        let rect = TextBufferRect::new(pos.x, pos.y, text.len(), 1);
        self.add_dirty_rect(rect);
        rect
    }

    /// Write text to the display at a specific location respecting text alignment, takes in a slice of VGAChar's and a color for the text
    pub fn write_text_align(&mut self, pos: TextBufferPos, text: &[VGAChar], color: CharacterColor, align: TextAlign) -> TextBufferRect {
        self.write_text(align.align_text(pos, text.len()), text, color)
    }

    /// Write a string to the display at a specific location. Takes in an &str instead of a slice of VGAChar's. This function attempts to perform the conversion to VGAChar's, returning an error if the conversion cannot be performed
    pub fn write_string(&mut self, pos: TextBufferPos, text: &str, color: CharacterColor) -> Result<TextBufferRect, char> {
        for (i, c) in text.chars().enumerate() {
            if let Some(cref) = self.inner_mut_char(TextBufferPos{x: pos.x + i as isize, y: pos.y}) {
                *cref = (c.try_into()?, color);
            }
        }

        let rect = TextBufferRect::new(pos.x, pos.y, text.len(), 1);
        self.add_dirty_rect(rect);

        Ok(rect)
    }

    /// Write a string to the display at a specific location respecting text alignment. Takes in an &str instead of a slice of VGAChar's. This function attempts to perform the conversion to VGAChar's, returning an error if the conversion cannot be performed
    pub fn write_string_align(&mut self, pos: TextBufferPos, text: &str, color: CharacterColor, align: TextAlign) -> Result<TextBufferRect, char> {
        self.write_string(align.align_text(pos, text.len()), text, color)
    }

    /// Write a line of pre colored text to the display at the given position, takes in a slice of (VGAChar, CharacterColor) pairs. This is preferable for staticly allocated text
    pub fn write_data(&mut self, pos: TextBufferPos, text: &[(VGAChar, CharacterColor)]) -> TextBufferRect {
        for (i, c) in text.iter().enumerate() {
            if let Some(cref) = self.inner_mut_char(TextBufferPos{x: pos.x + i as isize, y: pos.y}) {
                *cref = *c;
            }
        }

        let rect = TextBufferRect::new(pos.x, pos.y, text.len(), 1);
        self.add_dirty_rect(rect);

        rect
    }

    /// Write a line of pre colored text to the display at the given position respecting text alignment, takes in a slice of (VGAChar, CharacterColor) pairs. This is preferable for staticly allocated text
    pub fn write_data_align(&mut self, pos: TextBufferPos, text: &[(VGAChar, CharacterColor)], align: TextAlign) -> TextBufferRect {
        self.write_data(align.align_text(pos, text.len()), text)
    }

    /// Get all of the dirty rectangles and clear the already present list
    pub fn take_dirty(&mut self) -> Vec<TextBufferRect> {
        let mut result = vec![];
        std::mem::swap(&mut self.dirty_regions, &mut result);

        result
    }

    /// Get and clear the clear optimization flag
    pub fn get_and_clear_optimization_flag(&mut self) -> bool {
        let backup = self.clear_optimization_heuristic;
        self.clear_optimization_heuristic = false;
        backup
    }

    /// Write the text buffer to a canvas display
    pub fn write_to_canvas<'a, T: sdl2::render::RenderTarget>(&mut self, canvas: &mut Canvas<T>, character_map: &mut CharacterMap<'a>, pixel_scale: usize) -> Result<(), String> {
        // Copy the text buffer to the canvas
        let mut last_color = (255, 255, 255);
        character_map.texture.set_color_mod(last_color.0, last_color.1, last_color.2);

        let mut dirty_rects = self.take_dirty();
        let clear_optimization = self.get_and_clear_optimization_flag();

        if clear_optimization {
            canvas.clear();
            dirty_rects.clear();
            dirty_rects.push(self.screen_rect());
        }

        for dirty_rect in dirty_rects {
            for x in dirty_rect.x..dirty_rect.right() {
                for y in dirty_rect.y..dirty_rect.bottom() {
                    if let Some((character, color)) = self.char_ref(TextBufferPos{x, y}) {
                        if !(clear_optimization && *character == VGAChar(b' ')) {
                            let source_rect = character_map.get_rect(character.0.into());
                            let dest_rect = character_map.get_dest_rect(x, y, pixel_scale);

                            let (r, g, b) = color.into();
                            if (r, g, b) != last_color {
                                character_map.texture.set_color_mod(r, g, b);
                                last_color = (r, g, b);
                            }

                            canvas.copy(&character_map.texture,
                                Some(source_rect),
                                Some(dest_rect)).map_err(|e| e.to_string())?;
                        }
                    }
                }
            }
        }

        canvas.present();

        Ok(())
    }

    /// Resize the text buffer
    pub fn resize_buffer(&mut self, width: usize, height: usize) {
        let mut new_data = vec![(VGAChar(b' '), CharacterColor::Gray); width * height];

        for x in 0..self.width.min(width) {
            for y in 0..self.height.min(height) {
                if let Some(v) = self.char_ref(TextBufferPos{ x: x as isize, y: y as isize}) {
                    new_data[x + width * y] = *v;
                }
            }
        }

        self.data = new_data;
        self.width = width;
        self.height = height;

        self.dirty_regions = vec![self.screen_rect()];
    }

    /// Get the width of the text buffer
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the text buffer
    pub const fn height(&self) -> usize {
        self.height
    }
}

impl TextBufferInterface for TextBufferScreen {
    fn add_dirty_rect(&mut self, rect: TextBufferRect) {
        self.add_dirty_rect(rect)
    }

    fn index_of(&self, pos: TextBufferPos) -> Option<usize> {
        self.index_of(pos)
    }

    fn inner_data_buffer(&self) -> &[(VGAChar, CharacterColor)] {
        &self.data
    }

    fn inner_data_buffer_mut(&mut self) -> &mut [(VGAChar, CharacterColor)] {
        &mut self.data
    }

    fn clear_rect(&mut self, rect: TextBufferRect) {
        if let Some(rect) = rect.intersection(&self.screen_rect()) {
            self.clear_rect(rect);
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

/// Text View for a subscreen of the full display
pub struct TextView<'a> {
    rect: TextBufferRect,
    parent_buffer: &'a mut TextBufferScreen
}

impl<'a> TextView<'a> {
    /// Construct a new text view
    pub fn new(rect: TextBufferRect, parent_buffer: &'a mut TextBufferScreen) -> Self {
        Self {
            rect,
            parent_buffer
        }
    }
}

impl<'a> TextBufferInterface for TextView<'a> {
    fn add_dirty_rect(&mut self, rect: TextBufferRect) {
        self.parent_buffer.add_dirty_rect(TextBufferRect {
            x: self.rect.x + rect.x.max(0),
            y: self.rect.y + rect.y.max(0),
            width: ((rect.x + rect.width as isize).min(self.rect.width as isize) - rect.x) as usize,
            height: ((rect.y + rect.height as isize).min(self.rect.height as isize) - rect.y) as usize })
    }

    fn index_of(&self, pos: TextBufferPos) -> Option<usize> {
        if self.rect.contains_point(pos.x + self.rect.x, pos.y + self.rect.y) {
            self.parent_buffer.index_of((pos.x + self.rect.x, pos.y + self.rect.y).into())
        }
        else {
            None
        }
    }

    fn inner_data_buffer(&self) -> &[(VGAChar, CharacterColor)] {
        self.parent_buffer.inner_data_buffer()
    }

    fn inner_data_buffer_mut(&mut self) -> &mut [(VGAChar, CharacterColor)] {
        self.parent_buffer.inner_data_buffer_mut()
    }

    fn clear_rect(&mut self, rect: TextBufferRect) {
        if let Some(rect) = self.screen_rect().intersection(&rect) {
            self.parent_buffer.clear_rect(TextBufferRect {
                x: self.rect.x + rect.x.max(0),
                y: self.rect.y + rect.y.max(0),
                width: rect.width.min(self.width()),
                height: rect.height.min(self.width()),
            })
        }
    }

    fn width(&self) -> usize {
        self.rect.width
    }

    fn height(&self) -> usize {
        self.rect.height
    }
}