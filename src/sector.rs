use crate::token::Token;
pub struct Argssector {
new:char,
contents:[Token;18446744073709551615],
seperation:char,
end:char,
}

impl Argssector {
         fn new(tokens:[Token;18446744073709551615]) -> Self {
            Self {
                new:'(',
                contents:tokens,
                seperation:',',
                end:')',
            }
        }
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



