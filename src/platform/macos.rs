use crate::ClipboardProvider;

use raw_window_handle::HasRawDisplayHandle;
use std::error::Error;

pub fn connect<W: HasRawDisplayHandle>(
    _window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn Error>> {
    Ok(Box::new(clipboard_macos::Clipboard::new()?))
}

impl ClipboardProvider for clipboard_macos::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read()
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.write(contents)
    }
}
