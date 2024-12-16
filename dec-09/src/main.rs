fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let s = s.trim();
    let mut data = Data::new(s);
    data.compress();

    println!("Part 1 checksum: {}", data.checksum());

    data.compress_new();
    println!("Part 2 checksum: {}", data.checksum_new());
}

struct Data {
    original: Vec<i64>,
    layout: Vec<i64>,
    space_ind: Vec<usize>,
    files: Vec<usize>,
    spaces: Vec<i64>,
    compressed: Vec<i64>,
    compressed_new: Vec<i64>,
}

impl Data {
    fn new(s: &str) -> Data {
        let original: Vec<i64> = s
            .chars()
            .map(|c| -> i64 { c.to_digit(10).unwrap() as i64 })
            .collect();
        let mut layout = Vec::new();
        let mut space_ind = Vec::new();
        let mut files = Vec::new();
        let mut spaces = Vec::new();

        for (id, ind) in (0..original.len()).step_by(2).enumerate() {
            let ent = original[ind];
            files.push(ent as usize);
            for _ in 0..ent {
                layout.push(id as i64);
            }
            let layout_len = layout.len();
            if ind + 1 < original.len() {
                let sp = original[ind + 1];
                spaces.push(sp);
                for it in 0..sp {
                    layout.push(-1);
                    space_ind.push(it as usize + layout_len);
                }
            }
        }

        Data {
            original,
            layout,
            space_ind,
            files,
            spaces,
            compressed: Vec::new(),
            compressed_new: Vec::new(),
        }
    }

    fn checksum(&self) -> u64 {
        let mut chsum = 0;
        for (ind, ent) in self.compressed.iter().enumerate() {
            chsum += (ind as u64) * (*ent as u64);
        }
        chsum
    }

    fn checksum_new(&self) -> u64 {
        let mut chsum = 0;
        for (ind, ent) in self.compressed_new.iter().enumerate() {
            chsum += (ind as u64) * (*ent as u64);
        }
        chsum
    }

    fn compress(&mut self) {
        let mut compressed = self.layout.clone();
        let mut entries_rev = self.layout.iter().rev();
        for ind in &self.space_ind {
            let mut ent = *entries_rev.next().unwrap();
            while ent == -1 {
                ent = *entries_rev.next().unwrap();
            }
            compressed[*ind] = ent;
        }
        let chop_index = self.layout.len() - self.space_ind.len();
        self.compressed = compressed[0..chop_index].to_vec();
    }

    fn compress_new(&mut self) {
        let mut compressed_new = self.layout.clone();
        for (fl, sz) in self.files.iter().enumerate().rev() {
            // find the first index of the file in the original layout
            let fl_ind = compressed_new.iter().position(|x| *x == fl as i64).unwrap() + 1;
            // find out where we have a least sz -1 in a row in compressed_new
            let mut ind = 0;
            let mut count = 0;
            for (i, ent) in compressed_new.iter().take(fl_ind).enumerate() {
                if *ent == -1 {
                    if count == 0 {
                        ind = i;
                    }
                    count += 1;
                } else {
                    count = 0;
                    ind = 0;
                    continue;
                }

                if count == *sz {
                    break;
                };
            }
            if ind > 0 {
                // check that the file is not already to the left of this space, if it is, break
                // first occurance of fl:
                compressed_new = compressed_new.iter().map(|x| if *x == fl as i64 { 0 } else { *x }).collect();
                for i in compressed_new.iter_mut().skip(ind).take(*sz) {
                    *i = fl as i64;
                }
            }
        }

        compressed_new = compressed_new.iter().map(|x| if *x == -1 { 0 } else { *x }).collect();
        self.compressed_new = compressed_new;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "2333133121414131402";
    const TEST_DATA2: &str = "233313312141413140213";

    #[test]
    fn test_data_new() {
        let data = Data::new("23124");
        assert_eq!(data.original, vec![2, 3, 1, 2, 4]);
        assert_eq!(data.layout, vec![0, 0, -1, -1, -1, 1, -1, -1, 2, 2, 2, 2]);
        assert_eq!(data.space_ind, vec![2, 3, 4, 6, 7]);
        assert_eq!(data.compressed, Vec::new());
    }

    #[test]
    fn test_p1() {
        let mut data = Data::new(TEST_DATA);
        data.compress();
        assert_eq!(data.checksum(), 1928);
    }

    #[test]
    fn test_p2() {
        let mut data = Data::new(TEST_DATA);
        data.compress_new();
        assert_eq!(data.checksum_new(), 2858);
    }

    fn test_p2_data2() {
        let mut d = Data::new(TEST_DATA);
        d.compress_new();
        let mut data = Data::new(TEST_DATA2);
        data.compress_new();

        println!("d1l: {:?}", d.layout);
        println!("d2l: {:?}", data.layout);
        println!("d1: {:?}", d.compressed_new);
        println!("d2: {:?}", data.compressed_new);
        println!("len d1: {:?}", d.compressed_new.len());
        println!("len d2: {:?}", data.compressed_new.len());
        assert!(false);
    }
}
