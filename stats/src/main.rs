/* Exercise from https://doc.rust-lang.org/book/ch08-03-hash-maps.html
 * Given a list of integers, use a vector and return the mean (the average value), median (when
 * sorted, the value in the middle position), and mode (the value that occurs most often; a hash
 * map will be helpful here) of the list.
 */

use std::collections::HashMap;

// make a struct, so that we can call the stats functions more elegantly
struct IntegerList {
    vec: Vec<u32>,
}

impl IntegerList {
    fn default() -> IntegerList {
        IntegerList {
            vec: vec![
                123, 4123, 5, 65, 1, 12, 41, 2, 6, 67, 2, 3, 23, 3, 4, 532, 21, 12, 235, 235, 26,
                2634, 6234, 62,
            ],
        }
    }

    fn from_vec(vec: Vec<u32>) -> IntegerList {
        IntegerList { vec: vec }
    }

    fn mean(&self) -> f32 {
        let mut sum = 0;
        for i in self.vec.iter() {
            sum += i;
        }
        sum as f32 / self.vec.len() as f32
    }

    fn sort(&mut self) {
        self.vec.sort()
    }

    fn length(&self) -> usize {
        self.vec.len()
    }

    // Sort, then get middle value
    fn median(&mut self) -> f32 {
        self.sort();
        let vec_length = self.length();
        if vec_length % 2 == 0 {
            // average 2 middle values
            (self.vec[vec_length / 2] + self.vec[vec_length / 2 + 1]) as f32 / 2.0
        } else {
            // return middle value
            self.vec[vec_length / 2] as f32
        }
    }

    // the first u32 is a reference because we use Entry
    fn frequency_map(&self) -> HashMap<&u32, u32> {
        let mut hash = HashMap::new();

        for i in self.vec.iter() {
            // Entry returns reference to the value if the value exists
            // If it doesn't exist, or_insert inserts its value
            let count = hash.entry(i).or_insert(0);
            // since count is a reference, we use * to change its variables value
            *count += 1;
        }
        hash
    }

    // Most common element
    // If there are more than one, i guess just return one of them
    fn mode(&self) -> u32 {
        let frequencies = self.frequency_map();

        let mut max_val = 0;
        let mut max_key = 0;
        for (key, val) in frequencies.iter() {
            // The references and pointers following are a mystery to me
            if val > &max_val {
                max_val = *val;
                max_key = **key;
            }
        }
        max_key
    }
}

fn main() {
    //let mut integer_list: Vec<u8> = Vec::new();
    //let mut integer_list = vec![123,4123,5,65,1,12,41,2,6,67,2,3,23,3,4,532,21,12,235,235,26,2634,6234,62];
    let mut data = IntegerList::from_vec(vec![1, 2, 23, 4, 541, 6, 5, 12, 3, 1, 23, 23]);
    println!("data({}): {:?}", data.length(), data.vec);

    data.sort();
    println!("sorted: {:?}", data.vec);

    println!("mean: {}", data.mean());
    println!("median: {}", data.median());

    let fmap = data.frequency_map();
    println!("frequencies: {:?}", fmap);
    println!("mode: {}", data.mode());
}
