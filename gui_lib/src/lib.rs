pub trait Draw{
    fn draw(&self);
}

pub struct  Screen{
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen{
    pub fn run(&self) {
        for component in self.components.iter(){
            component.draw();
        }
    }
}



pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self){
        //draw button
    }
} // Note even though we must implement the Draw trait and draw() method for button, we can implement other methods as well. Like an on-click method.


