pub trait Printable {
    fn print(&self);
}
pub struct Display{}
impl Display {
    pub fn clearScreen(&self){
        print!("\x1B[2J\x1B[1;1H");
    }
    pub fn printScreen<T: Printable>(&self, other: &T){
        self.clearScreen();
        other.print();
    }
}
