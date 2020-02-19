# shelf

`shelf` is a key-value store for the command line. It can be used to store
information that you frequently need access to when using the terminal, like
bookmarks, todo items, commands, snippets etc.

## Installation

Download the latest release from Github and add it to your `PATH`.

## Usage

Use `shelf help` for documentation.

The `add` command takes any number of arguments (minimum 3) and creates a mapping:

```sh
shelf add bookmarks "Rust documentation" "https://doc.rust-lang.org/std/index.html"
shelf add bookmarks "Ruby documentation" "https://ruby-doc.org/"
```

The `get` command read values or maps. When reading a value it just prints the
value:

```sh
shelf get bookmarks "Rust documentation"
# output
# https://doc.rust-lang.org/std/index.html
```

But when reading a map, it prints all keys and values separated by a `tab` character:

```sh
shelf get bookmarks
# output
# Rust documentation	https://doc.rust-lang.org/std/index.html
# Ruby documentation	https://ruby-doc.org/
```

## Cool Usage

You can easily build a fuzzy finder for bookmarks by combining it with
[skim](https://github.com/lotabout/skim) or [fzf](https://github.com/junegunn/fzf):

```sh
shelf get bookmarks | fzf | awk -F $'\t' '{print $2}'
```

## Storage

Data is stored in a yaml file placed according to the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html),
which often just means `~/.config/shelf/shelf.yml`. You may want to track this
file in your dotfiles. The serialization is stable so the diff should be sane.
