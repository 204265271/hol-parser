use lalrpop_util::lalrpop_mod;
use std::io;
mod ast;
use crate::ast::{Ident, Type};

lalrpop_mod!(parser);

static mut VARS : Vec<Ident> = Vec::new();

fn main(){
    let mut input = String::new();
    loop{
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let res = parser::StmtParser::new().parse(&input).unwrap();
        res.print();

        if res.name != "it".to_string() {unsafe {
            let ln = VARS.len();
            let mut i = 0;
            while i < ln {
                if VARS[i].name == res.name
                {
                    break;
                }
                i = i + 1;
            }
            if i != ln
            {
                VARS[i] = res;
            }
            else {
                VARS.push(res);
            }

            println!("# print the var list:");
            for it in &VARS{
                it.print();
            }
        }}
    }
}