use num_traits::{cast, Zero, Signed, PrimInt};


pub trait SignedNumToAscii<const N: usize> {}

fn inner_to_ascii<T: cast::AsPrimitive<u8> + PrimInt>(n: T, buff: &mut [u8] ) -> usize {
    let ten: T = T::from(10u8).unwrap();

    let mut v = n;

    for (idx, elem) in buff.iter_mut().enumerate().rev() {
        let rem = v % ten;

        v = v / ten;
        *elem = (rem.as_() as i8).abs() as u8 + 48;
        if v == Zero::zero() {
            return idx;
        }
    }
    0
}

pub trait NumToAscii<const N: usize>:
    cast::AsPrimitive<u8>
    + PrimInt
    
{
    fn to_ascii(&self) -> [u8; N] {
        let mut buff = [0u8; N];
        let _ = inner_to_ascii(*self, &mut buff);
        buff
    }
}

impl<T, const N: usize> NumToAscii<N> for T
where T: 
Signed
+ SignedNumToAscii<N>
+ cast::AsPrimitive<u8>
+ PrimInt
{
    fn to_ascii(&self) -> [u8; N] {
        let mut buff = [0u8; N];
        let digits = &mut buff[1..N];
        let idx = inner_to_ascii(*self,digits);
        if *self < Zero::zero() {
            buff[idx] = 45;
        }
        buff
    }
}

impl NumToAscii<3> for u8 {}
impl SignedNumToAscii<4> for i8 {}

impl NumToAscii<5> for u16 {}
impl SignedNumToAscii<6> for i16 {}

impl NumToAscii<10> for u32 {}
impl SignedNumToAscii<11> for i32 {}

impl NumToAscii<20> for u64 {}
impl SignedNumToAscii<21> for i64 {}

impl NumToAscii<39> for u128 {}
impl SignedNumToAscii<40> for i128 {}