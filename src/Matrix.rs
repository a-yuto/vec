#[macro_use]
use nearly_eq::*;

pub struct Matrix {
    mat: Vec<Vec<f64>>,
    row: usize,
    col: usize
}

pub fn zero(_row: usize,_col: usize) -> Matrix {
    Matrix{
        mat: vec![vec![0.0;_row];_col],
        row: _row,
        col: _col
    }
}

pub fn iden(n: usize) -> Matrix {
    let mut b: Vec<Vec<f64>> = Vec::new();
    for i in 0..n {
        let mut c: Vec<f64> = Vec::new();
        for j in 0..n {
            match i == j {
                true  => c.push(1.0),
                false => c.push(0.0), 
            }
        }
        b.push(c);
    }
    Matrix {
        mat: b,
        row: n,
        col: n
    }
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
pub fn LU(a: &Matrix) -> (Result<Matrix,String>,Result<Matrix,String>) {
    let Mat = &a.mat; 
    let n   = Mat.len();
    let mut temp_l: Vec<Vec<f64>> = vec![vec![0.0;n];n];
    let mut temp_r: Vec<Vec<f64>> = vec![vec![0.0;n];n];

    for i in 0..n {
        for j in 0..n {
            if i > j {
                temp_l[i][j] = Mat[i][j];
                temp_r[i][j] = 0.0;
            } else if i < j {
                temp_l[i][j] = 0.0;
                temp_r[i][j] = Mat[i][j];
            } else {
                temp_l[i][j] = Mat[i][j];
                temp_r[i][j] = Mat[i][j];
            }
        }
    }

    let l:Result<Matrix,String> = match Mat.len() == Mat[0].len() {
        true  => Ok(
                Matrix {
                    mat: temp_l,
                    row: n,
                    col: n
                }
            ),
        false => Err("計算不可能です".to_string()),
    };
    let r:Result<Matrix,String> = match Mat.len() == Mat[0].len() {
        true  => Ok(
                Matrix {
                    mat: temp_r,
                    row: n,
                    col: n
                }
            ),
        false => Err("計算不可能です".to_string()),
    };
    (l,r)
}
pub fn is_tri(a: &Matrix) -> bool {
    let mut right_up  = 0.0;
    let mut left_down = 0.0;
    let Mat = &a.mat;
    let n   = Mat.len();
    for i in 0..n {
        for j in 0..n {
            if i > j {
                right_up += Mat[i][j];
            } else if i < j {
                left_down += Mat[i][j];
            }
        }
    }
    (right_up * left_down) == 0.0
}

pub fn det_for_tri(a: &Matrix) -> f64 {
    let mut det = 1.0;
    let n = a.row;
    for i in 0..n {
        det *= a.mat[i][i];
    }
    det
}
//pub fn adjugate(a: &Matrix) -> Matrix {
    
//}
//pub fn rev(a: &Matrix) -> Result<Matrix,String> {
//    let (l,r) = LU(&a);
//}
//----------------------------ここからテストです---------------------------
#[cfg(test)]
mod mat_tests {
    use super::*;
    #[test]
    pub fn zero_works() {
        let test_generate:Matrix = zero(3,3);
        let ans_generate  = Matrix {
                  mat: vec![vec![0.0,0.0,0.0],
                            vec![0.0,0.0,0.0],
                            vec![0.0,0.0,0.0]],
                  row: 3,
                  col: 3

        };
        assert_eq!(test_generate.mat,ans_generate.mat);
        assert_eq!(test_generate.row,ans_generate.col);
        assert_eq!(test_generate.col,ans_generate.col);
    }

    #[test]
    pub fn iden_works() {
        let test = iden(4);
        let ans  = Matrix {
            mat: vec![vec![1.0,0.0,0.0,0.0],
                      vec![0.0,1.0,0.0,0.0],
                      vec![0.0,0.0,1.0,0.0],
                      vec![0.0,0.0,0.0,1.0]],
            row: 4,
            col: 4
        };
        assert_eq!(test.mat,ans.mat);
        assert_eq!(test.row,ans.col);
        assert_eq!(test.col,ans.col);
    }
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
    //#[test]
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
    #[test]
    pub fn LU_works() {
        let _a = Matrix{
            mat: vec![vec![ 3.0, 1.0],
                      vec![ 5.0, 1.0]],
            row: 2,
            col: 2
        };
        let _l = Matrix {
            mat: vec![vec![3.0,0.0],
                      vec![5.0,1.0]],
            row: 2,
            col: 2
        };
        let _u = Matrix {
            mat: vec![vec![3.0,1.0],
                      vec![0.0,1.0]],
            row: 2,
            col: 2
        };
        let (mut _b,_c) = LU(&_a);
        let _b = _b.unwrap();
        let _c = _c.unwrap();
        assert_eq!(_b.mat,_l.mat);
        assert_eq!(_b.row,_l.row);
        assert_eq!(_b.col,_l.col);
        assert_eq!(_c.mat,_u.mat);
        assert_eq!(_c.row,_u.row);
        assert_eq!(_c.col,_u.col);
    }
    #[test]
    pub fn is_tri_works() {
        let _a = Matrix{
                mat: vec![vec![ 3.0, 1.0],
                          vec![ 5.0, 1.0]],
                row: 2,
                col: 2
            };
        let _l = Matrix {
            mat: vec![vec![3.0,0.0],
                      vec![5.0,1.0]],
            row: 2,
            col: 2
        };
        let _u = Matrix {
            mat: vec![vec![3.0,1.0],
                      vec![0.0,1.0]],
            row: 2,
            col: 2
        };
        let x = Matrix{
            mat: vec![vec![1.0,1.0,2.0,2.0,5.0],
                      vec![3.0,2.0,4.0,3.0,4.0],
                      vec![2.0,2.0,4.0,1.0,2.0],
                      vec![1.9,2.9,2.9,2.9,1.0],
                      vec![0.0,2.0,3.0,2.0,0.0]],
            row: 5,
            col: 5
        };
        let y = Matrix{
            mat: vec![vec![1.0,1.0,2.0,2.0,5.0],
                      vec![0.0,2.0,4.0,3.0,4.0],
                      vec![0.0,0.0,4.0,1.0,2.0],
                      vec![0.0,0.0,0.0,0.0,1.0],
                      vec![0.0,0.0,0.0,0.0,0.0]],
            row: 5,
            col: 5
        };
        assert!(!is_tri(&_a));
        assert!(is_tri(&_l));
        assert!(is_tri(&_u));
        assert!(!is_tri(&x));
        assert!(is_tri(&y));
    }
    #[test]
    pub fn det_for_tri_works() {
        let a = 144.0;
        let y = Matrix{
            mat: vec![vec![1.0,1.0,2.0,2.0,5.0],
                      vec![0.0,2.0,4.0,3.0,4.0],
                      vec![0.0,0.0,4.0,1.0,2.0],
                      vec![0.0,0.0,0.0,6.0,1.0],
                      vec![0.0,0.0,0.0,0.0,3.0]],
            row: 5,
            col: 5
        };
        assert_eq!(a,det_for_tri(&y));
    }
    //#[test]
    pub fn adjugate_works() {
        let test = Matrix {
            mat: vec![vec![ 1.0, 2.0, 3.0],
                      vec![ 1.0, 1.0,-1.0],
                      vec![ 4.0, 1.0, 5.0]],
            row: 3,
            col: 3
        };
        let ans = Matrix {
            mat: vec![vec![ 6.0,-7.0,-5.0],
                      vec![-9.0,-7.0, 4.0],
                      vec![-3.0, 7.0,-1.0]],
            row: 3,
            col: 3
        };
        assert_eq!(test.mat,ans.mat);
        assert_eq!(test.row,ans.row);
        assert_eq!(test.col,ans.col);
    }
}

