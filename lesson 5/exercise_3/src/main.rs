
pub trait HasArea {        //定义hasarea方法用来计算面积
    type Output;           //type is used to declare an associated type
    fn get_area(&self) -> Self::Output; //定义get_area函数，计算面积
}
struct Square<T> {     //正方形结构体
    side: T,           //边长
}
struct Circle<T> {     //圆形结构体
    radius: T,         //圆的半径
}
struct Triangle<T> {    //三角形结构体
    base: T,             //三角形的底边长
    hight: T,            //三角形的高
}
impl<T: std::ops::Mul<Output = T> + Copy> HasArea for Square<T> {  //正方形泛型结构体实现HasArea，同时实现Mul+Copy trait的类型才能够计算
    type Output = T;                //正方形Output类型定义为泛型T，即接收任何满足泛型约束的类型；
    
    fn get_area(&self) -> Self::Output {   //计算面积，返回output
        self.side * self.side       //计算公式
    }
}
impl<T: std::ops::Mul<Output = T> + Into<f64> + Copy> HasArea for Circle<T> {  //圆形泛型结构体实现HasArea，同时实现Mul+Copy trait的类型才能够计算
    type Output = f64;                   //只能接受f64类型的输出
    
    fn get_area(&self) -> Self::Output {
        (self.radius * self.radius).into() * std::f64::consts::PI  //计算圆形面积时需要将泛型T转换为f64浮点数
    }
}
impl<T: std::ops::Mul<Output = T> + Into<f64> + Copy> HasArea for Triangle<T> {  //三角形泛型结构体实现HasArea
    type Output = f64;  //只能接受f64类型的输出
    
    fn get_area(&self) -> Self::Output {
        (self.base * self.hight).into() * 0.5  //计算三角形的面积
    }
}
fn main() {

    let s = Square {side: 10};      //定义一个正方形结构体
    println!("square: {}", s.get_area());  //打印输出正方形的面积
    
    let r = Circle {radius: 2};       //定义了一个圆形结构体
    println!("circle: {}", r.get_area());  //打印输出了圆形的面积

    let t = Triangle {base: 2, hight: 1};  //定义了一个三角形
    println!("Triangle: {}", t.get_area()); //打印输出了三角形的面积
}