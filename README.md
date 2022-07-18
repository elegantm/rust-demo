# rust-demo
[TOC]



## 学习rust

### rust 语言特点

​    每一行结尾都是分号;  
​    println! 为一个输出宏 ，感叹号结尾表示宏  
​    rust中变量默认是不可变的  
​    io::result 意味着有返回值 ok,ERR 枚举值  
​    {}为参数占位符  println!("You guessed: {}", guess);  


### rust优势
- 编译后的代码 和 c/c++ 性能相当，同时有出色的内存和能源效率

- 避免 70%的c/c++存在的安全问题，和绝大多数内存问题
- 强类型系统可以防止数据竞争，带来无畏并发等特性
- 无缝与 C 互操作，支持数十个平台
- 六年中最受喜爱的语言
- 现代工具：cargo(编译)  clippy(450 多种 lint) rustup(简化工具链管理)


### rust劣势
- 陡峭的学习曲线 编译器强制执行内存规则
- 在某些领域、目标平台(尤其是嵌入式) IDE 特性缺少 rust 库
- 比其他语言类似代码更长的编译时间
- 没有正式的语言规范，可能会阻止某些领域(航空、医疗)合法使用
- 粗心的使用 unsafe 库 可以偷偷破坏安全保证

### 关键字

| 关键字   | 含义         |
| -------- | ------------ |
| as       | 强制类型转换 |
| async    |              |
| await    |              |
| break    |              |
| const    |              |
| continue |              |
| crate    |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |
|          |              |

## 不同点
表达式 和语句的不同
表达式 结尾没有分号，返回当前的值
语句结尾有分号  不返回值
{
    let x = 3;
    x + 1
}
 上述表达式 返回了 x 的值