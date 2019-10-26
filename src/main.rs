extern crate vec;
use vec::Opp;

fn main() {
    let _x = vec![1.0,5.0];
    let _y = vec![3.0,4.0];
    let _k = 7.0;
    let _z = Opp::add(&_x,&_y);
    let _zz = Opp::scl_mul(&_k,&_x);
    println!("{:?}",&_z);
    println!("{:?}",&_zz);
}
