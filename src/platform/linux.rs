use crate::ClipboardProvider;

use raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
use std::error::Error;

pub use clipboard_wayland as wayland;
pub use clipboard_x11 as x11;

pub fn connect<W: HasDisplayHandle>(
    window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn Error>> {
    let Ok(display_handle) = window.display_handle() else {
        Err("Window does not have a display handle")?
    };

    let clipboard = match display_handle.as_raw() {
        RawDisplayHandle::Wayland(handle) => Box::new(unsafe {
            wayland::Clipboard::connect(handle.display.as_ptr() as *mut _)
        }) as _,
        RawDisplayHandle::Xlib(_) | RawDisplayHandle::Xcb(_) => {
            Box::new(x11::Clipboard::connect()?) as _
        }
        _ => return Err("Unsupported display type")?,
    };

    Ok(clipboard)
}

impl ClipboardProvider for wayland::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read()
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.write(contents)
    }
}

impl ClipboardProvider for x11::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read().map_err(Box::from)
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.write(contents).map_err(Box::from)
    }
}
