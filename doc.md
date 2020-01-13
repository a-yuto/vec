# 行列定義
vecの行列構造体Matrixは行列fieldのmatだけではなく、行fieldのrowと列fieldのcolを持っています。
これは行列の行や列の長さをよびだすとき、matrix.len()やmatrix[0].len()と書くよりmatrix.colやmatrix.rowと書いた方がわかりやすいと考えているからです。

# 演算
## 行列同士の演算
vecでは行列同士の演算は演算子で以下のように実装します。

```

//aとbは行列
let plus  = &a + &b;
let sub   = &a - &b;
let mul   = &a * &b;
 
```

また、関数では以下のように実装します。

```

let plus = add_mat(&a,&b).unwrap();
let sub  = sub_mat(&a,&b).unwrap();
let mul  = mul_mat(&a,&b).unwrap();

```

四則演算を関数で実装する場合、Result<>型が返されるので、上記例のようにunwrap()関数を用いるか、matchを用いてください。

## 関数とスカラーの演算

行列とスカラーの演算は以下のように実装します。

```

//aは行列kはスカラー
let plus = &a + &k;
let sub  = &a - &k;
let mul  = &a * &k;

```

行列の各要素とkが計算された行列を返します。
また、演算の左側が行列で右側がスカラーでなければならないことに注意です。（これは実装上の問題で修正したいと考えています。）

# 関数
## matrix_test(&Matrix,&Matrix)
この関数はvecの形式の行列をtestする関数です。
Rustの一般的なtest関数であるassert_eq()では煩雑な書き方になってしまうでこのような関数を実装しました。

```
let test = zero(3,3);
let ans  = Matrix {
        mat: vec![vec![0.0,0.0,0.0],
                  vec![0.0,0.0,0.0],
                  vec![0.0,0.0,0.0]],
        row: 3,
        col: 3

};
matrix_test(&test,&ans);
//matrix_test()を使わないと以下のように書かなければなりません。
//assert_eq!(test.mat,ans.mat);
//assert_eq!(test.col,ans.col);
//assert_eq!(test.row,ans.row);
```

## get_row(usize,&Matrix) -> Result<Matrix,&str>
この関数は行列の指定された行を取り出した行列を返します。
0-indexedです。

```
let _a = Matrix{
        mat: vec![vec![-1.0, 2.0],
                  vec![ 0.0, 3.0],
                  vec![ 1.0,-2.0],
                  vec![-1.0, 1.0],
                  vec![ 2.0,-3.0]],
        row: 2,
        col: 5
};
let _b = Matrix{
        mat: vec![vec![0.0,3.0]],
        row: 2,
        col: 1
};
let _c = get_row(2,&_a).unwrap();
matrix_test(&_c,&_b);
```

## get_col(usize,&Matrix) -> Result<Matrix,&str> 
この関数は行列の指定された列を取り出した行列を返します。
0-indexedです。

```
let _a = Matrix{
        mat: vec![vec![-1.0, 2.0],
                  vec![ 0.0, 3.0],
                  vec![ 1.0,-2.0],
                  vec![-1.0, 1.0],
                  vec![ 2.0,-3.0]],
        row: 2,
        col: 5
};
let _b = Matrix{
        mat: vec![vec![ 2.0],
                  vec![ 3.0],
                  vec![-2.0],
                  vec![ 1.0],
                  vec![-3.0]],
        row: 1,
        col: 5
};
let _c = get_col(2,&_a).unwrap();
matrix_test(&_c,&_b);
```

## zero(usize,usize) -> Matrix
この関数は指定された大きさの零行列を返します。

```
let test = zero(3,3);
let ans  = Matrix {
          mat: vec![vec![0.0,0.0,0.0],
                    vec![0.0,0.0,0.0],
                    vec![0.0,0.0,0.0]],
          row: 3,
          col: 3

};
matrix_test(&test,&ans);
```

## iden(usize) -> Matrix 
この関数は指定された大きさの単位行列を返します。

```
let test = iden(4);
let ans  = Matrix {
        mat: vec![vec![1.0,0.0,0.0,0.0],
                  vec![0.0,1.0,0.0,0.0],
                  vec![0.0,0.0,1.0,0.0],
                  vec![0.0,0.0,0.0,1.0]],
        row: 4,
        col: 4
};
matrix_test(&test,&ans);
```

## is_same_size(&Matrix,&Matrix) -> bool 
行列が同じ大きさかどうかを判定します。

```
matrix_test(&iden(3),&zero(3));
```

