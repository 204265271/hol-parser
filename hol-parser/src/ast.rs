pub struct Ident{
    pub name : String,
    pub tp : String,
    pub val : Type,
}

pub enum Type {
    num(i32),
    boolean(bool),
    string(String),
}

impl Ident {
    pub fn print(&self){
        print!("val {} : {} = ", self.name, self.tp);
        match &self.val {
            Type::num(n) => { println!("{}", n); },
            Type::boolean(b) => { println!("{}", b); },
            Type::string(s) => {println!("{}", s); },
        };
    }
}