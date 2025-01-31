# Markdown Blog

# How it works

The program will convert every markdown file from 'articles' into HTML files
in the 'posts' directory.

`blog.html` will contain links to all of your posts with the possibility
to add more features in the future like *post description*, *published date* and more. 

# Website Structure

```bash
├── articles
│   ├── article_name1.md
│   ├── article_name2.md
│   ├── article_name3.md
├── posts (generated by MDB)
│   ├── post_name1.html
│   ├── post_name2.html
│   └── post_name3.html
├── index.html
└── styles
    ├── global.css
    ├── post_name1.css
    └── index.css
```

# Markdown rules

This parser doesn't support all features of Common Mark, instead it utilizes
a subset which fits all blogging needs.

Supported:
- Headers (1-6)
- Paragraphs
- Ordered / Unordered list (only via `- [text]` and `1. [text]`)
- Italics (only via `*text*`)
- Bold (only via `**text**`)
- Inline code
- Block code 
- Quotes (> only 1 level)
- Images (e.g. `![title](Images/example.png)`)
- Links (`[link](url)`)


# Build from source

```console
git clone git@github.com:R3ZV/mdb.git
cd mdb

zig build
```

The resulting binary can be either added to the PATH or if you
have a 'bin' folder for your scripts you can add it there.

```console
cp ./bin/mdb ~/bin/mdb
mdb help
```

Mdb also comes with it's own markdown preview so you can see how text
is parsed, you can launch a server via `mdb preview`.
