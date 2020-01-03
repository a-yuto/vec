extern crate matrix;
use vec::matrix::*;

fn rev(matrix: &Matrix) -> Result<Matrix,String> {
    match true {
        true  => Ok(iden(2)),
        false => Err("cannot".to_string())    
    }
}


#[test]
fn rev_works() {
    let a: Matrix = Matrix{
        mat: vec![vec![4.0,3.0],
                  vec![2.0,1.0]
        ],
        row: 2,
        col: 2
    };
    matrix_test(&iden(2),&(&rev(&a).unwrap() * &a));    
}
fn main() {
    let b: Matrix = Matrix{
        mat: vec![vec![4.0,3.0],
                  vec![2.0,1.0]
        ],
        row: 2,
        col: 2
    };
    mat_print(&b);
}

