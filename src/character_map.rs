use std::path::PathBuf;

use sdl2::{render::{Texture, TextureCreator}, rect::Rect};

pub struct CharacterMap<'a> {
    pub texture: Texture<'a>,
    character_size: (usize, usize),
    start_pos: (usize, usize),
    per_row: usize
}

impl<'a> CharacterMap<'a> {
    /// Construct a new character map from a bitmap texture
    pub fn new(texture: Texture<'a>, character_size: (usize, usize), start_pos: (usize, usize), per_row: usize) -> Self {
        Self {
            texture,
            character_size,
            start_pos,
            per_row
        }
    }

    /// Construct a new character map from a filename
    pub fn from_file<T: 'a>(path: impl Into<PathBuf>, texture_creator: &'a TextureCreator<T>, character_size: (usize, usize), start_pos: (usize, usize), per_row: usize) -> Result<Self, String> {
        let temp_surface = sdl2::surface::Surface::load_bmp(path.into())?;
        let texture = texture_creator
            .create_texture_from_surface(&temp_surface)
            .map_err(|e| e.to_string())?;

        Ok(Self::new(texture, character_size, start_pos, per_row))
    }

    /// Get the size of a character for the map
    pub const fn character_size(&self) -> (usize, usize) {
        self.character_size
    }

    /// Get the rectangle for a character
    pub fn get_rect(&self, character_index: u8) -> Rect {
        let rows = 256 / self.per_row;

        Rect::new(self.start_pos.0 as i32 + self.character_size.0 as i32 * (character_index as usize % self.per_row) as i32,
                  self.start_pos.1 as i32 + self.character_size.1 as i32 * ((character_index as usize / self.per_row) % rows) as i32,
              self.character_size.0 as u32,
             self.character_size.1 as u32)
    }

    /// Get the destination rectangle for a character
    pub fn get_dest_rect(&self, x: isize, y: isize, pixel_scale: usize) -> Rect {
        Rect::new((x * self.character_size.0 as isize * pixel_scale as isize / 100) as i32,
            (y * self.character_size.1 as isize * pixel_scale as isize / 100) as i32,
            (self.character_size.0 * pixel_scale / 100) as u32, 
            (self.character_size.1 * pixel_scale / 100) as u32)
    }
}