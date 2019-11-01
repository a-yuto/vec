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

