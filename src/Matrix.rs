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
pub fn scl_mul(k: &f64,a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut b: Vec<Vec<f64>> = Vec::new();
    for vec in a {
        let mut c: Vec<f64> = Vec::new();
        for val in vec {
            c.push(val*k);
        }
        b.push( c );
    }
    b
}

pub fn mul(a: &Vec<Vec<f64>>,b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let  _a_row:         usize = a[0].len();
    let  _a_col:         usize = a.len();
    let  _b_row:         usize = b[0].len();
    let  _b_col:         usize = b.len();
    let mut c: Vec<Vec<f64>> = vec![vec![0.0;_b_row];_a_col];
    for i in 0.._a_col {
        for j in 0.._b_row {
            for k in 0.._a_row {
                c[i][j] += a[i][k]*b[k][j];
            }
        }
    }
    c
}
pub fn trans(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let col = a.len();
    let row = a[0].len();
    let mut b: Vec<Vec<f64>>  = vec![vec![0.0;col];row];
    for i in 0..row {
        for j in 0..col {
            b[i][j] = a[j][i];
        }
    }
    b
}
//n = nに限定してLU分解で解く



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
        let _a = vec![vec![1.0,2.0],
                      vec![3.0,4.0]
        ];
        let _b = vec![vec![2.5,5.0],
                      vec![7.5,10.0]
        ];
        let _k = 2.5;
        assert_eq!(_b,scl_mul(&_k,&_a));
    }
    #[test]
    pub fn mul_works() {
        let _a = vec![vec![1.0,2.0],
                      vec![3.0,4.0]
        ];
        let _b = vec![vec![2.5,5.0],
                      vec![7.5,10.0]
        ];
        let _c = vec![vec![17.5,25.0],
                      vec![37.5,55.0]
        ];
        assert_eq!(_c, mul(&_a,&_b));

        let _d = vec![vec![ 1.0, 2.0, 4.0, 5.0, 1.0],
                      vec![ 0.0, 2.0, 1.0, 1.0, 3.0],
                      vec![ 3.0, 0.0, 2.0, 0.0, 1.0]
        ];
        let _e = vec![vec![-1.0, 2.0],
                      vec![ 0.0, 3.0],
                      vec![ 1.0,-2.0],
                      vec![-1.0, 1.0],
                      vec![ 2.0,-3.0]
        ];
        let _f = vec![vec![ 0.0, 2.0],
                      vec![ 6.0,-4.0],
                      vec![ 1.0,-1.0]
        ];
        assert_eq!(_f, mul(&_d,&_e));
    }
    #[test]
    pub fn trans_works(){
        let _a = vec![vec![ 1.0, 3.0,-2.0],
                      vec![ 4.0, 5.0, 2.0]
        ];
        let _b = vec![vec![ 1.0, 4.0],
                      vec![ 3.0, 5.0],
                      vec![-2.0, 2.0]
        ];
        assert_eq!(_b,trans(&_a));
    }
    //#[test]
    //pub fn rev_works() {
    //  let _a = vec![vec![ 3.0, 1.0],
    //                  vec![ 5.0, 1.0]
    //    ];
    //    let _b = vec![vec![-0.5, 0.5],
    //                  vec![ 2.5,-1.5]
    //    ];
    //    println!("{:?}",mul(&_a,&_b));
    //    assert_eq!(&_b,&rev(&_a));
    //}
}

