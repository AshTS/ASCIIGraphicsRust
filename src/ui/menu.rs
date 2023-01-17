/// Internal representation of a menu with a Vec of T's
pub struct SelectionMenu<T> {
    elements: Vec<T>,
    selected_index: Option<usize>,
    reselect_index: usize
}

impl<T> SelectionMenu<T> {
    /// Create a new SelectionMenu from a vector of elements
    pub fn new(elements: Vec<T>) -> Self {
        Self {
            elements,
            selected_index: None,
            reselect_index: 0
        }
    }

    /// Move the menu to the previous item in the vector
    pub fn prev(&mut self, wrapping: bool) {
        if let Some(index) = &mut self.selected_index {
            if *index == 0 {
                if wrapping {
                    *index = self.elements.len() - 1;
                }
            }
            else {
                *index -= 1;
            }
        }
        else {
            self.selected_index = Some(self.reselect_index.min(self.elements.len() - 1));
        }
    }

    /// Move the menu to the next item in the vector
    pub fn next(&mut self, wrapping: bool) {
        if let Some(index) = &mut self.selected_index {
            if *index == self.elements.len() - 1 {
                if wrapping {
                    *index = 0;
                }
            }
            else {
                *index += 1;
            }
        }
        else {
            self.selected_index = Some(self.reselect_index.min(self.elements.len() - 1));
        }
    }

    /// Deselect the current element in the vector
    pub fn deselect(&mut self) {
        if let Some(previous_index) = self.selected_index.take() {
            self.reselect_index = previous_index
        }
    }

    /// Get a reference to the currently selected element
    pub fn selected(&self) -> Option<&T> {
        self.elements.get(self.selected_index?)
    }

    /// Get the selected index of the menu
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    /// Returns true if the first element in the vector is selected
    pub fn is_first_selected(&self) -> bool {
        self.selected_index == Some(0)
    }

    /// Returns true if the last element in the vector is selected
    pub fn is_last_selected(&self) -> bool {
        self.selected_index == Some(self.elements.len() - 1)
    }

    /// Add an element to the end of the menu
    pub fn add_element(&mut self, element: T) {
        self.elements.push(element);
    }

    /// Remove the element at the given index in the menu
    pub fn remove_element_index(&mut self, index: usize) -> T {
        if let Some(selected_index) = &mut self.selected_index {
            if *selected_index >= index && *selected_index > 0 {
                *selected_index -= 1;
            }
        }

        if self.reselect_index >= index && self.reselect_index > 0 {
            self.reselect_index -= 1;
        }

        self.elements.remove(index)
    }

    /// Access the elements in order
    pub fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    /// Access an element mutably
    pub fn mutable_element(&mut self, index: usize) -> Option<&mut T> {
        self.elements.get_mut(index)
    }

    /// Access the elements in order with a boolean denoting if the given value is selected
    pub fn elements_flagged(&self) -> impl Iterator<Item=(bool, &T)> {
        let selected = self.selected_index;
        self.elements.iter().enumerate().map(move |(i, v)| (selected == Some(i), v))
    }

    /// Get the index at the selected position or where the menu would return to
    pub fn force_index(&self) -> usize {
        self.selected_index.unwrap_or(self.reselect_index)
    }

    /// Get the value at the selected position or where the menu would return to
    pub fn force_selected(&self) -> &T {
        &self.elements[self.force_index()]
    }

    /// Replace the vector of elements
    pub fn replace_elements(&mut self, elements: Vec<T>) {
        self.elements = elements;
        self.reselect_index = self.reselect_index.min(self.elements.len() - 1);

        if let Some(v) = &mut self.selected_index {
            *v = (*v).min(self.elements.len() - 1);
        }
    }
}