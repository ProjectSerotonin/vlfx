# vlfx

```vlfx``` is a tool to decrypt Fanatec steering wheel firmware files

# Usage

```text
$ vlfx.exe --help
```

```text
Tool to decrypt Fanatec steering wheel firmware files

USAGE:
    vlfx.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decrypt_firmware    Decrypts a Fanatec steering wheel firmware file
    help                Prints this message or the help of the given subcommand(s)
```

## Decrypt firmware mode

```
vlfx.exe decrypt_firmware --help
```
```
Decrypts a Fanatec steering wheel firmware file

USAGE:
    vlfx.exe decrypt_firmware --input <FILE> --output <FILE>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -i, --input <FILE>     The firmware file to decrypt
    -o, --output <FILE>    Where to output the decrypted firmware file
```