## can_mul(&Matrix,&Matrix) -> bool 
掛け算可能かを判定します。具体的には第一引数の行と第二引数の列が等しいかを判定します。

```
let _a = Matrix{
        mat: vec![vec![ 1.0, 2.0, 4.0, 5.0, 1.0],
                  vec![ 0.0, 2.0, 1.0, 1.0, 3.0],
                  vec![ 3.0, 0.0, 2.0, 0.0, 1.0]],
        row: 5,
        col: 3
};
let _b = Matrix{
        mat: vec![vec![-1.0, 2.0],
                  vec![ 0.0, 3.0],
                  vec![ 1.0,-2.0],
                  vec![-1.0, 1.0],
                  vec![ 2.0,-3.0]],
        row: 2,
        col: 5
};
let _c = Matrix{
        mat: vec![vec![ 0.0, 2.0],
                  vec![ 6.0,-4.0],
                  vec![ 1.0,-1.0]],
        row: 2,
        col: 3
};
assert!(can_mul(&_a,&_b));
assert!(!can_mul(&_b,&_c));

```

## trans(&Matrix) ->Matrix
引数の転地行列を返します。

```
let _a = Matrix{
        mat: vec![vec![ 1.0, 3.0,-2.0],
                  vec![ 4.0, 5.0, 2.0]],
        row: 3,
        col: 2
};
let _b = Matrix{
        mat: vec![vec![ 1.0, 4.0],
                  vec![ 3.0, 5.0],
                  vec![-2.0, 2.0]],
        row: 2,
        col: 3
};
let _c = trans(&_a);
matrix_test(&_b,&_c);
```

## split(&Matrix,usize) -> Result<Matrix,String>
引数の行列から第一列と第一行を取り除いた行列を返します。主にLU分解の実装に使用します。
```
let _a = Matrix{
        mat: vec![vec![ 8.0,16.0,24.0,32.0],
                  vec![ 2.0, 7.0,12.0,17.0],
                  vec![ 6.0,17.0,32.0,59.0],
                  vec![ 7.0,22.0,46.0,105.0]],
        row: 4,
        col: 4
};
let _b = Matrix
         mat: vec![vec![ 7.0,12.0,17.0],
                   vec![17.0,32.0,59.0],
                   vec![22.0,46.0,105.0]],
         row: 3,
         col: 3
};
assert_eq!(&_b.mat,&split(&_a,1).unwrap().mat)
```

## lu(&Matrix) -> (Result<Matrix,String>,Result<Matrix,String>)
引数の行列をLU分解した２つの行列を返します。LU分解とは以下のようなものです。

```
A = L^{\-1} * U{\-1}
Lは下三角行列、Uは上三角行列。
```

## is_tri(&Matrix) -> bool
引数の行列が三角行列かどうかを判定します。

```
let _a = Matrix{
            mat: vec![vec![ 3.0, 1.0],
                      vec![ 5.0, 1.0]],
            row: 2,
            col: 2
};
let _l = Matrix {
            mat: vec![vec![3.0,0.0],
                      vec![5.0,1.0]],
            row: 2,
            col: 2
}
assert!(!is_tri(&_a));
assert!(is_tri(&_l));
```

## mat_print(&Matrix)
行列が出力されます。

## reduct(&Matrix) -> Matrix
行列式を等しくしたまま、行列を小さくするアレ（）。

```
let a = Matrix{
            mat: vec![vec![4.0,3.0,2.0,1.0],
                      vec![1.0,4.0,3.0,2.0],
                      vec![2.0,1.0,4.0,3.0],
                      vec![3.0,2.0,1.0,4.0]],
            col: 4,
            row: 4
        };
let b = Matrix{
        mat: vec![vec![13.0/4.0,5.0/2.0,7.0/4.0],
                  vec![-1.0/2.0,    3.0,5.0/2.0],
                  vec![-1.0/4.0,-1.0/2.0,13.0/4.0]],
        col: 3,
        row: 3
    };
    matrix_test(&reduct(&a),&b);
}
``` 

## connect(&mut Matrix,&mut Matrix) -> Result<Matrix,String>
行列の結合を行う。

```
let mut a: Matrix = Matrix {
        mat: vec![vec![1.0,2.0],
                  vec![3.0,4.0]],
        row: 2,
        col: 2
};
let b: Matrix = Matrix {
        mat: vec![vec![1.0,2.0,1.0,0.0],
                  vec![3.0,4.0,0.0,1.0]],
        row: 4,
        col: 2
};
let mut e = iden(2);
matrix_test(&connect(&mut a,&mut e).unwrap(),&b);

```
