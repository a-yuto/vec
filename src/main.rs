extern crate matrix;
use vec::matrix::*;

fn main() {
    let a = iden(2);
    let b = iden(2);
    let plus  = add_mat(&a,&b).unwrap();
}

