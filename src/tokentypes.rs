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
// TODO does:
args:Option<Argssector>,
end:char,
}

pub struct Notate {
    a:i64,
    operation:char,
    b:i64,
}

pub struct Rtrn {
    value:Argssector,
}
