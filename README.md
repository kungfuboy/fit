Fit 语言

1. 使用 Rust 开发一个小语言——fit
2. Fit 的设计目的是用于很轻易地定义任意语法的 DSL，并且能够对输出进行后处理。

使用方式：

```bash
$ fit ./xyz.fit -o xyz
```
## 开发

#### 编译并运行
```bash
$ cargo run -- --name fit -p ./test/mid.fit
```

#### TODO 

- [ ] Lexer
- [ ] Parser
- [ ] count number