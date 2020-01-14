extern crate matrix;
use vec::matrix::*;

pub fn line_eq(_A: &Matrix, _b: &Matrix) -> Matrix {
    /* A  * x = b
     * LU * x = b
     * l  * y = b (U * x = y)
     *
     * U  * x = y
    */
    let (_rl,_ru) = lu(&_A);
    let       _l  = _rl.unwrap();
    let       _u  = _ru.unwrap();
    let mut   _y  = zero(1,_A.col);
    //solve l  * y = b
    for i in (0.._l.row) {
        _y.mat[i][0] = _b.mat[i][0];
        for j in (0..i) {
            _y.mat[i][0] += - 1.0 *  _y.mat[j][0] * _l.mat[i][j] 
        }
        _y.mat[i][0] /= _l.mat[i][i];
    }
    //solve U * x = y
    print_mat(&_u);
    println!(" * x = ");
    print_mat(&_y);
    let mut _x = zero(1,_A.col);
    for i in (0.._u.row).rev() {
        _x.mat[i][0] = _y.mat[i][0];
        for j in (i+1.._u.row) {
            _x.mat[i][0] += - 1.0 *  _x.mat[j][0] * _u.mat[i][j]
        }
    }
    _x
}

#[test]
fn line_eq_work(){
    let _A = Matrix {
        mat: vec![vec![1.0,1.0,1.0],
                  vec![4.0,3.0,5.0],
                  vec![3.0,5.0,3.0]],
        row: 3,
        col: 3
    };
    let _b = Matrix {
        mat: vec![vec![1.0],
                  vec![6.0],
                  vec![4.0]],
        row: 1,
        col: 3
    };
    let _x = Matrix {
        mat: vec![vec![-2.0],
                  vec![ 0.5],
                  vec![2.5]],
        row: 1,
        col: 3
    };
    matrix_test(&line_eq(&_A,&_b),&_x);
}

fn main() {
}

