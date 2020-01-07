use super::combinator::{alter, concat, map};

// Some macros to make combinators bearable
#[macro_export]
macro_rules! alternate {
    ($p:expr) => {
        $p
    };

    ($p:expr, $( $ps:expr ),+) => {
        alter($p, alternate!($($ps),+))
    }
}

#[macro_export]
macro_rules! sequence {
    ($p:expr) => {
        $p
    };

    ($( $ps:expr ),+, $p:expr) => {
        concat($p, sequence!($($ps),+))
    }
}