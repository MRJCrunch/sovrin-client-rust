extern crate serde;
extern crate libc;
use self::libc::{c_int, c_void, int64_t, uint8_t};

use errors::crypto::CryptoError;
use services::anoncreds::helpers::BytesView;
extern crate rand;

use self::rand::os::{OsRng};
use self::rand::Rng;
use self::serde::ser::{Serialize, Serializer};
use self::serde::de::{Deserialize, Deserializer};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointG1 {
    point: String
}

pub struct PointG2 {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GroupOrderElement {
    bn: String
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pair {
    pair: String
}

impl PointG1 {
    pub fn new() -> Result<PointG1, CryptoError> {
        // generate random point from the group G1
        unimplemented!();
    }

    pub fn new_inf() -> Result<PointG1, CryptoError> {
        unimplemented!();
    }

    pub fn mul(&self, e: &GroupOrderElement) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }

    pub fn add(&self, q: &PointG1) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }

    pub fn sub(&self, q: &PointG1) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }

    pub fn neg(&self) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }
}

impl GroupOrderElement {
    pub fn new() -> Result<GroupOrderElement, CryptoError> {
        // returns random element in 0, ..., GroupOrder-1
        unimplemented!();
    }

    pub fn pow_mod(&self, e: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }

    pub fn add_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }

    pub fn sub_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }

    pub fn mul_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }

    pub fn inverse(&self) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }

    pub fn mod_neg(&self) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }
}

impl Pair {
    pub fn pair(p: &PointG1, q: &PointG1) -> Result<Pair, CryptoError> {
        unimplemented!();
    }

    pub fn mul(&self, b: &Pair) -> Result<Pair, CryptoError> {
        unimplemented!();
    }

    pub fn pow(&self, b: &GroupOrderElement) -> Result<Pair, CryptoError> {
        unimplemented!();
    }

    pub fn inverse(&self) -> Result<Pair, CryptoError> {
        unimplemented!();
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<Pair, CryptoError> {
        unimplemented!();
    }
}

impl BytesView for Pair {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.to_bytes()?)
    }
}

impl BytesView for PointG1 {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.to_bytes()?)
    }
}

impl BytesView for GroupOrderElement {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.to_bytes()?)
    }
}

impl Serialize for GroupOrderElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        unimplemented!();
    }
}

impl<'a> Deserialize<'a> for GroupOrderElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        unimplemented!();
    }
}

impl Serialize for Pair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        unimplemented!();
    }
}

impl<'a> Deserialize<'a> for Pair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        unimplemented!();
    }
}

impl Serialize for PointG1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        unimplemented!();
    }
}

impl<'a> Deserialize<'a> for PointG1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        unimplemented!();
    }
}