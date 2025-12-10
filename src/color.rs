use std::fmt;
#[derive(Default, Debug, Clone, PartialEq)] 
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    #[default]
    Notset,
    Unset,
}
#[derive(Debug, Clone)] 
pub struct ColorHolder<B>{
    inner: B,
    fg: Color,
    bg: Color,
    bright: bool,
    bright_bg: bool,
    bold: bool,
    italic: bool,
    blink: bool,
    underline: bool,
    hidden: bool,
}
pub trait Colorized : Sized {
    fn color(self, fg: Color) -> ColorHolder<Self> {
        ColorHolder {
            inner: self,
            fg,
            bg: Default::default(),
            bright: Default::default(),
            bright_bg: Default::default(),
            bold: Default::default(),
            italic: Default::default(),
            blink: Default::default(),
            underline: Default::default(),
            hidden: Default::default(),
        }
    }
    fn on(self) -> ColorHolder<Self> {
        self.color(Color::Unset)
    }
    fn blue(self) -> ColorHolder<Self> {
          self.color(Color::Blue)
    }
    fn black(self) -> ColorHolder<Self> {
        self.color(Color::Black)
    }
    fn yellow(self) -> ColorHolder<Self> {
          self.color(Color::Yellow)
    }
    fn red(self) -> ColorHolder<Self> {
          self.color(Color::Red)
    }
    fn white(self) -> ColorHolder<Self> {
          self.color(Color::White)
    }
    fn green(self) -> ColorHolder<Self> {
        self.color(Color::Green)
    }
    fn cyan(self) -> ColorHolder<Self> {
          self.color(Color::Cyan)
    }
}

impl<T> ColorHolder<T> {
    fn color(mut self, color: Color) -> Self {
        if self.fg != Color::Notset {
            self.bg = color
        } else {
            self.fg = color
        }
        self
    }
    
    pub fn blue(self) -> Self {
        self.color(Color::Blue)
    }
    pub fn black(self) -> Self {
        self.color(Color::Black)
    }
    pub fn yellow(self) -> Self {
        self.color(Color::Yellow)
    }
    pub fn bright(mut self) -> Self {
        self.bright = true;
        self
    }
    pub fn on(mut self) -> Self {
        if self.fg == Color::Notset {
            self.fg = Color::Unset
        }
        self
    }

    fn ansi(&self) -> String {
        let mut color = String::new();
        if self.fg != Color::Notset && self.fg != Color::Unset {
            if self.bright {color.push('9')} else {color.push('3')} 
            color .push(get_color_num(&self.fg))
        }
        match self.bg {
            Color::Green => color.push_str("42;"),
            Color::Blue => color.push_str("44;"),
            Color::Black => color.push_str("40;"),
            Color::Yellow => color.push_str("43;"),
            _ => todo!("impl soon")
        }
        color
    }
}
fn get_color_num(color: &Color) -> char {
    match color {
        Color::Black => '0',
        Color::Red => '1',
        Color::Green  => '2',
        Color::Yellow => '3',
        Color::Blue => '4',
        Color::Magenta => '5',
        Color::Cyan => '6',
        Color::White => '7',
        _ => '0'
    }
}
impl<C> fmt::Display for ColorHolder<C> 
where
    C: fmt::Display,
{
    //fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = self.ansi();
        if color.is_empty() {
            write!(f, "{}", self.inner)
        } else {
            write!(f, "\x1b[{color}m{}\x1b[0m", self.inner)
        }
    }
}
impl Colorized for &str {}
impl Colorized for String {}
  
