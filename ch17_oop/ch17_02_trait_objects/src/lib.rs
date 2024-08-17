pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // dynamic dispatch
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        // dynamic dispatch
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// static dispatch  
impl Draw for Button {
    fn draw(&self) {
        // draw button
    }
}
