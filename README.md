# A-Test-of-Code
AtCoder専用のRustコードテストツールです。

## Features
- Rust製CLI
- `rustc`コマンドによるソースコードビルド
- テストデータダウンロードを1-clickで行えるchrome拡張機能同梱
- テストデータダウンロードのために、ツールに認証情報を渡す必要は一切ありません
- zipファイルでダウンロードしたテストデータを`tar`コマンドで解凍

## Usage
0. 事前準備
  - 本ソースをリリースビルドしたexeファイルを、AtCoder用のrustプロジェクト配下にフォルダを切って格納する
  - 本拡張機能をインストールする
1. AtCoderの各問題ページにて、インストールした拡張機能をクリックし、ダウンロードフォルダにファイル名を変更せずにダウンロードする
2. exeファイルのあるパスから、コマンドにて`./A-Test-of-Code.exe`を実行
3. ダウンロードしたテストデータが同じフォルダ内に展開され、`../src/main.rs`のテスト結果がコンソール上に表示される

## Requirement
以下のコマンドが使用できること
- `tar`
- `rustc`

## Note
- 各フォルダの配置は固定です
- chrome用拡張機能には以下の[ライセンス](https://github.com/McbeEringi/petit)で引用しているコード部分があります
```
MIT License

Copyright (c) 2022 McbeEringi

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## License
MIT
