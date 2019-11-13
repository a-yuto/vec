#[macro_use]
pub struct VecOpp;
extern crate matrix;
extern crate nearly_eq;
use nearly_eq::*;

impl VecOpp {
    pub fn add(a: &Vec<f64>,b: &Vec<f64>) -> Result<Vec<f64>,String>{
        let mut c: Vec<f64> = Vec::new();
        for i in 0..a.len() {
            c.push(a[i] + b[i]);
        }
        let _can = a.len() == b.len(); 
        let d:Result<Vec<f64>,String> = match _can {
            true  => Ok(c), 
            false => Err("計算不可能です".to_string()),
        };
        d
    }
    
    pub fn scl_mul(k: &f64,a: &Vec<f64>) -> Vec<f64> {
        let mut b: Vec<f64> = Vec::new();
        for i in a {
            b.push(k * i);
        }
        b
    }
    
    pub fn in_mul(a: &Vec<f64>,b: &Vec<f64>) -> Result<f64,String> {
        let mut c: f64 = 0.;
        for i in 0..a.len() {
            c += a[i]*b[i];
        }

        let _can = a.len() == b.len();
        let d:Result<f64,String> = match _can {
            true  => Ok(c),
            false => Err("計算不可能です".to_string()),
        };
        d
    }
    //いったん平面ベクトルのみに限って、外積はスカラーを返すようにします
    pub fn out_mul(a: &Vec<f64>,b: &Vec<f64>) -> Result<f64,String> {
        let mut c: f64 = 0.;
        for i in 0..a.len() {
            if i%2 == 0 {
                c += a[i] * b[a.len() - 1 - i];
        
            } else {
                c -= a[i] * b[a.len() - 1 - i]; 
            }
        }

        let _can = a.len() == b.len();
        let d:Result<f64,String> = match _can {
            true  => Ok(c),
            false => Err("計算不可能です".to_string()),
        };
        d

    }

    pub fn size(a: &Vec<f64>) -> f64 {
        let mut b: f64 = 0.;
        for i in a {
            b += i*i;
        }
        b.sqrt()
    }

    pub fn sin_bt_vec(a: &Vec<f64>,b: &Vec<f64>) -> Result<f64,String> {
        let c: f64 = VecOpp::out_mul(&a,&b).unwrap() / ( VecOpp::size(&a) * VecOpp::size(&b) );
        
        let _can = a.len() == b.len();
        let d:Result<f64,String> = match _can {
            true  => Ok(c),
            false => Err("計算不可能です".to_string()),
        };
        d
    }

    pub fn cos_bt_vec(a: &Vec<f64>,b: &Vec<f64>) -> Result<f64,String> {
        let c: f64 = VecOpp::in_mul(&a,&b).unwrap() / ( VecOpp::size(&a) * VecOpp::size(&b) ); 
        let _can = a.len() == b.len();
        let d:Result<f64,String> = match _can {
            true  => Ok(c),
            false => Err("計算不可能です".to_string()),
        };
        d
    }
    
    //合計 
    pub fn sum(v: &Vec<f64>) -> f64{
        let mut _sum = 0.;
        for num in v {
            _sum += num;
        }
        _sum
    }
    //平均
    pub fn mean(v: &Vec<f64>) -> f64{
        let _sum    = VecOpp::sum(&v);
        let _length = v.len() as f64;
        let _mean = _sum/_length;
        _mean
    }
    //標準偏差
    pub fn var(v: &Vec<f64>) -> f64{
        let _var = VecOpp::cov(&v,&v);
        _var.sqrt()
    }
    //共分散
    pub fn cov(x: &Vec<f64>,y: &Vec<f64>) -> f64{
        let mut sxy = 0.;
        for i in 0..x.len() {
            sxy += (x[i] - VecOpp::mean(&x))*(y[i] - VecOpp::mean(&y));
        }
        sxy/x.len() as f64
     }
     //相関係数　
     pub fn cor(x: &Vec<f64>,y: &Vec<f64>) -> f64{
         let _cor  = VecOpp::cov(&x,&y) / (VecOpp::var(&x) * VecOpp::var(&y) );
         _cor
     }
}

struct Matrix {
    mat: Vec<Vec<f64>>,
    row: usize,
    col: usize
}

