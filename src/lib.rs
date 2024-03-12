use nohash_hasher::IntMap;
use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hash, Hasher},
};
use uuid::Uuid;

pub fn log_message(message: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&message.into());

    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", message);
}

fn get_time() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        // WebAssembly-specific implementation using `web-sys`
        use web_sys::window;
        window().unwrap().performance().unwrap().now() / 1000.0
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Native implementation using std::time
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }
}

#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		$crate::log_message(&format!($($arg)*));
	}
}

const ITERATIONS: usize = 1000;
const DATA_SIZE: usize = 10000;

fn run_test(name: &str, f: impl Fn()) {
    let start = get_time();
    for _ in 0..ITERATIONS {
        f();
    }
    log!("{} time: {}", name, get_time() - start);
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct U128Wrap(u128);

impl From<U128Wrap> for u128 {
    fn from(value: U128Wrap) -> u128 {
        value.0
    }
}

impl From<u128> for U128Wrap {
    fn from(value: u128) -> U128Wrap {
        U128Wrap(value)
    }
}

impl nohash_hasher::IsEnabled for U128Wrap {}
impl Hash for U128Wrap {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_u64((self.0 >> 64) as u64 ^ self.0 as u64);
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct U128Wrap2(u128);

#[derive(Default)]
struct U128Wrap2Hash(u64);

impl From<U128Wrap2> for u128 {
    fn from(value: U128Wrap2) -> u128 {
        value.0
    }
}

impl From<u128> for U128Wrap2 {
    fn from(value: u128) -> U128Wrap2 {
        U128Wrap2(value)
    }
}

impl Hash for U128Wrap2 {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u64((self.0 >> 64) as u64 ^ self.0 as u64);
    }
}

impl Hasher for U128Wrap2Hash {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("wrong usage of U128Wrap2Hash");
    }

    fn write_u64(&mut self, u: u64) {
        self.0 = u;
    }
}

pub fn run() {
    let data = (0..DATA_SIZE)
        .map(|index| (Uuid::new_v4(), index))
        .collect::<Vec<_>>();

    run_test("uuid default hash", || {
        let map: HashMap<Uuid, usize> = data.iter().cloned().collect();
        for (key, value) in &data {
            assert_eq!(map.get(key), Some(value));
        }
    });

    let map: HashMap<Uuid, usize> = data.iter().cloned().collect();
    run_test("uuid default hash, no write", || {
        for (key, value) in &data {
            assert_eq!(map.get(key), Some(value));
        }
    });

    run_test("u128 default hash", || {
        let map: HashMap<u128, usize> = data.iter().map(|it| (it.0.as_u128(), it.1)).collect();
        for (key, value) in &data {
            assert_eq!(map.get(&key.as_u128()), Some(value));
        }
    });

    let map: HashMap<u128, usize> = data.iter().map(|it| (it.0.as_u128(), it.1)).collect();
    run_test("u128 default hash, no write", || {
        for (key, value) in &data {
            assert_eq!(map.get(&key.as_u128()), Some(value));
        }
    });

    run_test("u128 xor hash with nohash_hasher", || {
        let map: IntMap<U128Wrap, usize> = data
            .iter()
            .map(|it| (it.0.as_u128().into(), it.1))
            .collect();
        for (key, value) in &data {
            assert_eq!(map.get(&key.as_u128().into()), Some(value));
        }
    });

    let map: IntMap<U128Wrap, usize> = data
        .iter()
        .map(|it| (it.0.as_u128().into(), it.1))
        .collect();
    run_test("u128 xor hash with nohash_hasher, no write", || {
        for (key, value) in &data {
            assert_eq!(map.get(&key.as_u128().into()), Some(value));
        }
    });

    run_test("u128 xor hash manual impl", || {
        let map: HashMap<U128Wrap2, usize, BuildHasherDefault<U128Wrap2Hash>> = data
            .iter()
            .map(|it| (it.0.as_u128().into(), it.1))
            .collect();
        for (key, value) in &data {
            assert_eq!(map.get(&key.as_u128().into()), Some(value));
        }
    });

    let map: HashMap<U128Wrap2, usize, BuildHasherDefault<U128Wrap2Hash>> = data
        .iter()
        .map(|it| (it.0.as_u128().into(), it.1))
        .collect();
    run_test("u128 xor hash manual impl, no write", || {
        for (key, value) in &data {
            assert_eq!(map.get(&key.as_u128().into()), Some(value));
        }
    });
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn run() {
        console_error_panic_hook::set_once();

        super::run()
    }
}
