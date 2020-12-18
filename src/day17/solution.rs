use num::range;
use std::collections::HashMap;

/// Since we have a n dimensions we do not want to fix dimensions in stuff like x,y,z but rather
/// in infinite dimensions stored into [IndexType]
#[derive(Debug)]
struct IndexType(Vec::<i64>);

// We want to use the [IndexType] as a String index
//
// returns format as `x.y.z´
impl ToString for IndexType {
    fn to_string(&self) -> String {

        // We optimize this call a bit by some const transformations for 3 and 4 dims
        let len = self.0.len();
        if len == 3 {
            return format!("{}.{}.{}", &self.0[0], &self.0[1], &self.0[2]);
        } else if len == 4 {
            return format!("{}.{}.{}.{}", &self.0[0], &self.0[1], &self.0[2], &self.0[3]);
        }

        self.0.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(".")
        /* Iterative variant
        let mut s = String::new();
        for i in &self.0 {
            s += &format!("{}.", i);
        }

        s.remove(s.len() - 1);
        s*/
    }
}


/// We want to create a String in form of `1.4.2´ form [vec![1,4,2]]
impl From<String> for IndexType {
    fn from(s: String) -> Self {
        Self (s.split(".").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>())
    }
}

/// Stores our current field
/// where each field is a mapping in form of `2.2.3´ => true|false
struct Field {
    field: HashMap<String, bool>
}


/// Some functions are implemented for a 3d and a 4d field alike
/// however, they could be adapted to work with a nd field, if you are keen enough
impl Field {
    fn new() -> Self {
        return Field {
            field: HashMap::new()
        }
    }

    fn get_field_value(&self, index: &IndexType) -> bool {
        let index = index.to_string();
        if !self.field.contains_key(&index) {
            return false;
        }

        return *self.field.get(&index).unwrap()
    }

    fn set_field_value(&mut self, index: &IndexType, value: bool) {
        let index = index.to_string();
        self.field.insert(index, value);
    }

    /// Returns (#active, #inactive) neighbours
    fn get_neighbours_3d(&self, index: &IndexType) -> (u64, u64) {
        let index_vec = &index.0;

        let mut n_active = 0 as u64;
        let mut n_inactive = 0 as u64;
        for x in range(index_vec[0] - 1, index_vec[0] + 2) {
            for y in range(index_vec[1] - 1, index_vec[1] + 2) {
                for z in range(index_vec[2] - 1, index_vec[2] + 2) {
                    let val = self.get_field_value(&IndexType(vec!(x, y, z)));
                   if val {
                       n_active += 1;
                   } else {
                       n_inactive += 1;
                   };
                }
            }
        }

        // Remove own field from the counts
        if self.get_field_value(index) {
            n_active -= 1;
        } else {
            n_inactive -= 1;
        }

        return (n_active, n_inactive);
    }

    /// Returns (#active, #inactive) neighbours
    fn get_neighbours_4d(&self, index: &IndexType) -> (u64, u64) {
        let index_vec = &index.0;

        let mut n_active = 0 as u64;
        let mut n_inactive = 0 as u64;
        for x in range(index_vec[0] - 1, index_vec[0] + 2) {
            for y in range(index_vec[1] - 1, index_vec[1] + 2) {
                for z in range(index_vec[2] - 1, index_vec[2] + 2) {
                    for d in range(index_vec[3] - 1, index_vec[3] + 2) {
                        let val = self.get_field_value(&IndexType(vec!(x, y, z, d)));
                        if val {
                            n_active += 1;
                        } else {
                            n_inactive += 1;
                        };
                    }
                }
            }
        }

        // Remove own field from the counts
        if self.get_field_value(index) {
            n_active -= 1;
        } else {
            n_inactive -= 1;
        }

        return (n_active, n_inactive);
    }

