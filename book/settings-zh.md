## Settings

在 `settings.yml` 文件中存储是 typikon 程序默认配置文件，可以设置 typikon 渲染出来的静态网站元数据信息，例如方便 SEO 优化的 `description` 和 `keywords` 字段，`title` 用来设置书名，其他字段用于设置作者信息。

```yaml
settings:
  about:
    title: Typikon Book
    author: Leon Ding
    description: Typikon name derived from Typikon Book, the a static website rendering tool similar to mdbook and hugo, but it focuses only on rendering markdown into an online book, and is easier to use than the other tools.
    language: en
    keywords: typikon,book,website,generator,static,html,css,js,theme,rust
```

在 `directory` 节点下可以设置默认，数据文件存储位置的目录和文件夹，必须是当前目录下的，可以配置目录映射关系：

```yaml
  directory:
    theme: theme
    source: book
    output: docs
```


> 所有字段必须都是 `settings` 字段子节点。 

---

## Theme

默认 typikon 程序上支持主题文件自定义的，主题文件存储在 `theme` 目录中，你只需要修改默认的 `index.html` 内容就可以到达修改其主题的目的，`assets` 目录中存储是主题依赖资源文件。默认情况不建议去修改主题文件，如果你有前端基础可以对该目录中的文件进行修改。

在 `settings.yml` 文件有关于主题的 `theme` 字段，通过修改字段来支持其他主题样式，可以自定义 css 文件和 js 文件，只需要将其放入 `assets` 文件夹目录中，添加对应映射关系即可：

```yaml
    theme: typikon-theme

    custom_css:
        - /assets/css/bootstrap.min.css
        - /assets/sidebars.css

    custom_js:
        - /assets/js/bootstrap.min.js
        - /assets/js/bootstrap.bundle.js
```

如果你的网站需要统计用户访问数据，例如 Google Analytics 对 js 文件，可以通过此种方式进行自定义配置引入。