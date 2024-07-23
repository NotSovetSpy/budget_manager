#[derive(Debug)]
pub struct MenuPointer {
    position: [u32; 2],
    is_active: bool,
}

impl MenuPointer {
    pub fn new() -> Self {
        Self {
            position: [1, 1],
            is_active: false,
        }
    }

    // Move the pointer up
    pub fn up(&mut self) {
        if !self.is_active && self.position[0] > 1 {
            self.position[0] -= 1;
        }
    }

    // Move the pointer down
    pub fn down(&mut self) {
        if !self.is_active && self.position[0] < 3 {
            self.position[0] += 1;
        }
    }

    // Move the pointer left
    pub fn left(&mut self) {
        if !self.is_active && self.position[1] > 1 {
            self.position[1] -= 1;
        }
    }

    // Move the pointer right
    pub fn right(&mut self) {
        if !self.is_active && self.position[1] < 2 {
            self.position[1] += 1;
        }
    }

    pub fn is_tabs_selected(&self) -> bool {
        self.position == [1, 1] || self.position == [1, 2]
    }

    pub fn is_content_selected(&self) -> bool {
        self.position == [2, 1]
    }

    pub fn is_menu_selected(&self) -> bool {
        self.position == [2, 2]
    }

    pub fn is_bar_selected(&self) -> bool {
        self.position == [3, 1] || self.position == [3, 2]
    }
}
