use crate::mat_opp::Matrix;
use crate::nn::*;

#[cfg(test)]
mod unit_test{
    #[test]
    fn matrix_test(){
        let x = mat_opp::Matrix {
            mat: vec![vec![1.0,5.0]],
            row: 2,
            col: 1
        };
        let w1 = Matrix {
            mat: vec![vec![0.1,0.3,0.5],
                      vec![0.2,0.4,0.6]],
            row: 3,
            col: 2
        };
        let b1 = Matrix {
            mat: vec![vec![0.1,0.2,0.3]],
            row: 3,
            col: 1
        };
        let a1 = Matrix {
            mat: vec![vec![0.3,0.7,1.1]],
            row: 3,
            col: 1
        };
        assert_eq!(a1,&x*&w1+b1);
        
        let z1 = sigmoid(a1);
        let w2 = Matrix {
            mat: vec![vec![0.1,0.4],
                      vec![0.2,0.5],
                      vec![0.3,0.6]],
            row: 2,
            col: 3
        };
        let b2 = Matrix {
            mat: vec![vec![0.1,0.2]],
            row: 2,
            col: 1
        };
        let a2 = z1 * w2 + b2;
        
        let w3 = Matrix {
            mat: vec![vec![0.1,0.3],
                      vec![0.2,0.4]],
            row: 2,
            col: 2
        };
        let b3 = Matrix {
            mat: vec![vec![0.1,0.2]],
            row: 2,
            col: 1
        };
        let a3 = z2 * w3 + b3;
        let y  = identity(a3);
        let ans = Matrix {
            mat: vec![vec![0.3148278,0.6927909]],
            row: 2,
            col: 1
        };
    }

}
