fn largest(list: &[i32]) -> i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    *largest
}


// generice function that can be used in case of number and characters for same work
fn largest2<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest2(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut result = largest(&number_list);
    println!("The largest number is {}", result);
    result = result - 2;
    print!("{result}\n");
}
