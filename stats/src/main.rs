/* Exercise from https://doc.rust-lang.org/book/ch08-03-hash-maps.html
 * Given a list of integers, use a vector and return the mean (the average value), median (when
 * sorted, the value in the middle position), and mode (the value that occurs most often; a hash
 * map will be helpful here) of the list.
 */

use std::iter::Sum;

// make a struct, so that we can call the stats functions more elegantly
struct IntegerList {
    vec: Vec<u32>,
}

impl IntegerList {
    fn default() -> IntegerList {
        IntegerList { vec: vec![123,4123,5,65,1,12,41,2,6,67,2,3,23,3,4,532,21,12,235,235,26,2634,6234,62] }
    }

    fn mean(&self) -> f32 {
        let mut sum = 0;

        for i in self.vec.iter() {
            sum += i;
        }

        sum as f32 / self.vec.len() as f32
    }
}

fn main() {
    //let mut integer_list: Vec<u8> = Vec::new();
    //let mut integer_list = vec![123,4123,5,65,1,12,41,2,6,67,2,3,23,3,4,532,21,12,235,235,26,2634,6234,62];
    let data = IntegerList::default();
    println!("mean: {}", data.mean());
}