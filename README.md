# myls

これは2023年11月3日と4日の2日間かけて開催されたRust勉強会の演習課題の模範解答です。

## 実行例

```
$ ls
Cargo.lock      Cargo.toml      README.md       src             target
$ ./target/debug/myls 
Cargo.toml     target     Cargo.lock     README.md     src     
```

## 補足
`more-rusty`ブランチでは、よりRustっぽい書き方に`main.rs`修正しています。