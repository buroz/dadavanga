### DadaVanga

## A cli app for dadaist poetry

Dadavanga is an experimental poetry cli app writen in rust. You can use your own wordlist.

Build it.

```sh
cargo build
```

Then run it!

```sh
./dadavanga --help
```

````sh
dadavanga 1.0
Burak OZ <buroz@nethole.dev>
A cli app for dadaist poetry

USAGE:
    dadavanga [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lines <lines>    Max lines for poem | DEFAULT = 4
    -f, --file <file>      Path of your wordlist
    -w, --words <words>    Max words value per line | DEFAULT = 5
````
