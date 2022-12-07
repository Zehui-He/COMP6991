use std::{collections::HashMap, sync::Arc};
use std::sync::RwLock;
use exam_q6_lib::{get_factors, get_common_factors};

fn main() {
    let args: Vec<u128> = std::env::args().skip(1).map(|x| x.parse().unwrap()).collect();

    let factors: Arc<RwLock<HashMap<u128, Vec<u128>>>> = Arc::new(RwLock::new(HashMap::new()));
    std::thread::scope(|s| {
        for arg in args {
            let factors = factors.clone();
            s.spawn(move || {
                let fact = get_factors(arg);
                factors.write().unwrap().insert(arg, fact);
            });
        }
    });

    let common_factors: Arc<RwLock<HashMap<(u128, u128), Vec<u128>>>> = Arc::new(RwLock::new(HashMap::new()));
    std::thread::scope(|s| {
        for (val1, facs1) in factors.read().unwrap().iter() {
            for (val2, facs2) in factors.read().unwrap().iter() {
                if val1 > val2 {
                    let common_factors = common_factors.clone();
                    s.spawn(move || {
                        common_factors.write().unwrap().insert((*val1, *val2), get_common_factors(&facs1, &facs2));
                    });
                }
            }
        }
    });

    let binding = common_factors.read().unwrap();
    let mut keys = binding.keys().collect::<Vec<_>>();
    keys.sort();

    for k in keys {
        println!("{k:?}: {:?}", common_factors.read().unwrap()[k]);
    }
}