impl Matrix {

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
        let d:Result<Matrix,String> = match Matrix::is_same_size(&a,&b) { 
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
        for val in a {
            b.push( VecOpp::scl_mul(&k,&val) );
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
    pub fn rev(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let l = vec![vec![             1.0,                              0.0  ],
                     vec![ a[1][0]/a[0][0],                              1.0  ]
        ];
        let u = vec![vec![         a[0][0],                           a[0][1] ],
                     vec![             0.0, a[1][1] - a[1][0]*a[0][1]/a[0][0] ]
        ];

        let mut b: Vec<Vec<f64>> = vec![vec![0.0;a.len()];a.len()];
        b  
    }
}



//----------------------------ここからテストです---------------------------


#[cfg(test)]
mod vec_tests {
    use super::*;
    #[test]
    fn add_works() {
        let a = vec![1.0,2.0];
        let b = vec![3.0,4.5];
        let c = VecOpp::add(&a,&b).unwrap();
        assert_eq!(vec![4.0,6.5],c);
    }
    #[test]
    fn scl_ml_works() {
        let a = vec![1.0,2.0];
        let b = 5.;
        let c = VecOpp::scl_mul(&b,&a);
        assert_eq!(vec![5.0,10.0],c);
    }
    #[test]
    fn in_mul_works() {
        let a = vec![1.0,2.0];
        let b = vec![3.0,4.5];
        let c = VecOpp::in_mul(&a,&b).unwrap();
        let d = vec![1.0,0.0];
        let e = vec![0.0,1.0];
        let f = VecOpp::in_mul(&d,&e).unwrap();
        assert_eq!(&(12.0 as f64),&c);
        assert_eq!(&(0.0  as f64),&f);
    }
    #[test]
    fn out_mul_works() {
        let a = vec![1.0,2.0];
        let b = vec![3.0,4.5];
        let c = VecOpp::out_mul(&a,&b).unwrap();
        assert_eq!(&(-1.5 as f64),&c);
    }
    #[test]
    fn size_works() {
        let a = vec![1.0,2.0];
        let b = VecOpp::size(&a);
        let c = (5.0 as f64).sqrt();
        assert_eq!(c,b);
    }

    #[test]
    fn sin_bt_vec_works() {
        let x = vec![1.0,(3.0 as f64).sqrt()];
        let y = vec![1.0,0.0];
        let s = VecOpp::sin_bt_vec(&y,&x).unwrap();
        let z = (3.0 as f64).sqrt() / 2.0;
        assert_nearly_eq!(&s,&z);
    }

    #[test]
    fn cos_bt_vec_works() {
        let x = vec![1.0,(3.0 as f64).sqrt()];
        let y = vec![1.0,0.0];
        let s = VecOpp::cos_bt_vec(&y,&x).unwrap();
        let z = 0.5 as f64;
        assert_nearly_eq!(&s,&z);
    }
    #[test]
    fn sum_works() {
        let x = vec![1.0,2.0,1.0];
        assert_eq!(4.0,VecOpp::sum(&x));
    }
    #[test]
    fn mean_works() {
        let x = vec![1.0,2.0,1.0];
        assert_eq!(4.0/3.0,VecOpp::mean(&x));
    }
    #[test]
    fn var_works(){
        let a = vec![71.0,80.0,89.0];
        assert_nearly_eq!((54.0 as f64).sqrt(),VecOpp::var(&a));
    }
    #[test]
    fn cov_works() {
        let x: Vec<f64> = vec![50.0,60.0,70.0,80.0, 90.0];
        let y: Vec<f64> = vec![40.0,70.0,90.0,60.0,100.0];
        let z           = 220.0;
        assert_nearly_eq!(&z,VecOpp::cov(&x,&y));
    }
    #[test]
    fn cor_works() {
        let x: Vec<f64> = vec![50.0,60.0,70.0,80.0, 90.0];
        let y: Vec<f64> = vec![40.0,70.0,90.0,60.0,100.0];
        let z           = 0.728492796385774;
        assert_nearly_eq!(&z,VecOpp::cor(&x,&y));
    }
}

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
        assert!(Matrix::is_same_size(&_a,&_b));
        assert!(!Matrix::is_same_size(&_b,&_c));
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
        let _d = Matrix::add(&_b,&_c).unwrap();
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
        let _g = Matrix::add(&_d,&_e).unwrap();
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
        assert_eq!(_b,Matrix::scl_mul(&_k,&_a));
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
        assert_eq!(_c, Matrix::mul(&_a,&_b));

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
        assert_eq!(_f, Matrix::mul(&_d,&_e));
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
        assert_eq!(_b,Matrix::trans(&_a));
    }
    //#[test]
    pub fn rev_works() {
        let _a = vec![vec![ 3.0, 1.0],
                      vec![ 5.0, 1.0]
        ];
        let _b = vec![vec![-0.5, 0.5],
                      vec![ 2.5,-1.5]
        ];
        println!("{:?}",Matrix::mul(&_a,&_b));
        assert_eq!(&_b,&Matrix::rev(&_a));
    }
}

