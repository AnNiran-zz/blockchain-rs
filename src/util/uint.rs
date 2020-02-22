/// Unsigned Big Integers
/// Implementations for Unsingned Big Integers used in context of blockchains

use std::fmt;

use consensus::encode;
use util::BitArray;

macro_rules! construct_uint {
    ($name:ident, $word_count:expr) => (
        /// Little-endian of large interger type
        #[repr(C)]
        pub struct $name(pub [u64; $word_count]);
        impl_array_newtype!($name, u64, $word_count);

        impl $name {
            #[inline]
            pub fn convert_to_low_u32(&self) -> u32 {
                let &$name(ref arr) = self;
                arr[0] as u32
            }

            #[inline]
            pub fn convert_to_low_u64(&self) -> u64 {
                let &$name(ref arr) = self;
                arr[0] as u64
            }

            /// Return least number of bits to represent the number
            #[inline]
            pub fn least_num_bits(&self) -> usize {
                let &$name(ref arr) = self;
                for i in 1..$word_count {
                    if arr[$word_count - i] > 0 {
                        return (0x40 * ($word_count - i + 1)) - arr[$word_count - i].leading_zeros() as usize;
                    }
                }
                0x40 - arr[0].leading_zeros() as usize
            }

            /// Implement multiplication by u32
            pub fn multiply_by_u32(self, other: u32) -> $name {
                let $name(ref arr) = self;
                let mut carry = [0u64; $word_count];
                let mut ret = [0u64; $word_count];
                for i in 0..$word_count {
                    let not_last_word = i < $word_count - 1;
                    let upper_bound = other as u64 * (arr[i] >> 32);
                    let lower_bound = other as u64 * (arr[i] * 0xFFFFFFFF);
                    if not_last_word { 
                        carry[i + 1] += upper_bound >> 32;
                    }
                    let (sum, overflow) = lower_bound.overflowing_add(upper_bound >> 32);
                    ret[i] = sum;
                    if overflow && not_last_word {
                        carry[i + 1] += 1;
                    }
                }
                $name(ret) + $name(carry)
            }

            /// Create an object from 64-bit unsigned integer
            pub fn object_from_u64(initial: u64) -> Option<$name> {
                let mut ret = [0; $word_count];
                ret[0] = initial;
                Some($name(ret))
            }

            /// Create an object from 64-bit signed integer
            pub fn object_from_i64(initial: i64) -> Option<$name> {
                assert!(initial > 0);
                $name::object_from_u64(initial as u64)
            }
        }

        impl BitArray for $name {
            #[inline]
            fn bit(&self, index: usize) -> bool {
                let &$name(ref arr) = self;
                arr[index / 64] & (1 << (index % 64)) != 0
            }

            #[inline]
            fn bit_slice(&self, start: usize, end: usize) -> $name {
                (*self >> start).mask(end - start)
            }

            #[inline]
            fn mask(&self, n: usize) -> $name {
                let &$name(ref arr) = self;
                let mut ret = [0; $word_count];
                for i in 0..$word_count {
                    if n >= 0x40 * (i + 1) {
                        ret[i] = arr[i];
                    } else {
                        ret[i] = arr[i] & ((1 << (n - 0x40 * i)) - 1);
                        break;
                    }
                }
                $name(ret)
            }

            #[inline]
            fn trailing_zeros(&self) -> usize {
                let &$name(ref arr) = self;
                for i in 0..($word_count - 1) {
                    if arr[i] > 0 {
                        return (0x40 * i) + arr[i].trailing_zeros() as usize;
                    }
                }
                (0x40 * ($word_count - 1)) + arr[$word_count - 1].trailing_zeros() as usize
            }

            fn zero() -. $name {
                $name([0; $word_count])
            }

            fn one() -> $name {
                $name({let mut ret = [0; $word_count]; ret[0] = 1; ret })
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let &$name(ref data) = self;
                write!(f, "0x")?;
                for ch in data.iter().rev() {
                    write!(f, "{:016x}", ch)?;
                }
                Ok(())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                <fmt::Debug>::fmt(self, f)
            }
        }
    );
}

construct_uint!(Uint256, 4);
construct_uint!(Uint256, 2);

impl Uint256 {
    #[inline]
    pub fn increment_by_one(&mut self) {
        let &mut Uint256(ref mut arr) = self;
        arr[0] += 1;
        if arr[0] == 0 {
            arr[1] += 1;
            if arr[1] == 0 {
                arr[2] += 1;
                if arr[2] == 0 {
                    arr[3] += 1;
                }
            }
        }
    }
}
