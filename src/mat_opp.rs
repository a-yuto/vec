use std::ops::{Add,Sub,Mul};
use nearly_eq::*;

#[derive(Debug,Clone)]
pub struct Matrix {
    pub mat: Vec<Vec<f64>>,
    pub row: usize,
    pub col: usize
}
impl Add for &Matrix{
    type Output = Matrix;
    fn add(self, other: &Matrix) -> Matrix {
        add_mat(self,other).unwrap() 
    }
}

impl Add<&f64> for &Matrix {
    type Output = Matrix;
    fn add(self,other: &f64) -> Matrix {
        scl_add(other,self)
    }
}

pub fn add_mat(a: &Matrix,b: &Matrix) -> Result<Matrix,String> {
    let ans =
    (0..a.col).map(|i| {
        (0..a.row).map(|j|
            a.mat[i][j] + b.mat[i][j]
        ).collect()
    }).collect();
    match is_same_size(&a,&b) {
        true => Ok(Matrix {
            mat: ans,
            row: a.row,
            col: a.col
        }),
        false => Err("cannot calicurate.".to_string())
    }
}

pub fn scl_add(k: &f64,a: &Matrix) -> Matrix {
    let mut b: Vec<Vec<f64>> = Vec::new();
    for vec in &a.mat {
        let mut c: Vec<f64> = Vec::new();
        for val in vec {
            c.push(val+k);
        }
        b.push( c );
    }
    Matrix{
        mat: b,
        row: a.row,
        col: a.col
    }
}


impl Sub for &Matrix{
    type Output = Matrix;
    fn sub(self, other: &Matrix) -> Matrix {
        sub_mat(self,other).unwrap()
    }
}


impl Sub<&f64> for &Matrix {
    type Output = Matrix;
    fn sub(self,other: &f64) -> Matrix {
        scl_sub(&other,&self)
    }
}


pub fn scl_sub(k: &f64,a: &Matrix) -> Matrix {
    let mut b: Vec<Vec<f64>> = Vec::new();
    for vec in &a.mat {
        let mut c: Vec<f64> = Vec::new();
        for val in vec {
            c.push(val-k);
        }
        b.push( c );
    }
    Matrix{
        mat: b,
        row: a.row,
        col: a.col
    }
}

pub fn sub_mat(a: &Matrix,b: &Matrix) -> Result<Matrix,String> {
    let ans =
    (0..a.col).map(|i| {
        (0..a.row).map(|j|
            a.mat[i][j] - b.mat[i][j]
        ).collect()
    }).collect();
    match is_same_size(&a,&b) {
        true => Ok(Matrix {
            mat: ans,
            row: a.row,
            col: a.col
        }),
        false => Err("cannot calicurate.".to_string())
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self,other: &Matrix) -> Matrix {
        mul_mat(&self,&other).unwrap()
    }   
}

