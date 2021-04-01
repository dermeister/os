use core::fmt;
use core::fmt::Write;
use crate::spinlock::Spinlock;
use crate::lazy::SyncLazy;

#[derive(Copy, Clone)]
enum VgaColor {
    Black = 0,
    White = 15,
}

#[derive(Copy, Clone)]
#[repr(transparent)]
struct VgaChar(u16);

impl VgaChar {
    pub fn new(c: u8, background: VgaColor, foreground: VgaColor) -> Self {
        let background = (background as u16) << 12;
        let foreground = (foreground as u16) << 8;
        let c = c as u16;
        Self(background | foreground | c)
    }
}

const VGA_ROWS: usize = 25;
const VGA_COLS: usize = 80;

type VgaRow = [VgaChar; VGA_COLS];
type VgaBuffer = [VgaRow; VGA_ROWS];

pub struct VgaWriter {
    row: usize,
    col: usize,
    buffer: &'static mut VgaBuffer,
    background: VgaColor,
    foreground: VgaColor,
}

impl VgaWriter {
    pub fn new() -> Self {
        let mut writer = VgaWriter {
            row: 0,
            col: 0,
            buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) },
            foreground: VgaColor::Black,
            background: VgaColor::White,
        };

        writer.clean();
        writer
    }

    pub fn write(&mut self, text: &str) {
        for c in text.bytes() {
            self.scroll_if_needed();
            match c {
                b'\n' => self.write_new_line(),
                c => self.write_char(c)
            }
        }
    }

    fn scroll_if_needed(&mut self) {
        if self.row == VGA_ROWS {
            for i in 1..self.buffer.len() {
                self.buffer.copy_within(i..=i, i - 1);
            }

            self.row -= 1;
        }
    }

    fn write_new_line(&mut self) {
        self.col = 0;
        self.row += 1;
    }

    fn write_char(&mut self, c: u8) {
        self.buffer[self.row][self.col] = VgaChar::new(c, self.background, self.foreground);
        self.next_position();
    }

    fn next_position(&mut self) {
        self.col += 1;

        if self.col == VGA_COLS {
            self.col = 0;
            self.row += 1;
        }
    }

    fn clean(&mut self) {
        for c in self.buffer.iter_mut().flatten() {
            *c = VgaChar::new(0, VgaColor::White, VgaColor::White);
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

static WRITER: SyncLazy<Spinlock<VgaWriter>> = SyncLazy::new(|| Spinlock::new(VgaWriter::new()));

pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
