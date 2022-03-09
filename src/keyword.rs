use crate::tokentypes::Func;
use crate::sector::Argssector;
pub struct Printnl {
    start:String,
    //takes:Argssector,
    end:char,
}
impl Printnl {
    fn new() -> Self {
       Self {
        start:"printnl".to_string(),
        end:';',
       } 
    }
}
