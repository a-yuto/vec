#行列定義
vecの行列構造体Matrixは行列fieldのmatだけではなく、行fieldのrowと列fieldのcolを持っています。
これは行列の行や列の長さをよびだすとき、matrix.len()やmatrix[0].len()と書くよりmatrix.colやmatrix.rowと書いた方がわかりやすいと考えているからです。

#演算
##行列同士の演算
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


```
