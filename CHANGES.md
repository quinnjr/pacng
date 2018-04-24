# Change Log

## BREAKING CHANGES

### v0.1.0
1.  Removing POSIX-style subcommands.

__Breaks__: All previously devised `pacman` scripts which use
the POSIX-style subcommands.

__Intended Fix__: POSIX-style flags to alias known `pacman` subcommands.

__Reasoning__: As much as sticking to POSIX-style flags would be
better for the project, implementing an argparser and associated
help system is significantly more work than depending on `clap`,
the defacto standard crate for commands and subcommand parsing.

__Deprecation__: Expected around v2.0.x.

2.  Changing `pacman.conf` to `pacng.toml`

__Breaks__: Changes to pacman.conf will not be reflected in pacng.toml.

__Intended Fix__: Create a hook for `pacng` to move changes from `pacman.conf.new` to `pacng.toml.new` as needed.

__Reasoning__: `pacman.conf` is parsed in an INI-like fashion, which leaves much to be desired with standardization and ease of use with Rust. `pacng.toml` solves this issue by using [TOML](https://github.com/toml-lang/toml) standardized markup language for easier configuration and easier (de)serialization.

  Ex:
  ```ini
  [options]
  #RootDir     = /
  #DBPath      = /var/lib/pacman/
  #CacheDir    = /var/cache/pacman/pkg/
  #LogFile     = /var/log/pacman.log
  #GPGDir      = /etc/pacman.d/gnupg/
  #HookDir     = /etc/pacman.d/hooks/
  HoldPkg     = pacman glibc
  Architecture = auto

  # Misc options
  #UseSyslog
  Color
  #TotalDownload
  CheckSpace
  #VerbosePkgLists

  SigLevel    = Required DatabaseOptional
  LocalFileSigLevel = Optional
  #RemoteFileSigLevel = Required

  [core]
  Include = /etc/pacman.d/mirrorlist

  [extra]
  Include = /etc/pacman.d/mirrorlist

  [community]
  Include = /etc/pacman.d/mirrorlist
  ```

  ```toml
  [options]
  #RootDir     = "/"
  #DBPath      = "/var/lib/pacman/"
  #CacheDir    = "/var/cache/pacman/pkg/"
  #LogFile     = "/var/log/pacman.log"
  #GPGDir      = "/etc/pacman.d/gnupg/"
  #HookDir     = "/etc/pacman.d/hooks/"
  HoldPkg     = ["pacman", "glibc"]
  Architecture = "auto"

  # Misc options
  Misc = [
    #"UseSyslog"
    "Color"
    #"TotalDownload"
    "CheckSpace"
    #"VerbosePkgLists"
  ]

  SigLevel    = ["Required", "DatabaseOptional"]
  LocalFileSigLevel = ["Optional"]
  #RemoteFileSigLevel = ["Required"]

  [repositories.core]
  Include = "/etc/pacman.d/mirrorlist"

  [repositories.extra]
  Include = /etc/pacman.d/mirrorlist

  [repositories.community]
  Include = /etc/pacman.d/mirrorlist

  #[repositories.custom]
  #SigLevel = ["Optional", "TrustAll"]
  #Server = "file:///home/custompkgs"

  ```

3.  Removing customizable transfer command.

__Breaks__: Use of `wget`, `curl`, `aria2c`, etc. from use with `pacng`.

__Intended Fix__: :x:

__Reasoning__: Rust has perfectly good networking capabilities
built into the standard library and extended by other crates.
`GET` requests performed by the application are better off being
performed internally.

### v0.2.0

1. Removing use of `/etc/pacman.d/mirrorlist`

__Breaks__: Use of the same mirrorlist between `pacman` and `pacng`.

__Intended Fix__: :x:

__Reasoning__: Another move away from using INI-like files from
`pacman` in favor of TOML. Preferred since `pacng` handles
mirrorlist updating as a feature rather than relying on other
packages.
