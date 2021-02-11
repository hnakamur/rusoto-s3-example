rusot-s3-example
================

[オブジェクトストレージ | さくらのクラウド ドキュメント](https://manual.sakura.ad.jp/cloud/manual-objectstorage.html) の API でオブジェクトを1つ取得してオブジェクトのボディを標準出力にプリントするサンプルです（テキスト形式のオブジェクトを想定しています）。

[rusoto/rusoto: AWS SDK for Rust](https://github.com/rusoto/rusoto) を使って Rust で書いています。

## credential file

[オブジェクトストレージ サービス基本情報](https://manual.sakura.ad.jp/cloud/objectstorage/about.html) の
[ご利用手順](https://manual.sakura.ad.jp/cloud/objectstorage/about.html#id23) に従って
アクセスキー ID とシークレットアクセスキーを発行し下記の値を書き換えて
`~/.aws/credentials` に保存します（~/.aws ディレクトリが無い場合は作成）。

```
[default]
aws_access_key_id = _YOUR_SAKURA_OBJECT_STORAGE_ACCESS_KEY_ID_HERE_
aws_secret_access_key = _YOUR_SAKURA_OBJECT_STORAGE_SECRET_ACCESS_KEY_HERE_
```