impl Mul<&f64> for &Matrix {
    type Output = Matrix;
    fn mul(self,other: &f64) -> Matrix {
        scl_mul(other,self) 
    }    
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
pub fn mul_mat(a: &Matrix,b: &Matrix) -> Result<Matrix,String> {
    let mut ans: Vec<Vec<f64>> = vec![vec![0.0;b.row];a.col];
    for i in 0..a.col {
        for j in 0..b.row {
            for k in 0..a.row {
                ans[i][j] += &a.mat[i][k]*&b.mat[k][j];
            }
        }
    }
    let row = ans[0].len();
    let col = ans.len();
    match can_mul(&a,&b) {
        true  => Ok(
                Matrix {
                    mat: ans,
                    row: row,
                    col: col
                }
            ),
        false => Err("計算不可能です".to_string()),
    }
}

pub fn get_row(spe_row: usize,matrix: &Matrix) -> Result<Matrix,&str> {
    let row = vec![matrix.mat[spe_row].clone();1];
    match spe_row < matrix.col {
        true  => Ok(
            Matrix{
                mat: row,
                row: matrix.row,
                col: 1
            }
        ),
        false => Err("The row dose not exist."),
    }
}
pub fn get_col(spe_col: usize,matrix: &Matrix) -> Result<Matrix,&str> {
    let mut col = zero(1,matrix.col);
    for i in 0..matrix.col {
        col.mat[i][0] = matrix.mat[i][spe_col];
    }
    match matrix.row > spe_col {
        true  => Ok(col),
        false => Err("The col does not exist."),
    }
}
pub fn print_mat(matrix: &Matrix) {
    for vec in &matrix.mat {
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
    let iden_matrix = 
    (0..n).map(|i| {
            (0..n).map(|j| 
                if i == j {1.0} else {0.0}
            ).collect()
    }).collect();
    Matrix {
        mat: iden_matrix,
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
pub fn trans(a: &Matrix) ->Matrix {
    let ans : Vec<Vec<f64>> = 
    (0..a.row).map(|i| {
        (0..a.col).map(|j| 
            a.mat[j][i]
        ).collect()
    }).collect();
    Matrix {
        mat: ans,
        row: a.col,
        col: a.row
    }
}


//LU分解用の行列を縮小させる関数
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

pub fn lu_cal(i: usize,k: usize,mut temp_l: Matrix,mut temp_u: Matrix,mut lu: Matrix,mat_a: &Matrix) -> (Matrix,Matrix) {
    let n  = k - i - 1;
    let l0           = mat_a.mat[0][0];
    temp_l.mat[i][i] = mat_a.mat[0][0];

    let mut l1: Vec<f64> = vec![0.0;n];
    for j in 0..n {
        temp_l.mat[j + i + 1][i] = mat_a.mat[j + 1][0];
        l1[j]                    = mat_a.mat[j + 1][0];
    }

    let mut u1: Vec<f64> = vec![0.0;n];
    if l0 == 0.0 {
        println!("occors zero division!!!!!!!!!");
    }
    for j in 0..n {
        temp_u.mat[i][j + i + 1] = mat_a.mat[0][j + 1] / l0;
        u1[j]                    = mat_a.mat[0][j + 1] / l0;
    }

    for j in 0..n {
        for k in 0..n {
            lu.mat[j][k] = l1[j] * u1[k];
        }
    }

    let mut a1 = zero(n,n);
    for j in 0..n {
        a1.mat[j] = vec![0.0;n];
        for k in 0..n {
            a1.mat[j][k] = mat_a.mat[j + 1][k + 1] - lu.mat[j][k];
        }
    }

    if i == k - 1 {
        (temp_l,temp_u)
    }else{
        lu_cal(i + 1,k,temp_l,temp_u,lu,&a1)
    }
}

pub fn lu(a: &Matrix) -> (Result<Matrix,String>,Result<Matrix,String>) {
    let n = a.col;
    let temp_l = zero(n,n);
    let temp_u = iden(n);
    let lu     = zero(n,n);
    let (ll,uu) = lu_cal(0,n,temp_l,temp_u,lu,a);
    let l:Result<Matrix,String> = match a.col == a.row {
        true  => Ok(ll),
        false => Err("計算不可能です".to_string()),
    };
    let u:Result<Matrix,String> = match a.col == a.row {
        true  => Ok(uu),
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
pub fn mat_print(a: &Matrix) {
    let aa = &a.mat;
    for r in aa{
        println!("{:?}",r);
    }
}

pub fn det_for_tri(a: &Matrix) -> Result<f64,String> {
    let mut tmp = 1.0;
    let n = a.row;
    for i in 0..n {
        tmp *= a.mat[i][i];
    }
    match is_tri(&a) {
        true  => Ok(tmp),
        false => Err("三角行列ではありません".to_string()),
    }
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

pub fn inverse(a: &mut Matrix) -> Matrix{
    let n = a.col;
    let mut inv_a = iden(n);
    for i in 0..n {
        let mut buf = 1.0/a.mat[i][i];
        for j in 0..n {
            a.mat[i][j]     *= buf;
            inv_a.mat[i][j] *= buf;
        }
        for j in 0..n {
            if i != j {
                buf = a.mat[j][i];
            }
            for k in 0..n {
                a.mat[j][k]     -= a.mat[i][k]*buf;
                inv_a.mat[j][k] -= inv_a.mat[i][k]*buf;
            }
        }
    }
    inv_a
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


pub fn connect(a: &mut Matrix,b: &mut Matrix) -> Result<Matrix,String> {
    let ok = a.row == b.row;
    let mut matrix:Vec<Vec<f64>> = Vec::new();
    for i in 0..a.col {
        let left  = &mut a.mat[i];
        let mut right = &mut b.mat[i];
        left.append(&mut right);
        matrix.push(left.to_vec());
    }

    match ok {
    true  => Ok(Matrix {
            mat: matrix,
            row: a.row + b.row,
            col: a.col
        }),
    false => Err(
        "結合できません".to_string()
        )
    }
}
pub fn exchange(a: &Matrix, x: usize, y: usize) -> Result<Matrix,String> {
    let _can_exchange = (1 <= x) && (x < a.col) && (1 <= y) && (y < a.col);
    let  matrix = &mut a.mat.clone();
    matrix.swap(x - 1,y - 1);
    match true {
        true  => Ok(Matrix{
                mat: matrix.to_vec(),
                col: a.col,
                row: a.row
            }
        ),
        false => Err("指定地が範囲外です。".to_string())
    }
}



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

pub fn line_eq(_a: &Matrix, _b: &Matrix) -> Matrix {
    /* A  * x = b
     * LU * x = b
     * l  * y = b (U * x = y)
     *
     * U  * x = y
    */
    let (_rl,_ru) = lu(&_a);
    let       _l  = _rl.unwrap();
    let       _u  = _ru.unwrap();
    let mut   _y  = zero(1,_a.col);
    //solve l  * y = b
    for i in 0.._l.row {
        _y.mat[i][0] = _b.mat[i][0];
        for j in 0..i {
            _y.mat[i][0] += - 1.0 *  _y.mat[j][0] * _l.mat[i][j]
        }
        _y.mat[i][0] /= _l.mat[i][i];
    }
    //solve U * x = y
    let mut _x = zero(1,_a.col);
    for i in (0.._u.row).rev() {
        _x.mat[i][0] = _y.mat[i][0];
        for j in i + 1.._u.row {
            _x.mat[i][0] += - 1.0 *  _x.mat[j][0] * _u.mat[i][j]
        }
    }
    _x
}


//----------------------------ここからテストです---------------------------
pub fn matrix_test(a: &Matrix,b: &Matrix) {
    assert_nearly_eq!(a.mat,b.mat);
    assert_eq!(a.col,b.col);
    assert_eq!(a.row,b.row);
}
#[cfg(test)]
mod mat_tests {
    use super::*;
    
    #[test]
    fn line_eq_work(){
        let _a = Matrix {
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
        matrix_test(&line_eq(&_a,&_b),&_x);
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


    #[test]
    pub fn exchenge_works() {
        let a = iden(3);
        let b = Matrix {
            mat: vec![vec![0.0,0.0,1.0],
                      vec![0.0,1.0,0.0],
                      vec![1.0,0.0,0.0]],
            row: 3,
            col: 3  
        };
        matrix_test(&exchange(&a,1,3).unwrap(),&b)
    }

    #[test]
    pub fn connect_works(){
        let mut a: Matrix = Matrix {
            mat: vec![vec![1.0,2.0],
                      vec![3.0,4.0]],
            row: 2,
            col: 2
        };
        let b: Matrix = Matrix {
            mat: vec![vec![1.0,2.0,1.0,0.0],
                      vec![3.0,4.0,0.0,1.0]],
            row: 4,
            col: 2
        };
        let mut e = iden(2);
        matrix_test(&connect(&mut a,&mut e).unwrap(),&b);
    }

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
        matrix_test(&mul_mat(&a,&b).unwrap(),&strassen(&a,&b));

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
        matrix_test(&reduct(&a),&b);
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
            mat: vec![vec![1.0,-2.0]],
            row: 2,
            col: 1
        };
        let _c = get_row(2,&_a).unwrap();
        matrix_test(&_c,&_b);
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
        let _c = get_col(1,&_a).unwrap();
        matrix_test(&_c,&_b);
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
        matrix_test(&test_generate,&ans_generate);
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
        matrix_test(&test,&ans);
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
    pub fn add_mat_works() {
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
        let _d = add_mat(&_b,&_c).unwrap();
        matrix_test(&_a,&(&_b + &_c));
        matrix_test(&_a,&_d);

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
        let _g = add_mat(&_d,&_e).unwrap();
        matrix_test(&_f,&_g);

        matrix_test(&_f,&(&_d + &_e));
    }
    
    #[test]
    pub fn scl_add_works() {
        let _a = Matrix{
            mat: vec![vec![1.0,2.0],
                      vec![3.0,4.0]],
            row: 2,
            col: 2
        };
        let _b = Matrix{
            mat: vec![vec![3.5,4.5],
                      vec![5.5,6.5]],
            row: 2,
            col: 2
        };
        let _k = 2.5;
        let _c = scl_add(&_k,&_a);
        matrix_test(&_b,&(&_a + &_k));
        matrix_test(&_b,&_c);
    } 

    #[test]
    pub fn scl_sub_works() {
        let _a = Matrix{
            mat: vec![vec![1.0,2.0],
                      vec![3.0,4.0]],
            row: 2,
            col: 2
        };
        let _b = Matrix{
            mat: vec![vec![-1.5,-0.5],
                      vec![ 0.5, 1.5]],
            row: 2,
            col: 2
        };
        let _k = 2.5;
        let _c = scl_sub(&_k,&_a);
        matrix_test(&_b,&(&_a - &_k));
        matrix_test(&_b,&_c);
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
        matrix_test(&_b,&(&_a * &_k));
        matrix_test(&_b,&_c);
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
    pub fn mul_mat_works() {
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
        let _g = mul_mat(&_d,&_e).unwrap();
        matrix_test(&_f,&_g);
        matrix_test(&_f,&(&_d * &_e));
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
        matrix_test(&_b,&_c);
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
    pub fn lu_works() {
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
        let (mut _b,_c) = lu(&_a);
        let _b = _b.unwrap();
        let _c = _c.unwrap();
        matrix_test(&(&_b*&_c),&_a);
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
