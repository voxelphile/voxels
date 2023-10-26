#![feature(int_roundings)]
use std::ops::Rem;

mod tests;

#[derive(Default, Clone)]
struct Palcomp {
    palettes: Vec<u64>,
    data: Vec<u64>,
    len: usize,
}

const U64_BITS: usize = 64;

impl Palcomp {
    fn compress(raw: &[u64]) -> Self {
        let mut palettes = vec![];
        for &id in raw {
            let Err(pos) = palettes.binary_search_by(|probe: &u64| probe.cmp(&id)) else {
                continue;
            };
            palettes.insert(pos, id);
        }

        let bits = (palettes.len() as f32).log2().ceil() as usize;

        let mut cursor = 0;
        let mut index = 0;

        let mut data = vec![0u64; (raw.len() * bits).div_ceil(U64_BITS)];

        for &id in raw {
            let Ok(pos) = palettes.binary_search_by(|probe: &u64| probe.cmp(&id)) else {
                panic!("palette id not found");
            };
            let pos = pos as u64;

            let outer = cursor / U64_BITS;
            let inner = cursor % U64_BITS;

            data[outer] |= pos << inner;
            if inner + bits > U64_BITS {
                data[outer + 1] |= pos >> (U64_BITS - inner) as u64;
            }

            cursor += bits;
            index += 1;
        }

        Self {
            palettes,
            data,
            len: index,
        }
    }

    fn decompress(&self) -> Vec<u64> {
        let bits = (self.palettes.len() as f32).log2().ceil() as usize;
        let mask = ((1 << bits) - 1) as u64;

        let mut cursor = 0;
        let mut index = 0;

        let mut data = vec![];

        while index < self.len {
            let outer = cursor / U64_BITS;
            let inner = cursor % U64_BITS;

            let mut id = 0u64;
            id |= (self.data[outer] >> inner) & mask;
            if inner + bits > U64_BITS {
                id |= ((self.data[outer + 1] as u64) & (mask >> (U64_BITS - inner)))
                    << (U64_BITS - inner);
            }
            data.push(self.palettes[id as usize]);

            cursor += bits;
            index += 1;
        }
        
        data
    }
}

#[derive(Default, Clone)]
pub struct Channel {
    data: Palcomp,
}

impl Channel {
    pub fn extend<IntoIter: IntoIterator<Item = u64>>(&mut self, iter: IntoIter) {
        let mut raw = self.data.decompress();
        raw.extend(iter);
        self.data = Palcomp::compress(&raw)
    }

    pub fn into_iter(&self) -> impl Iterator<Item = u64> {
        self.data.decompress().into_iter()
    }

    pub fn get<IntoIter: IntoIterator<Item = usize>>(&self, iter: IntoIter) -> Vec<u64> {
        let raw = self.data.decompress();
        let mut data = vec![];
        for index in iter {
            data.push(*raw.get(index).expect("could not find data at index"))
        }
        data
    }

    pub fn set<IntoIter: IntoIterator<Item = (usize, u64)>>(&mut self, iter: IntoIter) {
        let mut raw = self.data.decompress();
        for (index, data) in iter {
            *raw.get_mut(index).expect("could not find data at index") = data;
        }
        self.data = Palcomp::compress(&raw);
    }
}
