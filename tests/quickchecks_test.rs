#[cfg(test)]
extern crate quickcheck;
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
use num_to_ascii::{NumToAscii};

#[cfg(test)]
mod tests {
    use crate::NumToAscii;

    #[macro_export]
    macro_rules! create_quickcheck {
        ($($t:ty),*) => {
            $(paste::item! {
                #[quickcheck]
                fn [< quickcheck_$t >] (input: $t) -> bool {
            
                    let res = input.to_ascii();
            
                    let out = core::str::from_utf8(&res).unwrap().trim_matches('\u{0}');
            
                    let str_out = format!("{}", input);
            
                    out == str_out
                }
            })*
        }
    }

    create_quickcheck!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
}