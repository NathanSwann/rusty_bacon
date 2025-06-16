#![allow(warnings)]
mod bindings;
use std::convert::From;

pub struct BqnValue(pub bindings::BQNV);

impl From<f64> for BqnValue {
    fn from(item: f64) -> Self {
        unsafe { BqnValue(bindings::bqn_makeF64(item)) }
    }
}

impl TryFrom<BqnValue> for f64 {
    type Error = &'static str;

    fn try_from(value: BqnValue) -> Result<Self, Self::Error> {
        if unsafe { 1 == bindings::bqn_type(value.0) } {
            Ok(unsafe { bindings::bqn_toF64(value.0) })
        } else {
            Err("BQNV does not contain an f64")
        }

    }
}