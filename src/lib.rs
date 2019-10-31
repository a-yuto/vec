pub struct Opp;
extern crate nearly_eq;
use nearly_eq::*;

impl Opp {
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
        let c: f64 = Opp::out_mul(&a,&b).unwrap() / ( Opp::size(&a) * Opp::size(&b) );
        
        let _can = a.len() == b.len();
        let d:Result<f64,String> = match _can {
            true  => Ok(c),
            false => Err("計算不可能です".to_string()),
        };
        d
    }

    pub fn cos_bt_vec(a: &Vec<f64>,b: &Vec<f64>) -> Result<f64,String> {
        let c: f64 = Opp::in_mul(&a,&b).unwrap() / ( Opp::size(&a) * Opp::size(&b) );
        
        let _can = a.len() == b.len();
        let d:Result<f64,String> = match _can {
            true  => Ok(c),
            false => Err("計算不可能です".to_string()),
        };
        d
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_works() {
        let a = vec![1.0,2.0];
        let b = vec![3.0,4.5];
        let c = Opp::add(&a,&b).unwrap();
        assert_eq!(vec![4.0,6.5],c);
    }
    #[test]
    fn scl_ml_works() {
        let a = vec![1.0,2.0];
        let b = 5.;
        let c = Opp::scl_mul(&b,&a);
        assert_eq!(vec![5.0,10.0],c);
    }
    #[test]
    fn in_mul_works() {
        let a = vec![1.0,2.0];
        let b = vec![3.0,4.5];
        let c = Opp::in_mul(&a,&b).unwrap();
        let d = vec![1.0,0.0];
        let e = vec![0.0,1.0];
        let f = Opp::in_mul(&d,&e).unwrap();
        assert_eq!(&(12.0 as f64),&c);
        assert_eq!(&(0.0  as f64),&f);
    }
    #[test]
    fn out_mul_works() {
        let a = vec![1.0,2.0];
        let b = vec![3.0,4.5];
        let c = Opp::out_mul(&a,&b).unwrap();
        assert_eq!(&(-1.5 as f64),&c);
    }
    #[test]
    fn size_works() {
        let a = vec![1.0,2.0];
        let b = Opp::size(&a);
        let c = (5.0 as f64).sqrt();
        assert_eq!(c,b);
    }

    #[test]
    fn sin_bt_vec_works() {
        let x = vec![1.0,(3.0 as f64).sqrt()];
        let y = vec![1.0,0.0];
        let s = Opp::sin_bt_vec(&y,&x).unwrap();
        let z = (3.0 as f64).sqrt() / 2.0;
        assert_nearly_eq!(&s,&z);
    }

    #[test]
    fn cos_bt_vec_works() {
        let x = vec![1.0,(3.0 as f64).sqrt()];
        let y = vec![1.0,0.0];
        let s = Opp::cos_bt_vec(&y,&x).unwrap();
        let z = 0.5 as f64;
        assert_nearly_eq!(&s,&z);
    }
}
