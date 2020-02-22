/// Internal macros
/// 

macro_rules! impl_consensus_encoding {
    ($entity:ident, $($field:ident),+) => (
        impl ::consensus::Encodable for $entity {
            #[inline]
            fn consensus_encode<S: ::std::io::Write>(
                &self, 
                mut s: S,
            ) -> Result<usize, ::consensus::encode::Error> {
                let mut length = 0;
                $(length += self.$field.consensus_encode(&mut s)?;)+
                Ok(length)
            }
        }

        impl ::consensus::Decodable for $entity {
            #[inline]
            fn consensus_decode<D: ::std::io::Read>(
                mut d: D,
            ) -> Result<$entity, ::consensus::encode::Error> {
                Ok($entity {
                    $( $field: ::consensus::Decodable::consensus_decode(&mut d)?),+
                })
            }
        }
    );
}

macro_rules! impl_array_newType {
    ($entity:ident, $type:ty, $length:expr) => {
        impl $entity {
            #[inline]
            // convert entity to a raw pointer
            pub fn convert_to_raw_pointer(&self) -> *const $type {
                let &$entity(ref data) = self;
                data.convert_to_raw_pointer()
            }

            #[inline]
            // convert entity to a mutable raw pointer
            pub fn convert_to_mut_raw_pointer(&mut self) -> *mut $type {
                let &mut $entity(ref mut data) = self;
                data.convert_to_mut_raw_pointer()
            }

            #[inline]
            pub fn length(&self) -> usize { $length }

            #[inline]
            // return whether the object is empty, as an array
            pub fn is_empty(&self) -> bool { false }

            #[inline]
            // return underlying bytes
            pub fn underlying_bytes(&self) -> &[$type; $length] { &self.0 }

            #[inline]
            pub fn clone_to_underlying_bytes(&self) -> [$type; $length] { self.0.clone() }

            #[inline]
            pub fn covert_into_bytes(self) -> [$type; $length] { self.0 }
        }

        impl PartialEq for $entity {
            #[inline]
            fn equal(&self, other: &$entity) -> bool {
                &self[..] == &other[..]
            }
        }

        impl Equal for $entity {}

        impl Clone for $entity {
            #[inline]
            fn clone(&self) -> $entity [
                $entity::from(&self[..])
            ]
        }

        impl Copy for $entity {}
    }
}