pub struct Game {
}

impl Game {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl crate::Loop for Game {
    fn init(&mut self, window: &mut crate::Window){
        window.set_title("Heptagon");
        let size = window.inner_size();
        let width = size.width;
    }

    fn update(&mut self, window: &mut crate::Window, delta: f64){
        // println!("FPS: {}", 1.0 / delta);
    }

    fn render(&mut self, window: &mut crate::Window){
    }
}