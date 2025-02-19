#[cfg(feature = "no_std")]
pub mod no_std;

#[cfg(not(feature = "no_std"))]
pub mod std;