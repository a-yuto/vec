#[macro_use]
pub struct VecOpp;
extern crate matrix;
extern crate nearly_eq;
use nearly_eq::*;

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
     let c: f64 = out_mul(&a,&b).unwrap() / ( size(&a) * size(&b) );   
     let _can = a.len() == b.len();
     let d:Result<f64,String> = match _can {
         true  => Ok(c),
         false => Err("計算不可能です".to_string()),
     };
     d
}

pub fn cos_bt_vec(a: &Vec<f64>,b: &Vec<f64>) -> Result<f64,String> {
    let c: f64 = in_mul(&a,&b).unwrap() / ( size(&a) * size(&b) ); 
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
    let _sum    = sum(&v);
    let _length = v.len() as f64;
    let _mean = _sum/_length;
    _mean
}
//標準偏差
pub fn var(v: &Vec<f64>) -> f64{
    let _var = cov(&v,&v);
    _var.sqrt()
}
//共分散
pub fn cov(x: &Vec<f64>,y: &Vec<f64>) -> f64{
    let mut sxy = 0.;
    for i in 0..x.len() {
        sxy += (x[i] - mean(&x))*(y[i] - mean(&y));
    }
    sxy/x.len() as f64
}
//相関係数　
pub fn cor(x: &Vec<f64>,y: &Vec<f64>) -> f64{
    let _cor  = cov(&x,&y) / (var(&x) * var(&y) );
    _cor
}

#[cfg(test)]
mod vec_tests {
    use super::*;
    #[test]
    fn add_works() {
        let a = vec![1.0,2.0];
        let b = vec![3.0,4.5];
        let c = add(&a,&b).unwrap();
        assert_eq!(vec![4.0,6.5],c);
    }
    #[test]
    fn scl_ml_works() {
        let a = vec![1.0,2.0];
        let b = 5.;
        let c = scl_mul(&b,&a);
        assert_eq!(vec![5.0,10.0],c);
    }
    #[test]
    fn in_mul_works() {
        let a = vec![1.0,2.0];
        let b = vec![3.0,4.5];
        let c = in_mul(&a,&b).unwrap();
        let d = vec![1.0,0.0];
        let e = vec![0.0,1.0];
        let f = in_mul(&d,&e).unwrap();
        assert_eq!(&(12.0 as f64),&c);
        assert_eq!(&(0.0  as f64),&f);
    }
    #[test]
    fn out_mul_works() {
        let a = vec![1.0,2.0];
        let b = vec![3.0,4.5];
        let c = out_mul(&a,&b).unwrap();
        assert_eq!(&(-1.5 as f64),&c);
    }
    #[test]
    fn size_works() {
        let a = vec![1.0,2.0];
        let b = size(&a);
        let c = (5.0 as f64).sqrt();
        assert_eq!(c,b);
    }

    #[test]
    fn sin_bt_vec_works() {
        let x = vec![1.0,(3.0 as f64).sqrt()];
        let y = vec![1.0,0.0];
        let s = sin_bt_vec(&y,&x).unwrap();
        let z = (3.0 as f64).sqrt() / 2.0;
        assert_nearly_eq!(&s,&z);
    }

    #[test]
    fn cos_bt_vec_works() {
        let x = vec![1.0,(3.0 as f64).sqrt()];
        let y = vec![1.0,0.0];
        let s = cos_bt_vec(&y,&x).unwrap();
        let z = 0.5 as f64;
        assert_nearly_eq!(&s,&z);
    }
    #[test]
    fn sum_works() {
        let x = vec![1.0,2.0,1.0];
        assert_eq!(4.0,sum(&x));
    }
    #[test]
    fn mean_works() {
        let x = vec![1.0,2.0,1.0];
        assert_eq!(4.0/3.0,mean(&x));
    }
    #[test]
    fn var_works(){
        let a = vec![71.0,80.0,89.0];
        assert_nearly_eq!((54.0 as f64).sqrt(),var(&a));
    }
    #[test]
    fn cov_works() {
        let x: Vec<f64> = vec![50.0,60.0,70.0,80.0, 90.0];
        let y: Vec<f64> = vec![40.0,70.0,90.0,60.0,100.0];
        let z           = 220.0;
        assert_nearly_eq!(&z,cov(&x,&y));
    }
    #[test]
    fn cor_works() {
        let x: Vec<f64> = vec![50.0,60.0,70.0,80.0, 90.0];
        let y: Vec<f64> = vec![40.0,70.0,90.0,60.0,100.0];
        let z           = 0.728492796385774;
        assert_nearly_eq!(&z,cor(&x,&y));
    }
}

