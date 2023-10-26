use std::collections::HashSet;

use crate::Palcomp;

#[test]
fn accuracy() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut random_data = vec![];
    let mut previous = HashSet::new();
    for i in 0..rng.gen_range::<usize, _>(1000..2000) {
        if rng.gen_bool(0.6) && i != 0 {
            random_data.push(
                *previous
                    .iter()
                    .nth(rng.gen_range::<usize, _>(0..previous.len()))
                    .unwrap(),
            );
        } else {
            let data = rng.gen::<u64>();
            previous.insert(data);
            random_data.push(data);
        }
    }

    let palcomp = Palcomp::compress(&random_data);

    assert_eq!(palcomp.decompress(), random_data);
}
