use std::cell::UnsafeCell;

use rand::{rngs::StdRng, SeedableRng};

thread_local!(
    pub static  THREAD_RNG_KEY: UnsafeCell<StdRng> = {
        let rng = SeedableRng::seed_from_u64(0);
        UnsafeCell::new(rng)
    }
);

#[inline]
pub fn with<F: FnOnce(&mut StdRng) -> R, R>(f: F) -> R {
    THREAD_RNG_KEY.with(|rng| f(unsafe { &mut *rng.get() }))
}

pub fn reseed(seed: u64) {
    THREAD_RNG_KEY.with(|rng| *unsafe { &mut *rng.get() } = SeedableRng::seed_from_u64(seed));
}

#[test]
fn test() {
    use rand::Rng;

    reseed(0);
    let a = with(|rng| rng.gen::<usize>());
    let b = with(|rng| rng.gen::<usize>());
    reseed(0);
    assert_eq!(with(|rng| rng.gen::<usize>()), a);
    assert_eq!(with(|rng| rng.gen::<usize>()), b);
}
