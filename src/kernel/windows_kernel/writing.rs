use { Context, ScreenManager };
use std::rc::Rc;
use std::sync::Mutex;

use winapi::um::wincon;
use winapi::um::winnt::HANDLE;
use winapi::um::consoleapi::WriteConsoleW;
use winapi::um::wincon::{WriteConsoleOutputA, PSMALL_RECT, FillConsoleOutputAttribute, FillConsoleOutputCharacterA, COORD, CHAR_INFO};
use winapi::shared::ntdef::NULL;

use super::kernel;

/// Fill a certain block with characters.
pub fn fill_console_output_character(
    cells_written: &mut u32,
    start_location: COORD,
    cells_to_write: u32,
    screen_manager: &Rc<Mutex<ScreenManager>>,
) -> bool {

    let handle = kernel::get_current_handle(screen_manager);

    unsafe {
        // fill the cells in console with blanks
        let success = FillConsoleOutputCharacterA(
            handle,
            ' ' as i8,
            cells_to_write,
            start_location,
            cells_written,
        );
        kernel::is_true(success)
    }
}

/// Set console ouput attribute for certain block.
pub fn fill_console_output_attribute(
    cells_written: &mut u32,
    start_location: COORD,
    cells_to_write: u32,
    screen_manager: &Rc<Mutex<ScreenManager>>,
) -> bool {
    // Get the position of the current console window

    let (csbi, mut handle) = kernel::get_buffer_info_and_hande(screen_manager);

    let success;

    unsafe {
        success = FillConsoleOutputAttribute(
            handle,
            csbi.wAttributes,
            cells_to_write,
            start_location,
            cells_written,
        );
    }

    kernel::is_true(success)
}

/// Write console output.
pub fn write_console_output(
    write_buffer: &HANDLE,
    copy_buffer: &mut [CHAR_INFO; 160],
    buffer_size: COORD,
    buffer_coord: COORD,
    source_buffer: PSMALL_RECT,
) {
    use self::wincon::WriteConsoleOutputA;

    unsafe {
        if !kernel::is_true(
            WriteConsoleOutputA(
                *write_buffer,            // screen buffer to write to
                copy_buffer.as_mut_ptr(), // buffer to copy into
                buffer_size,              // col-row size of chiBuffer
                buffer_coord,             // top left dest. cell in chiBuffer
                source_buffer,
            ), // screen buffer source rectangle
        ) {
            panic!("Cannot write to console output");
        }
    }
}

use winapi::ctypes::c_void;
use std::str;

/// Write utf8 buffer to console.
pub fn write_char_buffer(handle: &HANDLE, buf: &[u8]) -> ::std::io::Result<usize> {
    // get string from u8[] and parse it to an c_str
    let mut utf8 = match str::from_utf8(buf) {
        Ok(string) => string,
        Err(_) => "123",
    };

    let utf16: Vec<u16> = utf8.encode_utf16().collect();
    let utf16_ptr: *const c_void = utf16.as_ptr() as *const _ as *const c_void;

    // get buffer info
    let csbi = kernel::get_console_screen_buffer_info_from_handle(handle);

    // get current position
    let current_pos = COORD {
        X: csbi.dwCursorPosition.X,
        Y: csbi.dwCursorPosition.Y,
    };

    let mut cells_written: u32 = 0;

    let mut success = false;
    // write to console
    unsafe {
        success = kernel::is_true(WriteConsoleW(
            *handle,
            utf16_ptr,
            utf16.len() as u32,
            &mut cells_written,
            NULL,
        ));
    }

    match success
        {
            // think this is wrong could be done better!
            true => Ok(utf8.as_bytes().len()),
            false => Ok(0)
        }
}