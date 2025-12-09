use std::fmt;
#[derive(Default, Debug)] 
enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Brown,
    #[default]
    System,
}
struct ColorHolder{///<'a> {
    fg: Color,
    bg: Color,
    bright: bool,
    value: String,//Box<dyn std::fmt::Display>, //&'a impl std::fmt::Display,
}
pub trait Colorized : std::fmt::Display {
    fn green(&self) -> Box<dyn std::fmt::Display + '_> {
        Box::new(ColorHolder{fg:Color::Green,bg:Color::default(),bright:false,value:self.to_string()})
    }
    fn blue(&self) -> Box<dyn std::fmt::Display + '_> {
          Box::new(ColorHolder{fg:Color::Blue,bg:Color::default(),bright:false,value:self.to_string()})
    }
    fn red(&self) -> Box<dyn std::fmt::Display + '_> {
        Box::new(ColorHolder{fg:Color::Red,bg:Color::default(),bright:false,value:self.to_string()})
    }

}
impl fmt::Display for ColorHolder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.fg {
            Color::Green => write!(f, "\x1b[32m{}\x1b[0m", self.value),
            Color::Blue => write!(f, "\x1b[34m{}\x1b[0m", self.value),
            Color::Red => write!(f, "\x1b[31m{}\x1b[0m", self.value),

            _ => write!(f, "{}", self.value)
        }
        
    }
}
impl Colorized for str {}
impl Colorized for String {}
  
