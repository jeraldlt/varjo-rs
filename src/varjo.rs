use crate::varjo_types::*;

#[derive(Debug)]
pub struct Varjo {
    lib: libloading::Library,
    session_ptr: i64,

    pub is_initalized: bool,
    pub is_gaze_initialized: bool,
}

impl Varjo {
    pub fn new() -> Result<Self, String> {
        Self::new_from_path(String::from("varjo-sdk/bin/VarjoLib.dll"))
    }

    pub fn new_from_path(path: String) -> Result<Self, String> {
        let lib = unsafe { libloading::Library::new(path).map_err(|e| e.to_string())? };

        Ok(Self {
            lib,
            session_ptr: 0,
            is_initalized: false,
            is_gaze_initialized: false,
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
        self.is_gaze_initialized = false;

        Ok(())
    }

    pub fn gaze_init(&mut self) -> Result<(), String> {
        if !self.is_initalized {
            return Err(String::from(
                "Trying to init gaze on an uninitialized system.",
            ));
        }

        let func: libloading::Symbol<unsafe extern "C" fn(i64)> = unsafe {
            self.lib
                .get(b"varjo_GazeInit\0")
                .map_err(|e| e.to_string())?
        };

        unsafe {
            func(self.session_ptr);
        }

        self.is_gaze_initialized = true;

        Ok(())
    }

    pub fn request_gaze_calibration(&mut self) -> Result<(), String> {
        if !self.is_gaze_initialized {
            return Err(String::from("Gaze system is not initialized"));
        }

        let func: libloading::Symbol<unsafe extern "C" fn(i64)> = unsafe {
            self.lib
                .get(b"varjo_RequestGazeCalibration\0")
                .map_err(|e| e.to_string())?
        };

        unsafe {
            func(self.session_ptr);
        }

        Ok(())
    }

    pub fn get_gaze(&mut self) -> Result<VarjoGaze, String> {
        if !self.is_gaze_initialized {
            return Err(String::from("Gaze system is not initialized"));
        }

        let func: libloading::Symbol<unsafe extern "C" fn(i64) -> VarjoGaze> = unsafe {
            self.lib
                .get(b"varjo_GetGaze\0")
                .map_err(|e| e.to_string())?
        };

        let gaze = unsafe { func(self.session_ptr) };

        Ok(gaze)
    }
}
