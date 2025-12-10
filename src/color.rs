use std::fmt;
#[derive(Default, Debug, Clone)] 
pub enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Magenta,
    Cyan,
    Black,
    White,
    #[default]
    System,
}
#[derive(Default, Debug, Clone)] 
pub struct ColorHolder{///<'a> {
    fg: Color,
    bg: Color,
    bright: bool,
    bright_bg: bool,
    bold: bool,
    italic: bool,
    value: String,
}
pub trait Colorized : std::fmt::Display {
   
    fn blue(&self) -> ColorHolder {
          ColorHolder{fg:Color::Blue,bg:Color::default(),value:self.to_string(),..Default::default()}
    }
    fn bright(&self) -> ColorHolder {
          ColorHolder{bright:true,value:self.to_string(),..Default::default()}
    }
    fn on_black(&self) -> ColorHolder {
          ColorHolder{bg:Color::Black,value:self.to_string(),..Default::default()}
    }
}
impl Colorized for ColorHolder {
    fn blue(&self) -> ColorHolder {
        let mut res = self.clone();
        res.fg = Color::Blue;
        res
    }
    fn on_black(&self) -> ColorHolder {
        let mut res = self.clone();
        res.bg = Color::Black;
        res
    }
    fn bright(&self) -> ColorHolder {
        let mut res = self.clone();
        res.bright = true;
        res
    }
}
impl fmt::Display for ColorHolder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut color = String::new();
        match self.fg {
            Color::Green => color.push_str("32;"),
            Color::Blue => if self.bright {color.push_str("94;")} else {color.push_str("34;")},
            _ => todo!("impl soon")
        }
        match self.bg {
            Color::Green => color.push_str("42;"),
            Color::Blue => color.push_str("44;"),
            Color::Black => color.push_str("40;"),
            _ => todo!("impl soon")
        }
        if color.is_empty() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "\x1b[{color}m{}\x1b[0m", self.value)
        }
    }
}
impl Colorized for str {}
impl Colorized for String {}
  
