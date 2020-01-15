extern crate matrix;
use nearly_eq::*;
/*
 * perceptron
 */

pub fn and(_x1: f64,_x2: f64) -> f64 {
    let (w1,w2,bias) = (0.5,0.5,-0.7);
    let tmp          = _x1 * w1 + _x2 * w2 + bias;
    match tmp >= 0.0 {
        true  => 1.0,
        false => 0.0
    }
}

pub fn nand(_x1: f64,_x2: f64) -> f64 {
    let (w1,w2,bias) = (-0.5,-0.5,0.7);
    let tmp          = _x1 * w1 + _x2 * w2 + bias;
    match tmp >= 0.0 {
        true  => 1.0,
        false => 0.0
    }
}

pub fn or(_x1: f64,_x2: f64) -> f64 {
    let (w1,w2,bias) = (0.5,0.5,-0.2);
    let tmp          = _x1 * w1 + _x2 * w2 + bias;
    match tmp >= 0.0 {
        true  => 1.0,
        false => 0.0
    }
}

pub fn xor(_x1: f64,_x2: f64) -> f64 {
    let s1 = nand(_x1,_x2);
    let s2 = or(_x1,_x2);
    and(s1,s2)
}
#[cfg(test)]
mod perceptoron_test {
    use super::*;

    #[test]
    fn and_test(){
        assert_eq!(and(0.0,0.0) ,0.0);
        assert_eq!(and(1.0,0.0) ,0.0);
        assert_eq!(and(0.0,1.0) ,0.0);
        assert_eq!(and(1.0,1.0), 1.0);
    }
    #[test]
    fn nand_test(){
        assert_eq!(nand(0.0,0.0) ,1.0);
        assert_eq!(nand(1.0,0.0) ,1.0);
        assert_eq!(nand(0.0,1.0) ,1.0);
        assert_eq!(nand(1.0,1.0), 0.0);
    }
    #[test]
    fn or_test(){
        assert_eq!(or(0.0,0.0) ,0.0);
        assert_eq!(or(1.0,0.0) ,1.0);
        assert_eq!(or(0.0,1.0) ,1.0);
        assert_eq!(or(1.0,1.0), 1.0);
    }
    #[test]
    fn xor_test(){
        assert_eq!(xor(0.0,0.0) ,0.0);
        assert_eq!(xor(1.0,0.0) ,1.0);
        assert_eq!(xor(0.0,1.0) ,1.0);
        assert_eq!(xor(1.0,1.0), 0.0);
    }

}
/*
 * activity function
 */
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
pub fn step(x: f64) -> f64 {
    match x > 0.0 {
        true  => 1.0,
        false => 0.0
    }
}
fn max(x: f64,y: f64) -> f64 {
    match x > y {
        true  => x,
        false => y
    }
}
pub fn ReLU(x: f64) -> f64 {
    max(0.0,x)    
}
#[cfg(test)]
mod activity_function_tests {
    use super::*;
    
    #[test]
    fn sigmoid_work(){
        println!("f-1 = {}",sigmoid(-1.0));
        println!("f1  = {}",sigmoid(1.0));
        println!("f2  = {}",sigmoid(2.0));
        assert_nearly_eq!(sigmoid(-1.0),0.26894142137);
    }
    #[test]
    fn step_works(){
         assert_eq!(step(0.0),0.0);
         assert_eq!(step(5.0),1.0);
         assert_eq!(step(-100.0),0.0);
    }
    #[test]
    fn ReLU_work(){
         assert_eq!(ReLU(10.0),10.0);
         assert_eq!(ReLU(0.0),0.0);
         assert_eq!(ReLU(-100.0),0.0);
    }
}
