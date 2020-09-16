//! Input devices include keyboard, mouse, etc.
//!
pub mod event;
pub use event::*;

pub mod platform;
pub use platform::*;

#[cfg(test)]
mod tests {

    #[test]
    fn num() {
        let a = 0xFFFF_FFFDu32;
        let b1 = (a & 0x0000_FFFFu32) as i16;
        let b2 = ((a >> 16) & 0x0000_FFFFu32) as i16;
        println!("b1={}, b2={}", b1, b2);
    }
}
