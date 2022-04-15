/*
This code will build a TrafficLight enum of three kinds of traffic lights. each one will take a parameter of the duration. A funtion will be build to return the durtaion time from the enum.
*/

// define the enum
enum TrafficLight {
    Red(u8),
    Green(u8),
    Yellow(u8),
}

pub trait Duration {
    fn duration(&self) {}
}
// set the function to the enum
impl Duration for TrafficLight {
    fn duration(&self) {
        match self {
            TrafficLight::Red(time) => {
                println!("The red light will last for {} seconds.", time)
            }
            TrafficLight::Yellow(time) => {
                println!("The yellow light will last for {} seconds.", time)
            }
            TrafficLight::Green(time) => {
                println!("The green light will last for {} seconds.", time)
            }
        }
    }
}
// call the function
fn main() {
    let light_red = TrafficLight::Red(60);
    let light_yellow = TrafficLight::Yellow(5);
    let light_green = TrafficLight::Green(90);
    light_red.duration();
    light_yellow.duration();
    light_green.duration();
}
