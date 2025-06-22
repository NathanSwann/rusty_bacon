mod bindings;
use std::convert::From;
use std::ffi::CString;

#[derive(Clone, Debug)]
pub struct BQNCastError;
impl std::fmt::Display for BQNCastError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[repr(transparent)]
pub struct BqnValue(pub bindings::BQNV);

impl BqnValue {
    pub fn get_type(&self) -> i32 {
        unsafe { bindings::bqn_type(self.0) }
    }
}

impl From<f64> for BqnValue {
    fn from(item: f64) -> Self {
        unsafe { BqnValue(bindings::bqn_makeF64(item)) }
    }
}

impl From<String> for BqnValue {
    fn from(item: String) -> Self {
        let cstr = CString::new(item).unwrap();
        unsafe { BqnValue(bindings::bqn_makeUTF8Str(cstr.count_bytes(), cstr.as_ptr())) }
    }
}

impl TryFrom<BqnValue> for f64 {
    type Error = BQNCastError;

    fn try_from(value: BqnValue) -> Result<Self, Self::Error> {
        if unsafe { 1 == bindings::bqn_type(value.0) } {
            Ok(unsafe { bindings::bqn_toF64(value.0) })
        } else {
            Err(BQNCastError {})
        }
    }
}

impl Clone for BqnValue {
    fn clone(&self) -> Self {
        BqnValue(unsafe { bindings::bqn_copy(self.0) })
    }
}

impl From<Vec<f64>> for BqnValue {
    fn from(value: Vec<f64>) -> Self {
        BqnValue(unsafe { bindings::bqn_makeF64Vec(value.len(), value.as_ptr()) })
    }
}

impl TryFrom<BqnValue> for Vec<f64> {
    type Error = BQNCastError;
    fn try_from(value: BqnValue) -> Result<Self, Self::Error> {
        if unsafe { bindings::bqn_type(value.0) } != 0 {
            println!("ERROR INVALID Not a vec");
            return Err(BQNCastError {});
        }
        let arry_type = unsafe { bindings::bqn_directArrType(value.0) };
        if let bindings::BQNElType_elt_f64
        | bindings::BQNElType_elt_i16
        | bindings::BQNElType_elt_i32
        | bindings::BQNElType_elt_i8 = arry_type
        {
        } else {
            println!("ERROR INVALID Type type was {:?}", unsafe {
                bindings::bqn_directArrType(value.0)
            });
            return Err(BQNCastError {});
        }
        let r: usize = unsafe { bindings::bqn_bound(value.0) };
        let mut nv: Vec<f64> = vec![0.0; r];
        unsafe { bindings::bqn_readF64Arr(value.0, nv.as_mut_slice().as_mut_ptr()) }
        Ok(nv)
    }
}

impl TryFrom<BqnValue> for String {
    type Error = BQNCastError;

    fn try_from(value: BqnValue) -> Result<Self, Self::Error> {
        if unsafe { bindings::bqn_type(value.0) } != 0 {
            println!("ERROR INVALID Not a vec");
            return Err(BQNCastError {});
        }
        let arry_type = unsafe { bindings::bqn_directArrType(value.0) };
        if let bindings::BQNElType_elt_c8
        | bindings::BQNElType_elt_c16
        | bindings::BQNElType_elt_c32 = arry_type
        {
        } else {
            println!("ERROR INVALID Type type was {:?}", unsafe {
                bindings::bqn_directArrType(value.0)
            });
            return Err(BQNCastError {});
        }
        let r: usize = unsafe { bindings::bqn_bound(value.0) };
        let mut nv: Vec<u8> = vec![0; r];
        unsafe { bindings::bqn_readC8Arr(value.0, nv.as_mut_slice().as_mut_ptr()) }
        Ok(String::from_utf8(nv).expect("unable to read string"))
    }
}

pub fn call_bqn_1(f: BqnValue, x: BqnValue) -> BqnValue {
    BqnValue(unsafe { bindings::bqn_call1(f.0, x.0) })
}

pub fn call_bqn_str(s: String) -> BqnValue {
    BqnValue(unsafe { bindings::bqn_evalCStr(s.as_ptr() as *const i8) })
}
