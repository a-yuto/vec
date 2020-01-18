use nearly_eq::*;

trait VecOpp<T> {
    fn add(a: &Vec<T>,b: &Vec<T>) -> Result<Vec<T>,String>;
}
struct vec {
}
impl VecOpp<f64> for vec {
    fn add(a: &Vec<f64>,b: &Vec<f64>) -> Result<Vec<f64>,String>{
        let error = match a.len() > b.len() {
            true  => "fist arg length is longer is than second.",
            false => "second arg length is longet tham first.",
        };
        match a.len() == b.len() {
            true  => Ok(
                (0..a.len()).map(|i|
                    a[i] + b[i]
                ).collect()
            ),
            false => Err(error.to_string())
        }
    }
}
impl VecOpp<i64> for vec {
    fn add(a: &Vec<i64>,b: &Vec<i64>) -> Result<Vec<i64>,String>{
        let error = match a.len() > b.len() {
            true  => "fist arg length is longer is than second.",
            false => "second arg length is longet tham first.",
        };
        match a.len() == b.len() {
            true  => Ok(
                (0..a.len()).map(|i|
                    a[i] + b[i]
                ).collect()
            ),
            false => Err(error.to_string())
        }
    }
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
    use crate::vec_opp::*;
    #[test]
    fn add_works() {
        let int_a = vec![1,2];
        let int_b = vec![3,4];
        let int_ans = vec::add(&int_a,&int_b).unwrap();
        assert_eq!(vec![4,6],int_ans);
        let float_a   = vec![1.0,2.0];
        let float_b   = vec![3.0,4.5];
        let float_ans = vec::add(&float_a,&float_b).unwrap();
        assert_eq!(vec![4.0,6.5],float_ans);
    }
    #[test]
    fn scl_ml_works() {
        let int_a   = vec![3,4];
        let int_b   = 6;
        assert_eq!(vec![18,24],vec::scl_mul(int_b,&int_a));
        let float_a   = vec![1.0,2.0];
        let float_b   = 5.;
        let float_ans = vec::scl_mul(&b,&a);
        assert_eq!(vec![5.0,10.0],c);
    }
    #[test]
    fn in_mul_works() {
        let int_a = vec![3,2];
        let int_b = vec![2,6];
        assert_eq!(18,vec::in_mul(&int_a,&int_b).unwrap());
        let flaot_a = vec![1.0,2.0];
        let float_b = vec![3.0,4.5];
        let float_c = vec![1.0,0.0];
        let float_d = vec![0.0,1.0];
        assert_eq!(12.0,vec::in_mul(&float_a,&float_b).unwrap());
        assert_eq!( 0.0,vec::in_mul(&float_bc,float_d).unwrap());
    }
    #[test]
    fn out_mul_works() {
        let int_a = vec![3,5];
        let int_b = vec![2,4];
        assert_eq!(-2,vec::out_mul(&int_a,&int_b).unwrap());
        let float_a = vec![1.0,2.0];
        let float_b = vec![3.0,4.5];
        assert_eq!(-1.5,vec::out_mul(&a,&b).unwrap());
    }
    #[test]
    fn size_works() {
        //めんどくさいから全部floatで返す？
        assert_eq!(size(&vec![3,4]),5.0);
        assert_eq!(size(&vec![1.0,2.0],(5.0 as f64).sqrt());
    }

    #[test]
    fn sin_bt_vec_works() {
        let float_x = vec![1.0,(3.0 as f64).sqrt()];
        let float_y = vec![1.0,0.0];
        let float_s = sin_bt_vec(&y,&x).unwrap();
        let float_z = (3.0 as f64).sqrt() / 2.0;
        assert_nearly_eq!(&float_s,&float_z);
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

