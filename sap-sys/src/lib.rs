#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    NulError(#[from] std::ffi::NulError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cxx::bridge]
pub mod ffi {
    // use cxx::UniquePtr;

    unsafe extern "C++" {
        include!("sap-sys/include/sap.h");
        type SapLocation;
        unsafe fn location_new(svr_name: *const i8, dev_id: i32) -> UniquePtr<SapLocation>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_location_ok() -> Result<()> {
        let svr_name = CString::new("test")?;
        let svr_name = svr_name.as_ptr();
        let location = unsafe { ffi::location_new(svr_name, 0) };
        Ok(())
    }
}
