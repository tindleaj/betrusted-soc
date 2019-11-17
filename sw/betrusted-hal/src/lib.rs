#![no_std]

extern crate alloc;

pub mod hal_i2c;
pub mod hal_time;
pub mod hal_lcd;
pub mod hal_com;
pub mod hal_kbd;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
