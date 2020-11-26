enum TrafficLight {     //定义一个简单枚举TrafficLight
    Red,
    Yellow,
    Green,
}

pub trait LightTime {    //定义一个LightTime trait,包含lighttime方法
    fn lighttime(&self);   //引用self的值
}

impl LightTime for TrafficLight {  
    fn lighttime(&self) {       
        match self {          //匹配self
            TrafficLight::Red=> {   
                println!("If traffic light is red,it will 10 seconds")   //如果是红灯，
            },
            TrafficLight::Yellow => {
                println!("If traffic light is yellow, it will 20 seconds")  //如果是黄灯
            },
            TrafficLight::Green => {
                println!("If traffic light is green, it will 30 seconds") //如果是绿灯
            },
        }
    }
} 

fn main() {

    let G = TrafficLight::Green;   //定义TrafficLight类型的变量
    let Y = TrafficLight::Yellow;
    let R = TrafficLight::Red;

    G.lighttime();   //调用变量的方法，显示时间
    Y.lighttime();
    R.lighttime();
}
