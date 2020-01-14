extern crate matrix;
use vec::matrix::*;

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
    fn sigmoid_work(){
        assert_eq!(sigmoid(-1),0.26894142137);
    }
    fn step_works(){
         assert_eq!(step(0.0),0.0);
         assert_eq!(step(5.0),1.0);
         assert_eq!(step(-100.0),0.0);
    }
}
