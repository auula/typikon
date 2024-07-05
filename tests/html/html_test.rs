use std::path::Path;

use typikon::html::{from_markdown, Hypertext};

#[test]
fn test_hypertext_create() {
    let src_path = Path::new("tests/html/chapter_1.1.0.md");
    let md = typikon::html::from_markdown(src_path).unwrap();

    let html = Hypertext::new("Test Title", "tests/html/index.html", md);

    println!("{:?}", html.to_html());
}

#[test]
fn test_hypertext_to_html() {
    // Example usage:
    let path = Path::new("tests/html/chapter_1.1.0.md");
    match from_markdown(&path) {
        Ok(markdown) => {
            println!("HTML output:");
            println!("{}", markdown.to_html());
            assert!(true)
        }
        Err(err) => {
            eprintln!("Error reading markdown file: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn test_md_to_html() {
    let markdown_input = r#"
### MySQL 8.x 安装和配置

在 MySQL 8.x 中引入了许多新特性和改进，包括 JSON 和 JavaScript 作为数据库查询语言，以及更好的性能和安全性。例如，在连接过程中出现的 `Public Key Retrieval is not allowed` 错误提示，这些都是新版本的特性。本文将记录在最新版本的 Ubuntu 中安装 MySQL 8.x 的完整过程。

---

### 安装和配置

在安装 MySQL 8.x 之前，可以先更新操作系统和软件包：

```sh
sudo apt update && apt upgrade -y
```

更新完成后，可以执行以下命令来查找和安装 MySQL 8.x 软件包：

```sh
sudo apt search mysql
sudo apt install mysql-*
```

安装完成后，使用 `sudo mysql_secure_installation` 初始化 root 用户的密码：

```sh
sudo mysql_secure_installation
```

如果需要允许远程用户访问 MySQL 服务器，可以修改配置文件 `/etc/mysql/mysql.conf.d/mysqld.cnf` 中的 `bind-address`，设置为 `0.0.0.0`：

```ini
bind-address = 0.0.0.0
mysqlx-bind-address = 127.0.0.1
```

然后可以为远程用户授予访问权限：

```sql
GRANT ALL PRIVILEGES ON *.* TO 'root'@'%' IDENTIFIED WITH mysql_native_password BY '新密码' WITH GRANT OPTION;
```

---

### 解决安全连接问题

在 MySQL 8.x 中，如果出现 `Public Key Retrieval is not allowed` 错误提示，可以执行以下命令来解决：

```sql
ALTER USER 'root'@'%' IDENTIFIED WITH mysql_native_password BY '新密码';
```

执行后，刷新授权表：

```sql
flush privileges;
```

最后，请确保允许服务器上的 `3306` 端口连接。


## 列表示例

- 项目1
- 项目2
  - 子项目1
  - 子项目2

## 插入普通图片

![java](https://img.ibyte.me/470eor.jpg)

## 引用示例

> 这是一个引用示例，可以包含多行文本和**加粗**文字。

## 表格示例

| 姓名   | 年龄 | 城市   |
|--------|------|--------|
| Alice  | 25   | 北京   |
| Bob    | 30   | 上海   |
| Carol  | 28   | 广州   |
"#;

    // 将 Markdown 转换为 HTML
    let html_output = typikon::html::Markdown::new(markdown_input);
    // 打印 HTML 输出
    println!("{:?}", html_output.to_html());

    assert!(true);
}
