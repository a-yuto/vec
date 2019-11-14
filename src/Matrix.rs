#[macro_use]
use nearly_eq::*;

pub struct Matrix {
    mat: Vec<Vec<f64>>,
    row: usize,
    col: usize
}


pub fn is_same_size(a: &Matrix,b: &Matrix) -> bool {
    ( a.row == b.row ) && ( a.col == b.col )
}

pub fn can_mul(a: &Matrix,b: &Matrix) -> bool {
    a.row == b.col 
}

pub fn add(a: &Matrix,b: &Matrix) -> Result<Matrix,String> {
    let mut _c = vec![vec![0.0;a.row];a.col];
    for i in 0..a.col {
        for j in 0..a.row {
            _c[i][j] = a.mat[i][j] + b.mat[i][j];
        }
    }
    let d:Result<Matrix,String> = match is_same_size(&a,&b) {
        true  => Ok(
                Matrix {
                    mat: _c,
                    row: a.row,
                    col: a.col
                }
            ),
        false => Err("計算不可能です".to_string()),
    };
    d
}

pub fn scl_mul(k: &f64,a: &Matrix) -> Matrix {
    let mut b: Vec<Vec<f64>> = Vec::new();
    for vec in &a.mat {
        let mut c: Vec<f64> = Vec::new();
        for val in vec {
            c.push(val*k);
        }
        b.push( c );
    }
    Matrix{
        mat: b,
        row: a.row,
        col: a.col
    }
}

pub fn mul(a: &Matrix,b: &Matrix) -> Result<Matrix,String> {
    let mut c: Vec<Vec<f64>> = vec![vec![0.0;b.row];a.col];
    for i in 0..a.col {
        for j in 0..b.row {
            for k in 0..a.row {
                c[i][j] += &a.mat[i][k]*&b.mat[k][j];
            }
        }
    }
    let Row = c[0].len();
    let Col = c.len();
    let d:Result<Matrix,String> = match can_mul(&a,&b) {
        true  => Ok(
                Matrix {
                    mat: c,
                    row: Row,
                    col: Col
                }
            ),
        false => Err("計算不可能です".to_string()),
    };
    d
}
pub fn trans(a: &Matrix) ->Matrix {
    let mut b: Vec<Vec<f64>>  = vec![vec![0.0;a.col];a.row];
    for i in 0..a.row {
        for j in 0..a.col {
            b[i][j] = a.mat[j][i];
        }
    }
    Matrix {
        mat: b,
        row: a.col,
        col: a.row
    }
}

pub fn rev(a: &Matrix) -> Matrix {
    let b = Matrix {
        mat: vec![vec![0.0;1];1],
        row: 1,
        col: 1
    };
    b
}

//----------------------------ここからテストです---------------------------
#[cfg(test)]
mod mat_tests {
    use super::*;
    #[test]
    pub fn same_size_works() {
        let _a = Matrix{
            mat: vec![vec![1.0,2.0],
                      vec![3.0,4.0]],
            row: 2,
            col: 2
        };
        let _b = Matrix{
            mat: vec![vec![1.0,0.0],
                      vec![0.0,1.0]],
            row: 2,
            col: 2
        };
        let _c = Matrix{
            mat: vec![vec![0.0,2.0,2.0],
                      vec![3.0,3.0,5.0]],
            row: 3,
            col: 2,
        };
        assert!(is_same_size(&_a,&_b));
        assert!(!is_same_size(&_b,&_c));
    }
    #[test]
    pub fn add_works() {
        let _a = Matrix{
            mat: vec![vec![1.0,2.0],
                      vec![3.0,4.0]],
            row: 2,
            col: 2
        };
        let _b = Matrix{
            mat: vec![vec![1.0,0.0],
                      vec![0.0,1.0]],
            row: 2,
            col: 2
        };
        let _c = Matrix{
            mat: vec![vec![0.0,2.0],
                      vec![3.0,3.0]],
            row: 2,
            col: 2,
        };
        let _d = add(&_b,&_c).unwrap();
        assert_eq!(_a.mat,_d.mat);
        assert_eq!(_a.row,_d.row);
        assert_eq!(_a.col,_d.col);

        let _d = Matrix{
            mat: vec![vec![ 1.0,-2.0, 8.0],
                      vec![ 2.0, 5.0,-1.0]],
            row: 3,
            col: 2
        };
        let _e = Matrix{
            mat: vec![vec![-2.0, 5.0, 1.0],
                      vec![ 3.0,-1.0, 2.0]],
            row: 3,
            col: 2
        };
        let _f = Matrix{
            mat: vec![vec![-1.0, 3.0, 9.0],
                      vec![ 5.0, 4.0, 1.0]],
            row: 3,
            col: 2
        };
        let _g = add(&_d,&_e).unwrap();
        assert_eq!(_f.mat,_g.mat);
        assert_eq!(_f.col,_g.col);
        assert_eq!(_f.row,_g.row);
    }


