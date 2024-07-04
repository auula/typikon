## 快速入门

如果想使用 typikon 程序，必须先将 typikon 程序安装到你的计算机中，安装过程中根据自己的计算机版本找到对应的二进制安装包，安装包可以再 [release](https://github.com/auula/typikon/releases) 页面中找到，并下载 typikon 的二进制压缩包到你计算机中。

> 在使用 typikon 程序之前请确保你计算机已经成功连接到国际互联网，如果是中国大陆地区可能会出现无法创建工作目录的情况，如果出现此类问题，可以尝试使用加速或者开启代理软件，再使用 typikon 进行目录初始化工作。

当下载好 zip 二进制软件包后，请使用 `unzip` 或者其他解压软件来解压 `typikon-darwin-x64-beta-v0.1.1.zip` 软件包，命令如下：

```shell
unzip typikon-darwin-x64-beta-v0.1.1.zip -d /usr/local/bin
chmod +x /usr/local/bin/typikon
```

通过手动解压二进制到指定目录中，如果你想要全局使用 typikon 程序，那么必须将 typikon 所在的路径添加到计算机操作系统的环境变量中，例如：

```shell
export TYPIKON="/usr/local/bin"
export PATH=$TYPIKON:$PATH
```

最后刷新环境变量，使 typikon 能被正常使用。

---

## 快速开始

在 typikon 程序中提供了 3 个子命令，分别为 `init` 用来初始化工作目录， `build` 用来构建生成静态文件， `help` 智能化子命令帮助信息提示，当 `typikon` 主程序运行起来后结果如下：

```shell
$: typikon
  _             _ _
 | |_ _  _ _ __(_) |_____ _ _
 |  _| || | '_ \ | / / _ \ ' \
  \__|\_, | .__/_|_\_\___/_||_|
      |__/|_|     v0.1.1-beta

Typikon lets you use markdown to write your online books.
GitHub: http://typikonbook.github.io  License: Apache2.0


Usage: typikon <command> [<args>...]

The commands are:

    init      Initialize to working directory
    build     Builder static html file and output to book

Use typikon help <command> for more information about a command.
```

在编写你的书籍资料之前，需要一个干净的目录作为 `typikon` 的工作目录，执行以下命令可以创建新的目录并且初始化为工作目录：

```shell
$: mkdir example && cd example && typikon init
```

当初始化完成之后，默认的工作目录就为 `example` 目录，在 `example` 目录中有几个关键的子目录和文件，我们只关注 `settings.yml` 文件和 `root.yml` 文件。在 `root.yml` 文件中编写 markdown 文件到书籍目录的映射关系，
例如下面：

```yaml
# Book root path mapping file
root:
  index: "index.md"
  chapters:
    - title: "English"
      index: "english.md"
      # Sub chapters root
      sub_chapters:
        - title: "Introduce"
          path: "introduce.md"
        - title: "Quick Start"
          path: "quickstart.md"
        - title: "Settings"
          path: "settings.md"
      # Sub chapters root
    - title: "简体中文"
      index: "chinese.md"
      sub_chapters:
        # Chapter content
        - title: "概述介绍"
          path: "introduce-zh.md"
        - title: "快速开始"
          path: "quickstart-zh.md"
        - title: "自定义设置"
          path: "settings-zh.md"

```

源 `markdown` 文件默认存储在 `example/book` 目录中，你目前所阅读这个页面就是通过此种方式生成渲染的。

---

### 部署文件

如何部署渲染的静态文件，默认被渲染成功之后的存储存储在 `example/docs` 目录中，只需要在将该目录上传到 GitHub Pages 和 Cloudflare Pages 中部署就可以通过域名访问，如果在本地测试，可以使用 `live-server` 程序提供本地静态服务器来访问，例如命令：

```shell
$: typikon build

  _             _ _
 | |_ _  _ _ __(_) |_____ _ _
 |  _| || | '_ \ | / / _ \ ' \
  \__|\_, | .__/_|_\_\___/_||_|
      |__/|_|     v0.1.1-beta

Typikon lets you use markdown to write your online books.
GitHub: http://typikonbook.github.io  License: Apache2.0

[INFO]   2024/07/04 00:43:24 💬 Clean up output directory "docs" successful
[INFO]   2024/07/04 00:43:24 💬 New create output directory "docs" successful
[INFO]   2024/07/04 00:43:24 💬 Building static assets "docs/assets" successful
[INFO]   2024/07/04 00:43:24 💬 Data written to file "docs/index.html" successful
[INFO]   2024/07/04 00:43:24 💬 Loading markdown file "book/english.md" successful
[INFO]   2024/07/04 00:43:24 💬 Loading markdown file "book/chinese.md" successful
[INFO]   2024/07/04 00:43:24 💬 Folder created "docs/english" successful
[INFO]   2024/07/04 00:43:24 💬 Data written to file "docs/english/index.html" successful
[INFO]   2024/07/04 00:43:24 💬 Folder created "docs/chinese" successful
[INFO]   2024/07/04 00:43:24 💬 Data written to file "docs/chinese/index.html" successful
[INFO]   2024/07/04 00:43:24 💬 Loading markdown file "book/introduce.md" successful
[INFO]   2024/07/04 00:43:24 💬 Loading markdown file "book/quickstart.md" successful
[INFO]   2024/07/04 00:43:24 💬 Loading markdown file "book/settings.md" successful
[INFO]   2024/07/04 00:43:24 💬 Loading markdown file "book/introduce-zh.md" successful
[INFO]   2024/07/04 00:43:24 💬 Loading markdown file "book/quickstart-zh.md" successful
[INFO]   2024/07/04 00:43:24 💬 Loading markdown file "book/settings-zh.md" successful
[INFO]   2024/07/04 00:43:24 💬 Data written to file "docs/english/introduce" successful
[INFO]   2024/07/04 00:43:24 💬 Data written to file "docs/english/quickstart" successful
[INFO]   2024/07/04 00:43:24 💬 Data written to file "docs/english/settings" successful
[INFO]   2024/07/04 00:43:24 💬 Data written to file "docs/chinese/introduce-zh" successful
[INFO]   2024/07/04 00:43:24 💬 Data written to file "docs/chinese/quickstart-zh" successful
[INFO]   2024/07/04 00:43:24 💬 Data written to file "docs/chinese/settings-zh" successful
[INFO]   2024/07/04 00:43:24 💬 Rendering of static resource files complete 🎉
```

目前 typikon 程序没有内置静态文件服务器，会在未来的新版本添加支持；
可以使用本地服务器 `live-server` 来测试生成静态网站：

```shell
$: live-server docs
Serving "docs" at http://127.0.0.1:8080
Ready for changes
```
 
访问 `http://127.0.0.1:8080` 就可以预览到网站。