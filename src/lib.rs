pub struct VecOpp;
pub struct MatOpp;
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
}


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
}

impl MatOpp {
    pub fn add(a: &Vec<Vec<f64>>,b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let _c = vec![VecOpp::add(&a[0],&b[0]).unwrap(),
                      VecOpp::add(&a[1],&b[1]).unwrap()
        ];
        _c
    }
    pub fn scl_mul(k: &f64,a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut b: Vec<Vec<f64>> = Vec::new();
        for val in a {
            b.push( VecOpp::scl_mul(&k,&val) );
        }
        b
    }
    
}
mod mat_tests {
    use super::*;

    #[test]
    pub fn add_works() {
        let _a = vec![vec![1.0,2.0],
                      vec![3.0,4.0]
        ];
        let _b = vec![vec![1.0,0.0],
                      vec![0.0,1.0]
        ];
        let _c = vec![vec![0.0,2.0],
                      vec![3.0,3.0]
        ];
        assert_eq!(_a,MatOpp::add(&_b,&_c));
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
        assert_eq!(_b,MatOpp::scl_mul(&_k,&_a));
    }
}

