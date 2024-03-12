fn main() {
    let char_list = vec!['a', 'd', 'g'];
    let num_list = vec![2, 564, 78, 6, 23];

    let largest = get_largest(char_list);

    println!("The largest number is {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}
