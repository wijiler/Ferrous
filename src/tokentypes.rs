use crate::token::Token;
/* uncomment if you get some errors related to this
struct Argssector {
new:char,
ttype:Option<Token>,
id:String,
end:char,
}

struct Subsector {
contents:Option<Token>,
end:char,
}

struct Sector {
new:char,
contents:&'static [String],
subsectors:&'static [i64],
end:char,
}
*/
use crate::sector::{Sector,Subsector,Argssector};
pub struct Func {
start:Sector,
id:String,
args:Option<Argssector>,
contents:Subsector,
}



pub struct Keywrd {
id:String,
does:Option<T>, // TODO
args:Option<Argssector>,

}
