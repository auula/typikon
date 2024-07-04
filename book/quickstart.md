## Quick Start

To use the Typikon program, you must first install Typikon on your computer. During the installation process, download the appropriate binary package for your computer's version from the [release page](https://github.com/auula/typikon/releases). Once you have downloaded the Typikon binary zip package to your computer:

> Before using Typikon, ensure that your computer is successfully connected to the international internet. If you are in mainland China, you may encounter issues creating a working directory. If this happens, try using acceleration or enabling proxy software before using Typikon to initialize the directory.

After downloading the zip binary package, unzip it using `unzip` or another decompression software. For example:

```shell
unzip typikon-darwin-x64-beta-v0.1.1.zip -d /usr/local/bin
chmod +x /usr/local/bin/typikon
```

Then manually extract the binaries to the specified directory. If you want to use the Typikon program globally, you must add the path where Typikon is located to the environment variables of your computer's operating system. For example:

```shell
export TYPIKON="/usr/local/bin"
export PATH=$TYPIKON:$PATH
```

Remember to refresh the environment variables to ensure that Typikon can be used properly.

---

## Getting Started

Typikon provides three main commands: `init` to initialize the working directory, `build` to generate static HTML files, and `help` to provide command-specific help information. When you run the typikon main program, it will display the following:

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

Before you start writing your book content, you need a clean directory to serve as the Typikon working directory. Execute the following command to create a new directory and initialize it:


```shell
$: mkdir example && cd example && typikon init
```

After initialization, the default working directory is example. Inside example, you will find several key subdirectories and files, but the most important are `settings.yml` and `root.yml`. In `root.yml`, you map your Markdown files to chapters in your book directory. For example:


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

Markdown files are stored by default in the `example/book` directory. The page you are currently reading is generated and rendered using this method.

---

### Deployment

To deploy the rendered static files, which are by default stored in the `example/docs` directory after successful rendering, follow these steps to host them on `GitHub Pages` or `Cloudflare Pages`. If testing locally, you can use the live-server program to serve the static files locally.

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


If you want to preview your website locally before deployment, use `live-server` to set up a local static server. Here's how to run `live-server` :


```shell
$: live-server docs
Serving "docs" at http://127.0.0.1:8080
Ready for changes
```
 
Navigate to `http://127.0.0.1:8080` in your web browser to preview your website locally.

This setup allows you to deploy your static website to a hosting service or test it locally using live-server for immediate feedback.






