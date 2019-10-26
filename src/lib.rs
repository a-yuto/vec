pub struct Opp;

impl Opp {
    pub fn add(a: &Vec<f64>,b: &Vec<f64>) -> Vec<f64> {
        let mut c: Vec<f64> = Vec::new();
        for i in 0..a.len() {
            c.push(a[i] + b[i]);
        }
        c
    }
}
