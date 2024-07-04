## Settings

The `settings.yml` file is the default configuration file for the Typikon program, where you can set the metadata information of the static website rendered by Typikon. This includes fields for SEO optimization like `description` and `keywords`. The `title` field is used to set the book name, and other fields are used to set author information.

```yaml
settings:
  about:
    title: Typikon Book
    author: Leon Ding
    description: Typikon name derived from Typikon Book, the a static website rendering tool similar to mdbook and hugo, but it focuses only on rendering markdown into an online book, and is easier to use than the other tools.
    language: en
    keywords: typikon,book,website,generator,static,html,css,js,theme,rust
```

In the `directory` node, you can set the default directory and folder for storing data files, which must be under the current directory. You can configure directory mappings as follows:

```yaml
  directory:
    theme: theme
    source: book
    output: docs
```

> All fields must be sub-nodes of the settings field.


---

## Theme

By default, the Typikon program supports custom theme files. Theme files are stored in the `theme` directory. You only need to modify the default `index.html` content to achieve the purpose of modifying its theme. The `assets` directory stores the theme's dependent resource files. It is not recommended to modify theme files by default, but if you have front-end skills, you can modify the files in this directory.

In the `settings.yml` file, there is a `theme` field related to the theme. By modifying the field, you can support other theme styles. You can customize CSS and JS files by placing them in the `assets` folder and adding the corresponding mappings as follows:


```yaml
    theme: typikon-theme

    custom_css:
        - /assets/css/bootstrap.min.css
        - /assets/sidebars.css

    custom_js:
        - /assets/js/bootstrap.min.js
        - /assets/js/bootstrap.bundle.js
```

If your website needs to track user visit data, such as Google Analytics for JS files, you can customize the configuration and include it in this way.





