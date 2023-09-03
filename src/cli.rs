use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Cmd {
    args: Vec<Arg>,
    cmds: HashMap<String, Cmd>,
    err: Option<Error>,
    pub name: String,
    opts: HashMap<String, Opt>,
    raw_args: Vec<String>,
    taken_arg: usize,
    taken_len: usize
}

impl Cmd {
    pub fn new(name: &str) -> Cmd {
        Cmd {
            args: vec![],
            cmds: HashMap::new(),
            err: None,
            name: name.to_string(),
            opts: HashMap::new(),
            raw_args: vec![],
            taken_arg: 0,
            taken_len: 0
        }
    }
    pub fn add_arg(mut self, arg: Arg) -> Self {
        if self.is_err() {
            return self
        }
        self.args.push(arg);
        self
    }
    pub fn add_cmd(mut self, cmd: Cmd) -> Self {
        if self.is_err() {
            return self
        }
        if self.cmds.contains_key(&cmd.name) ||
            !cmd.name.contains(|c: char| { c.is_ascii_alphabetic() }){
            self.err = Some(Error { source: "error: invalid command name.".to_string() });
            return self
        }
        self.cmds.insert(cmd.name.clone(), cmd);
        self
    }
    pub fn add_opt(mut self, opt: Opt) -> Self {
        if self.is_err() {
            return self
        }
        if self.opts.contains_key(&opt.name) || !opt.name.contains(|c: char| { c.is_ascii_alphanumeric() }) {
            self.err = Some(Error { source: "error: invalid option name".to_string() });
            return self
        }
        if self.opts.contains_key(&opt.short_name) ||
            (!opt.short_name.contains(|c: char| { c.is_ascii_alphanumeric() }) &&
            opt.short_name != "") {
            self.err = Some(Error { source: "error: invalid option short name".to_string() });
            return self
        }
        self.opts.insert(opt.name.clone(), opt.clone());
        if opt.short_name != "" {
            self.opts.insert(opt.short_name.clone(), opt);
        }
        self
    }
    pub fn err(&self) -> Option<Error> {
        self.err.clone()
    }
    pub fn is_err(&self) -> bool {
        return self.err.is_some()
    }
    pub fn len(&self) -> usize {
        self.raw_args.len() - self.taken_len
    }
    pub fn main() -> Result<Cmd, Error> {
        let args: Vec<_> = std::env::args().into_iter().collect();
        let name: &str;
        let executable_name =  args.get(0).unwrap();
        if cfg!(target_os = "windows") {
            let Some(name_) = executable_name
                .split('\\')
                .last()
                .unwrap()
                .strip_suffix(".exe")
            else { return Err(Error { source: "error: unknown error.".to_string() }) };
            name = name_;
        } else if(cfg!(target_os = "linux")) {
            let Some(name_) = executable_name.split('/').last()
            else { return Err(Error { source: "error: unknown error.".to_string() }) };
            name = name_
        } else { return Err(Error { source: "error: unknown error.".to_string() }) }
        let mut cmd = Cmd::new(name);
        cmd.raw_args = std::env::args().into_iter().collect();
        cmd.raw_args = cmd.raw_args
            .split_first()
            .unwrap_or((&"".to_string(), &["".to_string()]))
            .1.to_vec();
        Ok(cmd)
    }
    pub fn parse(mut self, s: &str) -> Self {
        self.raw_args = s
            .split_whitespace()
            .map(|x| x.to_string())
            .filter(|x| *x != self.name)
            .collect();
        self
    }
}

impl Iterator for Cmd {
    type Item = Input;

    fn next(&mut self) -> Option<Self::Item> {
        let pre = self.raw_args.get(self.taken_len);
        if pre.is_none() {
            return None
        }
        let first = pre.unwrap();
        self.taken_len += 1;
        if first.starts_with("-") {
            let start: usize = if first.starts_with("--") { 2 } else { 1 };
            let s = first.split_at(start).1.to_string();
            if s.is_empty() {
                self.err = Some(Error { source: "error: empty option.".to_string() });
                return None
            }
            if self.opts.contains_key(&s) {
                let opt = self.opts.get(&s).unwrap();
                if opt.require_value.is_none() {
                    Some(Input::Opt(opt.clone(), "".to_string()))
                } else {
                    let second = self.raw_args.get(self.taken_len);
                    self.taken_len += 1;
                    if second.is_none() && opt.require_value.unwrap() {
                        Some(Input::Err(Error { source: "error: option requires a value".to_string() }))
                    } else {
                        Some(Input::Opt(opt.clone(), second.unwrap_or(&"".to_string()).to_string()))
                    }
                }
            } else {
                Some(Input::Err(Error { source: "error: unknown option.".to_string() }))
            }
        } else if self.cmds.contains_key(first) {
            let mut cmd = self.cmds.get(first).unwrap().clone();
            cmd.raw_args = self.raw_args.split_at(self.taken_len).1.to_vec();
            self.taken_len = self.raw_args.len();
            Some(Input::Cmd(cmd))
        } else {
            if self.taken_arg >= self.args.len() {
                return Some(Input::Err(Error {
                    source: "error: received too more arguments".to_string()
                }))
            }
            let arg = self.args.get(self.taken_arg + 1).unwrap().clone();
            self.taken_arg += 1;
            Some(Input::Arg(arg, first.clone()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    source: String
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug, Clone)]
pub enum Input {
    Arg(Arg, String),
    Cmd(Cmd),
    Err(Error),
    Opt(Opt, String)
}

#[derive(Debug, Clone)]
pub struct Opt {
    name: String,
    short_name: String,
    require_value: Option<bool>
}

impl Opt {
    pub fn new(name: &str, short_name: &str, require_value: Option<bool>) -> Opt {
        Opt {
            name: name.to_string(),
            short_name: short_name.to_string(),
            require_value
        }
    }
}
