# Requirement

- yarn ^1.22
- docker ^20.10
- rust ~1.51
  - rustc
  - rustup
  - cargo

# Installation

```
yarn install

# serverless-rustのデフォルトは20211103時点ではrust1.4系でしかビルドしない
# 1.4系だと最新のlambda_httpクレートを利用できないので、最新コミットのimageを利用
# いずれdockerhubのstableがきっと更新されるハズ
docker build -t softprops/lambda-rust:1.51 https://github.com/softprops/lambda-rust.git#e6137ddbac36d104236407eb537c6c03a16a30fa
```

# Usage

## Local

```
export SLS_DEBUG=*
yarn sls invoke local -f test-func --path test/resources/test_request.json 

{"statusCode":200,"headers":{"content-type":"application/json"},"multiValueHeaders":{"content-type":["application/json"]},"body":"{\"response\":\"Test\"}","isBase64Encoded":false}
```

## Test

```
cargo test
```

## Deploy
