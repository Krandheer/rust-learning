struct Temperature {
    degrees_f: f64,
}

fn display_temp(temp: Temperature){
    println!("the temperature in farenhite is: {:?}", temp.degrees_f);
}

fn main() {
    let temp = Temperature{degrees_f: 98.9};
    display_temp(temp);
}
