// Clone macro
// 
//
#[macro_export]
macro_rules! clone {
    ($($name:ident),+, $expression:expr) => {
        {
            $( let $name = $name.clone(); )+
            $expression
        }
    };
}

