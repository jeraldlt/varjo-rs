use libloading::{self, Library};

fn main() -> Result<(), String> {
    let varjo = Varjo::new()?;

    let is_available = varjo.is_available()?;

    println!("{:?}", is_available);

    Ok(())
}

#[derive(Debug)]
pub struct Varjo {
    lib: libloading::Library,
    //functions: Option<VarjoFunctions<'a>>,
    session_ptr: i64,

    pub is_initalized: bool,
    //varjo_IsAvailable: &'a libloading::Symbol<'a, unsafe extern "C" fn() -> i32>,
}

impl Varjo {
    pub fn new() -> Result<Self, String> {
        Self::new_from_path(String::from("varjo-sdk/bin/VarjoLib.dll"))
    }

    pub fn new_from_path(path: String) -> Result<Self, String> {
        let lib = unsafe { Library::new(path).map_err(|e| e.to_string())? };

        Ok(Self {
            lib,
            session_ptr: 0,
            is_initalized: false,
        })
    }

    pub fn is_available(&self) -> Result<bool, String> {
        let func: libloading::Symbol<unsafe extern "C" fn() -> i32> = unsafe {
            self.lib
                .get(b"varjo_IsAvailable\0")
                .map_err(|e| e.to_string())?
        };

        let result = unsafe { func() };

        Ok(result != 0)
    }

    pub fn session_init(&mut self) -> Result<bool, String> {
        let func: libloading::Symbol<unsafe extern "C" fn() -> i64> = unsafe {
            self.lib
                .get(b"varjo_SessionInit\0")
                .map_err(|e| e.to_string())?
        };

        let result = unsafe { func() };

        if result > 0 {
            self.session_ptr = result;
            self.is_initalized = true;
        }

        return Ok(result > 0);
    }

    pub fn session_shut_down(&mut self) -> Result<(), String> {
        if !self.is_initalized {
            return Err(String::from("Trying to shut down an uninitialized system."));
        }

        let func: libloading::Symbol<unsafe extern "C" fn(i64)> = unsafe {
            self.lib
                .get(b"varjo_SessionShutDown\0")
                .map_err(|e| e.to_string())?
        };

        unsafe {
            func(self.session_ptr);
        }

        self.session_ptr = 0;
        self.is_initalized = false;

        Ok(())
    }
}
