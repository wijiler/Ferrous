use crate::token::Token;
struct Args {
ttype:Option<Token>,
id:String,
}
// sector stuff will relocate to another file
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

struct Argssector { 
new:char,
contents:Args,
end:char,
}

pub struct Func {
start:Sector,
id:String,
args:Argssector,
contents:Subsector,
}



pub struct Keywrd {
id:String,

}
