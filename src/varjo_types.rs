
#[repr(C)]
pub struct VarjoRay {
    pub origin: [f64; 3],
    pub forward: [f64; 3],
}

#[repr(C)]
pub struct VarjoGaze {
    pub left_eye: VarjoRay,
    pub right_eye: VarjoRay,
    pub gaze: VarjoRay,
    pub focus_distance: f64,
    pub stability: f64,
    pub capture_time: i64,
    pub left_status: i64,
    pub right_status: i64,
    pub status: i64,
    pub frame_number: i64,
    pub left_pupil_size: f64,
    pub right_pupil_size: f64,
}

