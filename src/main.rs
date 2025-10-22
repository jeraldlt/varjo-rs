
use varjo_rs::varjo::Varjo;

fn main() -> Result<(), String> {
    let mut varjo = Varjo::new()?;

    let is_available = varjo.is_available()?;

    println!("{:?}", is_available);

    varjo.session_init()?;

    varjo.gaze_init()?;

    varjo.request_gaze_calibration()?;

    loop {
        let gaze = varjo.get_gaze()?;

        println!("Frame: {}; Forward: {:?}", gaze.frame_number, gaze.gaze.forward);
    }

    varjo.session_shut_down()?;

    Ok(())
}
