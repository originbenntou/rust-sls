# Rust severless



## Requirement

- yarn `^1.22`
- docker `^20.10`
- rust `~1.51`
  - rustc
  - rustup
  - cargo

## Installation

```
yarn install
```

```
docker build -t softprops/lambda-rust:1.51 https://github.com/softprops/lambda-rust.git#e6137ddbac36d104236407eb537c6c03a16a30fa
```

- serverless-rustは`softprops/lambda-rust`のstableイメージでランタイムが動くが、そのrustバージョンは1.4系（20211103時点）
  - しかし、1.4系だとlambda_httpの最新クレートを利用できずコンパイルエラーとなる
    - `softprops/lambda-rust`の最新コミットでイメージを作成 → 上記ビルドコマンド
  - また、serverless-rustはAL1が指定されているためエラーとなる
    - `softprops/serverless-rust`も最新コミットを利用する → package.json

## Usage

### Local

```
export SLS_DEBUG=*
yarn sls invoke local -f test-func --path test/resources/test_request.json 

{"statusCode":200,"headers":{"content-type":"application/json"},"multiValueHeaders":{"content-type":["application/json"]},"body":"{\"response\":\"Test\"}","isBase64Encoded":false}
```

### Test

```
cargo test
```

### Deploy

```
 yarn sls deploy --profile [aws-profile]
```

## 参考資料

- [RustとLambdaの相性が良い7つの理由 〜RustでLambdaをやっていく〜](https://zenn.dev/hinastory/articles/b603b76bf01ccc#fn3)
- [【AWS】RustでServerlessFrameworkを使ってみる](https://qiita.com/hisayuki/items/b4f8b21986468c34be02)
- [serverless-rustプラグインでRustアプリをデプロイするためのworkaround
  ](https://zenn.dev/ryosukeeeee/articles/rust-serverless-framework)
