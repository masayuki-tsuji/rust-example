# rust-example

## 初期セットアップ

1. rustバージョン管理ツールの導入

    公式の導入手順は[こちら](https://www.rust-lang.org/ja/tools/install)

    ```bash
    brew install rustup-init
    ```

1. rustup-initの初期設定

    デフォルト設定でインストールする。

    ```bash
    rustup-init
    ```

1. シェルを再起動

    ```bash
    exec $SHELL -l
    ```

## 各種バージョン確認

- Rustバージョン管理

    ```bash
    rustup --version
    ```

- Rustコンパイラ

    ```bash
    rustc --version
    ```

- ビルド＆パッケージマネージャー

    ```bash
    cargo --version
    ```

## 新規でプロジェクトを作成する

  ```bash
  cargo new project
  ````

## 参考サイト

- [Rust公式](https://www.rust-lang.org/ja/)
- [日本語版THE BOOK](https://doc.rust-jp.rs/book-ja/)