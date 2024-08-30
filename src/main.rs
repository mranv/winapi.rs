use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        MessageBoxW(
            None,
            w!("Hello, World!"),
            w!("Windows Hello World"),
            MB_OK | MB_ICONINFORMATION,
        );
    }
    Ok(())
}