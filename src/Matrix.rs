#[macro_use]
use nearly_eq::*;

pub struct Matrix {
    mat: Vec<Vec<f64>>,
    row: usize,
    col: usize
}

pub fn get_row(k: usize,a: &Matrix) -> Result<Matrix,String> {
    let b =  vec![a.mat[k - 1].clone();1];
    let r = b[0].len();
    let l = b.len();
    let c:Result<Matrix,String> = match true {
        true  => Ok(
            Matrix{
                mat: b,
                row: r,
                col: l
            }
        ),
        false => Err("計算不可能です".to_string()),
    };
    c
}

pub fn get_col(k: usize,a: &Matrix) -> Result<Matrix,String> {
    let mut b = zero(1,a.col);
    for i in 0..a.col {
        println!("{}",i);
        b.mat[i][0] = a.mat[i][k - 1];
    }
    let c:Result<Matrix,String> = match true  {
        true  => Ok(b),
        false => Err("計算不可能です".to_string()),
    };
    c
}
pub fn print_mat(a: &Matrix) {
    let mat = &a.mat;
    for vec in mat {
        println!("{:?}",vec);
    }
    println!("");
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

    let row = c[0].len();
    let col = c.len();
    let d:Result<Matrix,String> = match can_mul(&a,&b) {
        true  => Ok(
                Matrix {
                    mat: c,
                    row: row,
                    col: col
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

pub fn split(a: &Matrix,n: usize) -> Result<Matrix,String> {
    let can = (a.col == a.row) && (a.col != 1);
    let s   = a.col - n;
    let mat         = &a.mat;
    let mut spl_mut = zero(s,s).mat;
    let mut x = 0;
    for i in n..a.col {
        let mut y = 0;
        for j in n..a.row {
            spl_mut[x][y] = mat[i][j];
            y += 1;
        }
        x += 1;
    }
    let d:Result<Matrix,String> = match can {
        true  => Ok(
                Matrix {
                mat: spl_mut,
                row: s,
                col: s}
            ),
        false => Err("計算不可能です".to_string()),
    };
    d
}

pub fn LU_cal(i: usize,N: usize,mut temp_l: Matrix,mut temp_u: Matrix,mut lu: Matrix,matA: &Matrix) -> (Matrix,Matrix) {
    let n  = N - i - 1;
    let l0           = matA.mat[0][0];
    temp_l.mat[i][i] = matA.mat[0][0];

    let mut l1: Vec<f64> = vec![0.0;n];
    for j in 0..n {
        temp_l.mat[j + i + 1][i] = matA.mat[j + 1][0];
        l1[j]                    = matA.mat[j + 1][0];
    }

    let mut u1: Vec<f64> = vec![0.0;n];
    for j in 0..n {
        temp_u.mat[i][j + i + 1] = matA.mat[0][j + 1] / l0;
        u1[j]                    = matA.mat[0][j + 1] / l0;
    }

    for j in 0..n {
        for k in 0..n {
            lu.mat[j][k] = l1[j] * u1[k];
        }
    }

    let mut A1 = zero(n,n);
    for j in 0..n {
        A1.mat[j] = vec![0.0;n];
        for k in 0..n {
            A1.mat[j][k] = matA.mat[j + 1][k + 1] - lu.mat[j][k];
        }
    }

    if i == N - 1 {
        (temp_l,temp_u)
    }else{
        LU_cal(i + 1,N,temp_l,temp_u,lu,&A1)
    }
}
pub fn LU(a: &Matrix) -> (Result<Matrix,String>,Result<Matrix,String>) {
    let N = a.col;
    let temp_l = zero(N,N);
    let temp_u = iden(N);
    let lu     = zero(N,N);

    let (L,U) = LU_cal(0,N,temp_l,temp_u,lu,a);
    let l:Result<Matrix,String> = match a.col == a.row {
        true  => Ok(L),
        false => Err("計算不可能です".to_string()),
    };
    let u:Result<Matrix,String> = match a.col == a.row {
        true  => Ok(U),
        false => Err("計算不可能です".to_string()),
    };
    (l,u)
}
pub fn is_tri(a: &Matrix) -> bool {
    let mut right_up  = 0.0;
    let mut left_down = 0.0;
    let mat = &a.mat;
    let n   = mat.len();
    for i in 0..n {
        for j in 0..n {
            if i > j {
                right_up += mat[i][j];
            } else if i < j {
                left_down += mat[i][j];
            }
        }
    }
    (right_up * left_down) == 0.0
}

pub fn det_for_tri(a: &Matrix) -> Result<f64,String> {
    let mut det = 1.0;
    let n = a.row;
    for i in 0..n {
        det *= a.mat[i][i];
    }
    let Det:Result<f64,String> = match is_tri(&a) {
        true  => Ok(det),
        false => Err("三角行列ではありません".to_string()),
    };
    Det
}
pub fn reduct(a: &Matrix) -> Matrix {
    let mut w = zero(a.col - 1,a.row - 1);
    for row in 1..a.row {
        for col in 1..a.col {
            w.mat[col - 1][row - 1] = a.mat[col][row] - (a.mat[col][0] / a.mat[0][0]) * a.mat[0][row];
        }
    }
    w
}

pub fn inverse(mut a: &mut Matrix) -> Matrix{
    let n = a.col;
    let mut inv_A = iden(n);
    let mut buf   = 0.0;
    for i in 0..n {
        buf = 1.0/a.mat[i][i];
        for j in 0..n {
            a.mat[i][j]     *= buf;
            inv_A.mat[i][j] *= buf;
        }
        for j in 0..n {
            if i != j {
                buf = a.mat[j][i];
            }
            for k in 0..n {
                a.mat[j][k]     -= a.mat[i][k]*buf;
                inv_A.mat[j][k] -= inv_A.mat[i][k]*buf;
            }
        }
    }
    inv_A
}
//2×2に限定
pub fn strassen(a: &Matrix,b: &Matrix) -> Matrix {
    if !((a.row == a.col) && (b.row == b.col) && (a.row == 2) ) {
        panic!("条件外です。");
    }
    let p1 = (a.mat[0][0] +  a.mat[1][1]) * (b.mat[0][0] + b.mat[1][1]);
    let p2 = (a.mat[1][0] +  a.mat[1][1]) *  b.mat[0][0];
    let p3 =  a.mat[0][0] * (b.mat[0][1]  -  b.mat[1][1]);
    let p4 =  a.mat[1][1] * (b.mat[1][0]  -  b.mat[0][0]);
    let p5 = (a.mat[0][0] +  a.mat[0][1]) *  b.mat[1][1];
    let p6 = (a.mat[1][0] -  a.mat[0][0]) * (b.mat[0][0] + b.mat[0][1]);
    let p7 = (a.mat[0][1] -  a.mat[1][1]) * (b.mat[1][0] + b.mat[1][1]);

    let c11 = p1 + p4 - p5 + p7;
    let c12 = p3 + p5;
    let c21 = p2 + p4;
    let c22 = p1 + p3 - p2 + p6;
    Matrix{
        mat:vec![vec![c11,c12],
                 vec![c21,c22]],
        col: 2,
        row: 2,
    }
}
//----------------------------ここからテストです---------------------------
#[cfg(test)]
mod mat_tests {
    use super::*;
    #[test]
    pub fn strassen_works(){
        let a = Matrix{
            mat: vec![vec![1.0,2.0],
                      vec![3.0,4.0]],
            col: 2,
            row: 2
        };
        let b = Matrix{
            mat: vec![vec![5.0,6.0],
                      vec![7.0,8.0]],
            col: 2,
            row: 2
        };
        assert_eq!(mul(&a,&b).unwrap().mat,strassen(&a,&b).mat);

    }
    //#[test]
    pub fn det_works() {
        let a = Matrix{
            mat: vec![vec![1.0,2.0,3.0,4.0],
                      vec![1.0,2.0,3.0,4.0],
                      vec![1.0,2.0,3.0,4.0],
                      vec![1.0,2.0,3.0,4.0]],
            col: 4,
            row: 4
        };
        let b = -160.0;
    }
    #[test]
    pub fn reduct_test(){
        let a = Matrix{
            mat: vec![vec![4.0,3.0,2.0,1.0],
                      vec![1.0,4.0,3.0,2.0],
                      vec![2.0,1.0,4.0,3.0],
                      vec![3.0,2.0,1.0,4.0]],
            col: 4,
            row: 4
        };
        let b = Matrix{
            mat: vec![vec![13.0/4.0,5.0/2.0,7.0/4.0],
                      vec![-1.0/2.0,    3.0,5.0/2.0],
                      vec![-1.0/4.0,-1.0/2.0,13.0/4.0]],
            col: 3,
            row: 3
        };
        assert_eq!(reduct(&a).mat,b.mat);
    }
    #[test]
    pub fn get_row_works() {
        let _a = Matrix{
            mat: vec![vec![-1.0, 2.0],
                      vec![ 0.0, 3.0],
                      vec![ 1.0,-2.0],
                      vec![-1.0, 1.0],
                      vec![ 2.0,-3.0]],
            row: 2,
            col: 5
        };
        let _b = Matrix{
            mat: vec![vec![0.0,3.0]],
            row: 2,
            col: 1
        };
        let _c = get_row(2,&_a).unwrap();
        assert_eq!(_c.mat,_b.mat);
        assert_eq!(_c.row,_b.row);
        assert_eq!(_c.col,_b.col);
    }
    #[test]
    pub fn get_col_works() {
        let _a = Matrix{
            mat: vec![vec![-1.0, 2.0],
                      vec![ 0.0, 3.0],
                      vec![ 1.0,-2.0],
                      vec![-1.0, 1.0],
                      vec![ 2.0,-3.0]],
            row: 2,
            col: 5
        };
        let _b = Matrix{
            mat: vec![vec![ 2.0],
                      vec![ 3.0],
                      vec![-2.0],
                      vec![ 1.0],
                      vec![-3.0]],
            row: 1,
            col: 5
        };
        let _c = get_col(2,&_a).unwrap();
        assert_eq!(_c.mat,_b.mat);
        assert_eq!(_c.row,_b.row);
        assert_eq!(_c.col,_b.col);
    }
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
        let mut _a = Matrix{
            mat: vec![vec![ 1.0, 1.0,-1.0],
                      vec![-2.0, 0.0, 1.0],
                      vec![ 0.0, 2.0, 1.0]
            ],
            row: 3,
            col: 3
        };
        let _b = Matrix{
            mat: vec![vec![-0.5,-0.75,0.25],
                      vec![ 0.5, 0.25,0.25],
                      vec![-1.0, -0.5, 0.5]
            ],
            row: 3,
            col: 3
        };
        let _c = inverse(&mut _a);
        assert_eq!(&_b.mat,&_c.mat);
        assert_eq!(&_b.col,&_c.col);
        assert_eq!(&_b.row,&_c.row);
    }
    #[test]
    pub fn split_wprks() {
        let _a = Matrix{
            mat: vec![vec![ 8.0,16.0,24.0,32.0],
                      vec![ 2.0, 7.0,12.0,17.0],
                      vec![ 6.0,17.0,32.0,59.0],
                      vec![ 7.0,22.0,46.0,105.0]],
            row: 4,
            col: 4
        };
        let _b = Matrix{
            mat: vec![vec![ 7.0,12.0,17.0],
                      vec![17.0,32.0,59.0],
                      vec![22.0,46.0,105.0]],
            row: 3,
            col: 3
        };
        assert_eq!(&_b.mat,&split(&_a,1).unwrap().mat)

    }
    #[test]
    pub fn LU_works() {
        let _a = Matrix{
            mat: vec![vec![ 8.0,16.0,24.0,32.0],
                      vec![ 2.0, 7.0,12.0,17.0],
                      vec![ 6.0,17.0,32.0,59.0],
                      vec![ 7.0,22.0,46.0,105.0]],
            row: 4,
            col: 4
        };
        let _l = Matrix {
            mat: vec![vec![ 8.0, 0.0, 0.0, 0.0],
                      vec![ 2.0, 3.0, 0.0, 0.0],
                      vec![ 6.0, 5.0, 4.0, 0.0],
                      vec![ 7.0, 8.0, 9.0, 8.0]],
            row: 4,
            col: 4
        };

        let _u = Matrix {
            mat: vec![vec![ 1.0, 2.0, 3.0, 4.0],
                      vec![ 0.0, 1.0, 2.0, 3.0],
                      vec![ 0.0, 0.0, 1.0, 5.0],
                      vec![ 0.0, 0.0, 0.0, 1.0]],
            row: 4,
            col: 4
        };
        let (mut _b,_c) = LU(&_a);
        let _b = _b.unwrap();
        let _c = _c.unwrap();
        assert_eq!(mul(&_b,&_c).unwrap().mat,_a.mat);
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
        assert_eq!(a,det_for_tri(&y).unwrap());
    }
}
