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

