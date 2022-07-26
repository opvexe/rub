# Cargo web

```shell
cargo new rub && cd rub 
cargo new tcpserver
cargo new tcpclient
cargo run -p tcpserver
cargo run -p webservice --bin service1
cargo build --features some_condition
```

# Cargo JSON 序列化

- [https://www.bilibili.com/video/BV1RP4y1G7KF?p=7](https://www.bilibili.com/video/BV1RP4y1G7KF?p=7)


# [学习地址](https://learnku.com/articles/43146)


# 闭包带Move与不带Move区别

- 带move: 函数外与函数内的同一个变量不是同一个变量。
- 不带move闭包，函数外和函数内的同名变量是同一个变量

