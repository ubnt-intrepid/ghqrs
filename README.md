# `ghq.rs`

`ghq.rs` is a CLI tools to manage cloned repositories from Git hostings, written in Rust.

## Synposis (work in progress)

```
ghqrs 0.1.1
Yusuke Sasaki <yusuke.sasaki.nuem@gmail.com>
Manages cloned repositories from Git hostings

USAGE:
    ghqrs.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    clone    Clone remote repository into your working directory
    help     Prints this message or the help of the given subcommand(s)
    list     List local repositories into the working directories
    root     Show repositories's root
```

## CI Status
| Travis CI | Appveyor | Wercker |
|:---------:|:--------:|:-------:|
| [![Build Status](https://travis-ci.org/ubnt-intrepid/ghqrs.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/ghqrs) | [![Build status](https://ci.appveyor.com/api/projects/status/92oteveaufy3ia4u?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/ghqrs) | [![wercker status](https://app.wercker.com/status/4ebe317cc9e62a5c45e83e6d04ecba8b/s/master "wercker status")](https://app.wercker.com/project/byKey/4ebe317cc9e62a5c45e83e6d04ecba8b) |

## License
This software is released under the MIT license. See [LICENSE](LICENSE) for details.
