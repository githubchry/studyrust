# studyrust



## Rust crates 国内镜像源加速配置

```shell
sudo vim ~/.cargo/config
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

[net]
git-fetch-with-cli = true

```