    /// Small helper function to print a part of the field
    fn print_field_3d(&self) {
        for z in range(0, 3) {
            for y in range(-5, 5) {
                for x in range(-5, 5) {
                    let val = self.get_field_value(&IndexType(vec![x as i64, y as i64, z as i64]));
                    if val {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
                println!();
            }
            println!();
        }
    }

    /// Will get the amount of dimension (3d or 4d here)
    ///
    /// Its done by counting the amount of fields in one of the keys of the [field]
    fn get_dimensions(&self) -> usize {
        if self.field.is_empty() { return 0; }
        (IndexType::from(self.field.keys().next().unwrap().clone()).0.len())
    }

    /// Gets the current minima and maxima for each dimension
    ///
    /// If you want to save time, you could just remember them when creating / removing outer stars
    fn get_extends(&self) -> (IndexType, IndexType) {
        let n_dims = self.get_dimensions();
        let mut from = vec![i64::max_value(); n_dims];
        let mut to = vec![i64::min_value(); n_dims];

        for point_str in self.field.keys().into_iter() {
            for (i, value) in (IndexType::from(point_str.clone())).0.iter().enumerate() {
                if from[i] > *value {
                    from[i] = *value;
                } else if to[i] < *value{
                    to[i] = *value;
                }
            }
        }

        return (IndexType(from), IndexType(to));
    }

    fn step3d(&mut self) {
        let mut new_field = HashMap::new();
        let (from, to) = self.get_extends();
        for x in range(from.0.get(0).unwrap() - 1, to.0.get(0).unwrap() + 2) {
            for y in range(from.0.get(1).unwrap() - 1, to.0.get(1).unwrap() + 2) {
                for z in range(from.0.get(2).unwrap() - 1, to.0.get(2).unwrap() + 2) {
                    let val = self.get_field_value(&IndexType(vec!(x,y,z)));
                    let index = IndexType(vec![x,y,z]);
                    let (active, _) = self.get_neighbours_3d(&index);

                    let mut new_val = true;
                    if val {
                        if !(active == 2 || active == 3) {
                            new_val = false;
                        }
                    } else {
                        if !(active == 3) {
                            new_val = false;
                        }
                    }

                    if new_val {
                        new_field.insert(index.to_string(), new_val);
                    }
                }
            }
        }
        self.field = new_field;
    }

    fn step4d(&mut self) {
        let mut new_field = HashMap::new();
        let (from, to) = self.get_extends();
        for x in range(from.0.get(0).unwrap() - 1, to.0.get(0).unwrap() + 2) {
            for y in range(from.0.get(1).unwrap() - 1, to.0.get(1).unwrap() + 2) {
                for z in range(from.0.get(2).unwrap() - 1, to.0.get(2).unwrap() + 2) {
                    for d in range(from.0.get(3).unwrap() - 1, to.0.get(3).unwrap() + 2) {
                        let index = IndexType(vec![x, y, z, d]);
                        let val = self.get_field_value(&index);
                        let (active, _) = self.get_neighbours_4d(&index);
                        let mut new_val = true;

                        if val {
                            if !(active == 2 || active == 3) {
                                new_val = false;
                            }
                        } else {
                            if !(active == 3) {
                                new_val = false;
                            }
                        }
                        if new_val {
                            new_field.insert(index.to_string(), new_val);
                        }
                    }
                }
            }
        }
        self.field = new_field;
    }

    pub fn count_active_blocks(&self) -> u64 {
        self.field.values().filter(|f| **f).map(|b| {
            if *b {
                return 1 as u64;
            }
            0 as u64
        }).sum::<u64>()
    }

}

pub fn task1() {
    let input = get_input_data();
    let mut field = parse_start_field(input, false);
    field.step3d();
    // field.print_field_3d();
    field.step3d();
    field.step3d();
    field.step3d();
    field.step3d();
    field.step3d();

    let sum_of_active = field.count_active_blocks();
    println!("Sum of actives (Task 1) {}", sum_of_active);
}

pub fn task2() {
    let input = get_input_data();
    let mut field = parse_start_field(input, true);
    field.step4d();
    field.step4d();
    field.step4d();
    field.step4d();
    field.step4d();
    field.step4d();

    let sum_of_active = field.count_active_blocks();

    println!("Sum of actives (Task 2) {}", sum_of_active);

}

/// Reads the first layer of data
///
/// the game can be started in 3d or 4d mode the read input will represent
/// exactly one slice of the dimensions
fn parse_start_field(input: &str, generate_4d_field: bool) -> Field {
    let mut field = Field::new();

    for (col, line) in input.split("\r\n").enumerate() {
        for (row, value) in line.chars().into_iter().enumerate() {
            let val: bool = match value {
                '.' => false,
                '#' => true,
                _ => panic!("Invalid char found")
            };

            if !generate_4d_field {
                // 3d field
                field.set_field_value(&IndexType(vec![row as i64, col as i64, 1 as i64]), val);
            } else{
                // 4d field
                field.set_field_value(&IndexType(vec![row as i64, col as i64, 1 as i64, 1 as i64]), val);

            }
        }
    }

    field
}


/// Reads the file into the binary
fn get_input_data() -> &'static str {
    return include_str!("input.txt");
}