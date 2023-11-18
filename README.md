# etcd 版本历史查询工具

## 1.编译代码

编译需要提前安装 Rust 环境，可参考如下命令安装：

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后克隆项目并编译：

```shell
git clone https://github.com/monchickey/etcd-version-history.git
cd etcd-version-history
cargo build
```



## 2.运行工具

查看帮助：

```shell
./etcd-version-history --help
```

使用参数：

`--addrs` 指定 etcd 集群地址列表，多个使用逗号分割，例如：`10.1.0.10:2379,10.1.0.11:2379,10.1.0.12:2379`

`--user` 或 `-u` 指定 etcd 集群的访问用户，如果未开启认证则不需要指定。

`--password` 或 `-p` 指定 etcd 集群的访问密码，如果未开启认证则不需要指定。

`--keys` 指定要回溯版本的 key 列表，多个使用逗号分割，例如：`foo,bar`

如果 value 是二进制类型，则返回以 Base64 编码显示。

