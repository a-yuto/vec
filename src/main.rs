extern crate matrix;
use vec::matrix::*;





pub fn div_for_row(mat: &Matrix,_row: usize,_scl: f64) -> Matrix {
    let tmp = &(&get_row(_row,&mat).unwrap() * &(1.0 / _scl)).mat[0];
    let mut ans = mat.clone();
    ans.mat[_row] = tmp.to_vec();
    ans
}

pub fn basic_deform_2_row(_mat :&Matrix, scl: f64, col: usize, coled: usize) -> Matrix {
    let mut tmp      = _mat.clone();
    let changed_row  = &(&get_row(col,&_mat).unwrap() * &scl) + &get_row(coled,&_mat).unwrap();
    tmp.mat[coled]  = changed_row.mat[0].clone();
    tmp
}

#[test]
fn div_for_row_works() {
    let _a = Matrix{
        mat: vec![vec![ 8.0,16.0,24.0,32.0],
                  vec![ 2.0, 7.0,12.0,17.0],
                  vec![ 6.0,17.0,32.0,59.0],
                  vec![ 7.0,22.0,46.0,105.0]],
        row: 4,
        col: 4
    };
    let _b = Matrix{
        mat: vec![vec![ 1.0, 2.0, 3.0, 4.0],
                  vec![ 2.0, 7.0,12.0,17.0],
                  vec![ 6.0,17.0,32.0,59.0],
                  vec![ 7.0,22.0,46.0,105.0]],
        row: 4,
        col: 4
    };
    matrix_test(&div_for_row(&_a,0,_a.mat[0][0]),&_b);    
}

#[test]
fn basic_deform_2_row_works() {
    let mut _a = Matrix{
        mat: vec![vec![ 8.0,16.0,24.0,32.0],
                  vec![ 2.0, 7.0,12.0,17.0],
                  vec![ 6.0,17.0,32.0,59.0],
                  vec![ 7.0,22.0,46.0,105.0]],
        row: 4,
        col: 4
    };
    let _b = Matrix{
        mat: vec![vec![ 4.0, 2.0, 0.0,-2.0],
                  vec![ 2.0, 7.0,12.0,17.0],
                  vec![ 6.0,17.0,32.0,59.0],
                  vec![ 7.0,22.0,46.0,105.0]],
        row: 4,
        col: 4
    };
    matrix_test(&basic_deform_2_row(&_a,-2.0,1,0),&_b);
}

fn main() {
}

