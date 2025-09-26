use std::env;

pub enum OptTyp {
    Num,
    FNum,
    Str,
    None
}
#[derive(PartialEq, Debug)]
pub enum OptVal {
    Num(i64),
    FNum(f64),
    Str(String),
    Empty,
    Unmatch
}
pub struct CliOpt {
    t: OptTyp,
    v: Option<OptVal>,
    nme: String,
}
pub struct CLI {
    args: Vec<String>,
    opts: Vec<CliOpt>,
    unprocessed: bool
}
impl CLI {
    pub fn new() -> Self {
        CLI {
            args: vec![],
            opts: vec![],
            unprocessed: true
        }
    }
    
    pub fn opt(&mut self, name: &str, t: OptTyp) -> &mut Self {
        self.opts.push(CliOpt{ t:t, nme: name.to_string(), v:None});
        self
    }
    
    pub fn get_opt(&mut self, name: &str) -> Option<&OptVal> {
        if self.unprocessed {
            self.parse()
        }
        for opt in &self.opts {
            if opt.nme == name {
                return opt.v.as_ref()
            }
        }
        None
    }
    
    pub fn args(&mut self) -> &Vec<String> {
        if self.unprocessed {
            self.parse()
        }
        &self.args
    }
    
    fn parse(&mut self) {
        let mut args = env::args();
        args.next(); // swallow first
        while let Some(arg) = args.next() {
            if arg.starts_with('-') {
                // TODO eat extra -
                let sarg = arg.strip_prefix('-').unwrap();
                for opt in &mut self.opts {
                    if opt.nme == sarg {
                        match opt.t {
                            OptTyp::Num => {
                                match args.next() {
                                    Some(val) => {
                                        match val.parse::<i64>() {
                                            Ok (num) => opt.v = Some(OptVal::Num(num)),
                                            _ => opt.v = Some(OptVal::Unmatch)
                                        }
                                    }
                                    _ => ()
                                }
                            },
                            OptTyp::None => opt.v = Some(OptVal::Empty),
                            OptTyp::FNum => match args.next() {
                                    Some(val) => {
                                        match val.parse::<f64>() {
                                            Ok (num) => opt.v = Some(OptVal::FNum(num)),
                                            _ => opt.v = Some(OptVal::Unmatch)
                                        }
                                    }
                                    _ => ()
                                }
                            OptTyp::Str => match args.next() {
                                Some(str) => opt.v = Some(OptVal::Str(str)),
                                _ => ()
                            }
                        }
                    }
                }
            } else {
                self.args.push(arg)
            }
        }
        self.unprocessed = false
    }
}
