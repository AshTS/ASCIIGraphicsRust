use std::fmt::Display;

use crate::screen::{TextBufferPos, CharacterColor, TextAlign, TextBufferInterface, TextBufferRect, Drawable, TextFormatting, VGAChar};

use super::{SelectionMenu, UIElement};

pub struct DisplayBox {
    menu: SelectionMenu<T>,
    pos: TextBufferPos,
    settings: MenuSettings,
    last_rect: Option<TextBufferRect>,
    dirty: bool,
}

impl<T: Drawable<TextFormatting>> GraphicalMenu<T> {
    pub fn new(elements: Vec<T>, pos: TextBufferPos, settings: MenuSettings) -> Self {
        Self {
            menu: SelectionMenu::new(elements),
            pos,
            settings,
            last_rect: None,
            dirty: true,
        }
    }

    pub fn draw(&mut self, screen: &mut impl TextBufferInterface) {
        let mut running_rect: Option<TextBufferRect> = None;

        let pos = if !self.settings.fix_selected {
            self.pos
        }
        else {
            let i = self.menu.force_index() as isize;
            (self.pos.x - self.settings.menu_step.0 * i,
            self.pos.y - self.settings.menu_step.1 * i).into()
        };

        for (index, (is_selected, value)) in self.menu.elements_flagged().enumerate() {
            if is_selected || !self.settings.hide_others {
                let color = if is_selected { self.settings.selected_color } else { self.settings.unselected_color };
                let position = (pos.x + index as isize * self.settings.menu_step.0 as isize,
                                    pos.y + index as isize * self.settings.menu_step.1 as isize).into();

                let rect = value.draw(screen, position, &(color, self.settings.text_align).into());

                if let Some(running_rect) = &mut running_rect {
                    *running_rect = running_rect.union(&rect);
                }
                else {
                    running_rect = Some(rect);
                }
            }
        }

        self.last_rect = running_rect;
    }

    pub fn prev(&mut self) {
        let w = self.settings.wrapping;
        self.mut_menu().prev(w);
    }

    pub fn next(&mut self) {
        let w = self.settings.wrapping;
        self.mut_menu().next(w);
    }

    pub fn menu(&self) -> &SelectionMenu<T> {
        &self.menu
    }

    pub fn mut_menu(&mut self) -> &mut SelectionMenu<T> {
        self.dirty = true;
        &mut self.menu
    }

    pub fn clear_last(&self, screen: &mut impl TextBufferInterface) {
        if let Some(rect) = self.last_rect {
            screen.clear_rect(rect);
        }
    }

    pub fn take_dirty(&mut self) -> bool {
        let dirty = self.dirty;
        self.dirty = false;
        dirty
    }

    pub fn settings(&self) -> &MenuSettings {
        &self.settings
    }

    pub fn mut_settings(&mut self) -> &mut MenuSettings {
        self.dirty = true;
        &mut self.settings
    }

    pub fn set_pos(&mut self, pos: TextBufferPos) {
        self.pos = pos;
    }
}

impl<T: Drawable<TextFormatting>> UIElement for GraphicalMenu<T> {
    fn take_dirty(&mut self) -> bool {
        self.take_dirty()
    }

    fn ui_draw(&mut self, screen: &mut impl TextBufferInterface) {
        self.draw(screen)
    }

    fn clear_last(&self, screen: &mut impl TextBufferInterface) {
        self.clear_last(screen)
    }
}

impl<T: Display> Drawable<TextFormatting> for GraphicalMenu<T> {
    fn draw(&self, screen: &mut impl TextBufferInterface, pos: TextBufferPos, settings: &TextFormatting) -> TextBufferRect {
        let mut total = Vec::new();

        total.push((VGAChar(b'<'), settings.color));
        total.push((VGAChar(b' '), settings.color));

        let s = self.menu.force_selected().to_string();

        for c in s.chars() {
            total.push((c.try_into().unwrap(), settings.color));
        }

        total.push((VGAChar(b' '), settings.color));
        total.push((VGAChar(b'>'), settings.color));

        screen.write_data_align(pos, &total, settings.alignment)
    }
}