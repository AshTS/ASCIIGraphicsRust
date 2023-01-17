use crate::screen::TextBufferInterface;

/// Shared behavior for UIElements
pub trait UIElement {
    fn take_dirty(&mut self) -> bool;
    fn ui_draw(&mut self, screen: &mut impl TextBufferInterface);
    fn clear_last(&self, screen: &mut impl TextBufferInterface);
}