    #[test]
    pub fn scl_mul_works() {
        let _a = Matrix{
            mat: vec![vec![1.0,2.0],
                      vec![3.0,4.0]],
            row: 2,
            col: 2
        };
        let _b = Matrix{
            mat: vec![vec![2.5,5.0],
                      vec![7.5,10.0]],
            row: 2,
            col: 2
        };
        let _k = 2.5;
        let _c = scl_mul(&_k,&_a);
        assert_eq!(_b.mat,_c.mat);
        assert_eq!(_b.col,_c.col);
        assert_eq!(_b.row,_b.col);
    }

    #[test]
    pub fn can_mul_works() {
        let _d = Matrix{
            mat: vec![vec![ 1.0, 2.0, 4.0, 5.0, 1.0],
                      vec![ 0.0, 2.0, 1.0, 1.0, 3.0],
                      vec![ 3.0, 0.0, 2.0, 0.0, 1.0]],
            row: 5,
            col: 3
        };
        let _e = Matrix{
            mat: vec![vec![-1.0, 2.0],
                      vec![ 0.0, 3.0],
                      vec![ 1.0,-2.0],
                      vec![-1.0, 1.0],
                      vec![ 2.0,-3.0]],
            row: 2,
            col: 5
        };
        let _f = Matrix{
            mat: vec![vec![ 0.0, 2.0],
                      vec![ 6.0,-4.0],
                      vec![ 1.0,-1.0]],
            row: 2,
            col: 3
        };
        assert!(can_mul(&_d,&_e));
        assert!(!can_mul(&_d,&_f));

    }

    #[test]
    pub fn mul_works() {
        let _d = Matrix{
            mat: vec![vec![ 1.0, 2.0, 4.0, 5.0, 1.0],
                      vec![ 0.0, 2.0, 1.0, 1.0, 3.0],
                      vec![ 3.0, 0.0, 2.0, 0.0, 1.0]],
            row: 5,
            col: 3
        };
        let _e = Matrix{
            mat: vec![vec![-1.0, 2.0],
                      vec![ 0.0, 3.0],
                      vec![ 1.0,-2.0],
                      vec![-1.0, 1.0],
                      vec![ 2.0,-3.0]],
            row: 2,
            col: 5
        };
        let _f = Matrix{
            mat: vec![vec![ 0.0, 2.0],
                      vec![ 6.0,-4.0],
                      vec![ 1.0,-1.0]],
            row: 2,
            col: 3
        };
        let _g = mul(&_d,&_e).unwrap();
        assert_eq!(_f.mat, _g.mat);
        assert_eq!(_f.row, _g.row);
        assert_eq!(_f.col, _g.col);
    }
    #[test]
    pub fn trans_works(){
        let _a = Matrix{
            mat: vec![vec![ 1.0, 3.0,-2.0],
                      vec![ 4.0, 5.0, 2.0]],
            row: 3,
            col: 2
        };
        let _b = Matrix{
            mat: vec![vec![ 1.0, 4.0],
                      vec![ 3.0, 5.0],
                      vec![-2.0, 2.0]],
            row: 2,
            col: 3
        };
        let _c = trans(&_a);
        assert_eq!(_b.mat,_c.mat);
        assert_eq!(_b.col,_c.col);
        assert_eq!(_b.row,_c.row);
    }
    #[test]
    pub fn rev_works() {
        let _a = Matrix{
            mat: vec![vec![ 3.0, 1.0],
                      vec![ 5.0, 1.0]],
            row: 2,
            col: 2
        };
        let _b = Matrix{
            mat: vec![vec![-0.5, 0.5],
                      vec![ 2.5,-1.5]],
            row: 2,
            col: 2
        };
        let _c = rev(&_a);
        assert_eq!(&_b.mat,&_c.mat);
        assert_eq!(&_b.col,&_c.col);
        assert_eq!(&_b.row,&_c.row);
    }
}

