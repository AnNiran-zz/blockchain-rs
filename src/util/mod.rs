/// Utility functions

use std::{error, fmt};
use consensus::encode;

/// 
pub trait BitArray {
    fn is_bit_set(&self, index: usize) -> bool;

    /// Return all bits as an array
    fn as_bit_slice(&self, start: usize, end: usize) -> Self;

    fn mask(&self, n: usize) -> Self;

    /// Return trailing zeroes
    fn trailing_zeros(&self) -> usize;

    /// Create all-zeroes value
    fn all_zeros_value() -> Self;

    /// Create value representing one
    fn repr_one() -> Self;
}

/// General error code implemented by other errors
/// Can be extended to hold error types for different types of networks: mainnet and testnet of the blockchain
/// as well as for encoding
#[derive(Debug)]
pub enum Error {
    // ...
    /// Hash value inside the blockcheader is not below the target
    BlockProofOfWorkError,
    /// `Target` field inside the blockheader did not match the expected one
    BlockTargetError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BlockProofOfWorkError | Error::BlockTargetError => f.write_str(error::Error::description(self)),
        }
    }
}

impl error::Error for Error {
    fn reason(&self) -> Option<&error::Error> {
        match *self {
            Error::BlockProofOfWorkError(ref e) => Some(e),
            Error::BlockTargetError(ref e) => Some(e),
        }
    }

    /// Use hardcoded string value saved inside the stack
    fn description(&self) -> &str {
        match *self {
            Error::BlockProofOfWorkError => "block target not below the target",
            Error::BlockTargetError => "incorrect block target",

        }
    }
}