/* Exercise from https://doc.rust-lang.org/book/ch08-03-hash-maps.html
 * Given a list of integers, use a vector and return the mean (the average value), median (when
 * sorted, the value in the middle position), and mode (the value that occurs most often; a hash
 * map will be helpful here) of the list.
 */

// make a struct, so that we can call the stats functions more elegantly
struct IntegerList {
    vec: Vec<u32>,
}

impl IntegerList {
    fn default() -> IntegerList {
        IntegerList { vec: vec![123,4123,5,65,1,12,41,2,6,67,2,3,23,3,4,532,21,12,235,235,26,2634,6234,62] }
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
        if vec_length % 2 == 0 {  // average 2 middle values
            (self.vec[vec_length / 2] + self.vec[vec_length / 2 + 1]) as f32 / 2.0
        } else {  // return middle value
            self.vec[vec_length / 2] as f32
        }
    }

    // Most common element
    // If there are more than one, i guess just return one of them
    fn mode(&self) -> u32 {
        2
    }
}

fn main() {
    //let mut integer_list: Vec<u8> = Vec::new();
    //let mut integer_list = vec![123,4123,5,65,1,12,41,2,6,67,2,3,23,3,4,532,21,12,235,235,26,2634,6234,62];
    let mut data = IntegerList::from_vec(vec![1,2,23,4,541,6,5,12,3, 1]);
    println!("data({}): {:?}", data.length(), data.vec);
    data.sort();
    println!("sorted: {:?}", data.vec);
    println!("mean: {}", data.mean());
    println!("median: {}", data.median());
    println!("mode: {}", data.mode());
}