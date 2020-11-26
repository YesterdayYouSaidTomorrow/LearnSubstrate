enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait LightTime {
    fn lighttime(&self);
}

impl LightTime for TrafficLight {
    fn lighttime(&self) {
        match self {
            TrafficLight::Red=> {
                println!("If traffic light is red,it will 10 seconds") 
            },
            TrafficLight::Yellow => {
                println!("If traffic light is yellow, it will 20 seconds")
            },
            TrafficLight::Green => {
                println!("If traffic light is green, it will 30 seconds")
            },
        }
    }
} 

fn main() {

    let G = TrafficLight::Green;
    let Y = TrafficLight::Yellow;
    let R = TrafficLight::Red;

    G.lighttime();
    Y.lighttime();
    R.lighttime();
}
