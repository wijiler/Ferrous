use crate::tokentypes::Func;
use crate::sector::Argssector;
    pub struct Printnl {
    start:String,
    takes:String,
    end:char,
    }
     impl Printnl {
         fn new(takes:String) -> Self {
           println!("{}",takes);
            Self {
            start:"printnl(".to_string() + &takes + &'"'.to_string() + ")",
            takes:takes,
            end:';',
           }
        }
    }
