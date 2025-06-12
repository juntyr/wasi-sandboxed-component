pub mod insecure;
pub mod insecure_seed;
#[expect(clippy::module_inception)]
pub mod random;

use crate::VirtMerged as VirtRandom;
