/// Stores coordinates into the text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextBufferPos {
    pub x: isize,
    pub y: isize
}

impl std::convert::From<(isize, isize)> for TextBufferPos {
    fn from(data: (isize, isize)) -> Self {
        TextBufferPos {
            x: data.0,
            y: data.1
        }
    }
}

/// Stores rectangular coordinates refering to regions of the text buffer screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextBufferRect {
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize
}

impl TextBufferRect {
    /// Construct a new text buffer rectangle reference
    pub const fn new(x: isize, y: isize, width: usize, height: usize) -> Self {
        Self {
            x, y,
            width, height
        }
    }

    /// Get the interior of the rectangle
    pub const fn interior(&self) -> TextBufferRect {
        Self {
            x: self.x + 1,
            y: self.y + 1,
            width: self.width - 2,
            height: self.height - 2
        }
    }

    /// Get the right hand side of the rect
    pub const fn right(&self) -> isize {
        self.x + self.width as isize
    } 

    /// Get the bottom side of the rect
    pub const fn bottom(&self) -> isize {
        self.y + self.height as isize
    }

    /// Check containment of one rect in another
    pub const fn contains(&self, other: &TextBufferRect) -> bool {
        self.x >= other.x && self.y >= other.y && self.right() <= other.right() && self.bottom() <= other.bottom()
    }

    /// Check if a point is contained within the rect
    pub const fn contains_point(&self, x: isize, y: isize) -> bool {
        self.x <= x && self.right() > x && self.y <= y && self.bottom() > y
    }

    /// Check if two rects overlap
    pub const fn overlap(&self, other: &TextBufferRect) -> bool {
        self.contains_point(other.x, other.y) ||
        self.contains_point(other.x, other.bottom() - 1) ||
        self.contains_point(other.right() - 1, other.y) ||
        self.contains_point(other.right() - 1, other.bottom() - 1)
    }

    /// Get the intersection of two rects
    pub const fn intersection(&self, other: &TextBufferRect) -> Option<TextBufferRect> {
        if self.overlap(other) {
            let new_x = if self.x >= other.x { self.x } else { other.x };
            let new_y = if self.y >= other.y { self.y } else { other.y };

            let new_right = if self.right() <= other.right() { self.right() } else { other.right() };
            let new_bottom = if self.bottom() <= other.bottom() { self.bottom() } else { other.bottom() };

            Some(TextBufferRect::new(new_x, new_y, (new_right - new_x) as usize, (new_bottom - new_y) as usize))
        }
        else {
            None
        }
    }

    /// Get the "union" of two rects, actually the smallest rect that contains both
    pub const fn union(&self, other: &TextBufferRect) -> TextBufferRect {
        let new_x = if self.x <= other.x { self.x } else { other.x };
        let new_y = if self.y <= other.y { self.y } else { other.y };

        let new_right = if self.right() >= other.right() { self.right() } else { other.right() };
        let new_bottom = if self.bottom() >= other.bottom() { self.bottom() } else { other.bottom() };

        TextBufferRect::new(new_x, new_y, (new_right - new_x) as usize, (new_bottom - new_y) as usize)
    }

    /// Get the size of the rectangle
    pub const fn area(&self) -> usize {
        self.width * self.height
    }

    /// Get the horizontal center of the rectangle
    pub const fn horizontal_center(&self) -> isize {
        self.width as isize / 2 + self.x
    }

    /// Get the vertical center of the rectangle
    pub const fn vertical_center(&self) -> isize {
        self.height as isize / 2 + self.y
    }
}

impl std::convert::From<(isize, isize, usize, usize)> for TextBufferRect {
    fn from(data: (isize, isize, usize, usize)) -> Self {
        TextBufferRect { x: data.0, y: data.1, width: data.2, height: data.3 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Right,
    Center
}

impl TextAlign {
    pub fn align_text(&self, pos: TextBufferPos, length: usize) -> TextBufferPos {
        match self {
            TextAlign::Left => pos,
            TextAlign::Right => (pos.x - length as isize, pos.y).into(),
            TextAlign::Center => (pos.x - length as isize / 2, pos.y).into(),
        }
    }
}