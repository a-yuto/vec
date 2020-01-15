extern crate matrix;
extern crate nn;

#[cfg(test)]
mod unit_test(){
    #[test]
    fn matrix_test(){
        let x = matrix{
            mat: vec![vec![1.0,5.0]],
            row: 2,
            col: 1
        };
        let w1 = matrix {
            mat: vec![vec![0.1,0.3,0.5],
                      vec![0.2,0.4,0.6]],
            row: 3,
            col: 2
        };
        let b1 = matrix {
            mat: vec![vec![0.1,0.2,0.3]],
            row: 3,
            col: 1
        };
        let a1 = matrix {
            mat: vec![vec![0.3,0.7,1.1]],
            row: 3,
            col: 1
        };
        assert_eq!(a1,&x*&w1+b1);
    }

}
