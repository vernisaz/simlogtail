use std::env;
use std::collections::HashSet;

#[cfg(unix)]
const OPT_PREFIX: char = '-';
#[cfg(windows)]
const OPT_PREFIX: char = '/';

#[derive(PartialEq)]
pub enum OptTyp {
    Num,
    FNum,
    Str,
    InStr,
    None
}
#[derive(PartialEq, Debug)]
pub enum OptVal {
    Num(i64),
    FNum(f64),
    Str(String),
    Arr(HashSet<(String,String)>),
    Empty,
    Unmatch
}
pub struct CliOpt {
    t: OptTyp,
    v: Option<OptVal>,
    nme: String,
    descr: Option<String>,
}
pub struct CLI {
    args: Vec<String>,
    opts: Vec<CliOpt>,
    descr: Option<String>,
    unprocessed: bool
}
impl CLI {
    pub fn new() -> Self {
        CLI {
            args: vec![],
            opts: vec![],
            descr: None,
            unprocessed: true
        }
    }
    
    pub fn opt(&mut self, name: &str, t: OptTyp) -> &mut Self {
        self.opts.push(CliOpt{ t:t, nme: name.to_string(), descr:None, v:None});
        self
    }
    
    pub fn description(&mut self, descr: &str) -> &mut Self {
        if self.opts.is_empty() {
            self.descr = Some(descr.to_string())
        } else {
            let indx = self.opts.len() - 1;
            self.opts[indx].descr = Some(descr.to_string())
        }
        self
    }
    
    pub fn get_description(&self) -> Option<String> {
        let mut descr = String::new();
        if self.descr.is_some() {
            descr += &self.descr.as_ref().unwrap()
        }
        for opt in &self.opts {
            descr += &format!("\n{OPT_PREFIX}{}", opt.nme);
            if opt.descr.is_some() {
                descr += &format!("\t{}", opt.descr.as_ref().unwrap())
            }
        }
        if descr.is_empty() {
            None
        } else {
            Some(descr)
        }
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
            if arg.starts_with(OPT_PREFIX) {
                // TODO eat extra -'s
                let sarg = arg.strip_prefix(OPT_PREFIX).unwrap();
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
                            OptTyp::InStr => ()
                        }
                    } else if opt.t == OptTyp::InStr && sarg.starts_with(&opt.nme) {
                        //let &(mut set);
                        if opt.v.is_none() {
                           opt.v = Some(OptVal::Arr(HashSet::new()))
                        } 
                        
                        match &opt.v {
                            Some(OptVal::Arr(set)) => {
                                let mut set = set.clone();
                                if let Some(pair) = sarg.strip_prefix(&opt.nme).unwrap().split_once('=') {
                                    set.insert((pair.0.to_string(), pair.1.to_string()));
                                } else {
                                    set.insert((sarg.strip_prefix(&opt.nme).unwrap().to_string(), String::new()));
                                }
                                opt.v = Some(OptVal::Arr(set))
                            }
                            _ => {
                            // somehow to report data inconsistency
                                opt.v = Some(OptVal::Arr(HashSet::new()))
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
