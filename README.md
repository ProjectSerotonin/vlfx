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
    decrypt_firmware    Decrypts a Fanatec steering wheel firmware file with the given key
    extract_key         Extracts the decryption key from FwClubSportBaseUpdater.exe
    help                Prints this message or the help of the given subcommand(s)
```

## Extract key mode

```
$ vlfx.exe extract_key --help
```
```
Extracts the decryption key from FwClubSportBaseUpdater.exe

USAGE:
    vlfx.exe extract_key --input <FILE> --output <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>
    -o, --output <FILE>     [default: fanatec_key.key]
```

## Decrypt firmware mode

```
vlfx.exe decrypt_firmware --help
```
```
Decrypts a Fanatec steering wheel firmware file with the given key

USAGE:
    vlfx.exe decrypt_firmware --input <FILE> --key <FILE> --output <FILE>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -i, --input <FILE>     The firmware file to decrypt
    -k, --key <FILE>       The key file to use
    -o, --output <FILE>    Where to output the decrypted firmware file
```