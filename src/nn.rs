extern crate matrix;
use vec::matrix::*;

/*
 * perceptron
 */
pub fn and(_x1: f64,_x2: f64) -> f64 {
    let w1,w2,theta = (0.5,0.5,0.7);
    let tmp         = _x1 * w1 + _x2 * w2;
    match tmp <= theta {
        true  => 0.0,
        false => 1.0
    }
}


#[cfg(test)]
mod perceptoron_test() {
    //(w1,w2,theta) = (0.5,0.5,0.7)
    #[test]
    fn and_test(){
        assert_eq!(and(0.0,0.0) ,0.0);
        assert_eq!(and(1.0,0.0) ,0.0);
        assert_eq!(and(0.0,1.0) ,0.0);
        assert_eq!(and(1.0,1.0), 1.0);
    }
}
/*
 * activity function
 */
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + x.exp())    
}
pub fn step(x: f64) -> f64 {
    match x > 0 {
        true  => 1.0,
        false => 0.0
    }
}

#[cfg(test)]
mod activity_function_tests {
    #[test]
    fn sigmoid_work(){
        assert_eq!(sigmoid(-1),0.26894142137);
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
         assert_eq!(ReLu(0.0)),0.0);
         assert_eq!(ReLU(-100.0),0.0);
    }
}
