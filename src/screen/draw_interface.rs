use std::{fmt::Display, borrow::Borrow};

use super::{TextBufferPos, TextBufferRect, TextBufferInterface, CharacterColor, TextAlign};

/// A trait which grants objects the ability to be drawn to a text buffer surface
pub trait Drawable<Settings> {
    /// Draws this object to the text buffer interface at the given position
    fn draw(&self, screen: &mut impl TextBufferInterface, pos: TextBufferPos, settings: &Settings) -> TextBufferRect;
}

/// Text drawable trait to allow the simplest implementation of Drawable
pub trait TextDrawable<Settings> {
    /// Draws this object to the text buffer interface at the given position
    fn text_draw(&self, screen: &mut impl TextBufferInterface, pos: TextBufferPos, _settings: &Settings) -> TextBufferRect {
        screen.write_string(pos, self.as_str(), CharacterColor::BrightWhite).unwrap()
    }

    /// Get an &str representation of the object, implementing this function enables simple use of this trait to display text like objects
    fn as_str(&self) -> &str {
        "[TextDrawable Unimplemented]"
    }
}

impl<T, Settings> Drawable<Settings> for T where T: TextDrawable<Settings> {
    fn draw(&self, screen: &mut impl TextBufferInterface, pos: TextBufferPos, settings: &Settings) -> TextBufferRect {
        self.text_draw(screen, pos, settings)
    }
}

pub struct TextFormatting {
    pub color: CharacterColor,
    pub alignment: TextAlign
}

impl std::convert::From<(CharacterColor, TextAlign)> for TextFormatting {
    fn from(data: (CharacterColor, TextAlign)) -> Self {
        TextFormatting { color: data.0, alignment: data.1 }
    }
}

impl TextDrawable<TextFormatting> for String {
    fn text_draw(&self, screen: &mut impl TextBufferInterface, pos: TextBufferPos, settings: &TextFormatting) -> TextBufferRect {
        screen.write_string_align(pos, &self.to_string(), settings.color, settings.alignment).unwrap()
    }
}

impl<'a> TextDrawable<TextFormatting> for &'a str {
    fn text_draw(&self, screen: &mut impl TextBufferInterface, pos: TextBufferPos, settings: &TextFormatting) -> TextBufferRect {
        screen.write_string_align(pos, self, settings.color, settings.alignment).unwrap()
    }
}

impl<U, V: Sized> TextDrawable<U> for std::sync::Arc<std::cell::RefCell<V>> where V: TextDrawable<U> {
    fn text_draw(&self, screen: &mut impl TextBufferInterface, pos: TextBufferPos, settings: &U) -> TextBufferRect {
        self.try_borrow().unwrap().text_draw(screen, pos, settings)
    }
} 