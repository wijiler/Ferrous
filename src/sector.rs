use crate::token::Token;
pub struct Argssector {
new:char,
ttype:Option<Token>,
id:String,
end:char,
}

pub struct Subsector {
contents:Option<Token>,
end:char,
}

pub struct Sector {
new:char,
contents:&'static [String],
subsectors:&'static [i64],
end:char,
}



