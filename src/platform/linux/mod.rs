#[cfg(any(
    feature = "linux-evdev",
    not(any(feature = "linux-evdev", feature = "linux-mousedev"))
))]
pub mod evdev;
#[cfg(any(
    feature = "linux-evdev",
    not(any(feature = "linux-evdev", feature = "linux-mousedev"))
))]
pub use evdev::*;

#[cfg(feature = "linux-mousedev")]
pub mod mousedev;
#[cfg(feature = "linux-mousedev")]
pub use mousedev::*;
