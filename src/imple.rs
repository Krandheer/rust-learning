struct Temperature {
    degrees_f: f64,
}

// fn display_temp(temp: Temperature){
//     println!("the temperature in farenhite is: {:?}", temp.degrees_f);
// }

// when we are doing temperature related things only the why not implement temperature struct
impl Temperature {
    // fn display_temp(temp: Temperature) {
    //     println!("the temperature in farenhite is: {:?}", temp.degrees_f);
    // }
    
    fn display_temp(&self) {
        println!("the temperature in farenhite is: {:?}", self.degrees_f);
    }
}

fn main() {
    let temp = Temperature { degrees_f: 98.9 };
    temp.display_temp();
}
