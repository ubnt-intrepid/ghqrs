# vcs_info.rs
A tiny CLI tool to show informative prompt string of version control systems, like Zsh's `vcs_info`.

## Usage

Zsh:
```zsh
autoload -Uz add-zsh-hook
autoload -Uz colors; colors

function __update_vcs_info {
  vcs_info_msg_0_="$(vcs_info 2>/dev/null || echo -n)"
}

add-zsh-hook precmd  __update_vcs_info
add-zsh-hook chpwd   __update_vcs_info

PROMPT='
%F{green}%~%f ${vcs_info_msg_0_}
$ '
```

Fish:
```fish
function fish_prompt
   echo (vcs_info)" % "
end
```

PowerShell:
```ps1
function prompt {
  write-host "$(pwd) " -nonewline
  write-host (vcs_info) -nonewline
  return "`n> "
}
```

Bash:
```bash
PS1='\w $(vcs_info) % '
```

## Installation

```shell-session
$ cargo install --git https://github.com/ubnt-intrepid/vcs_info.rs.git
```

## License
This software is released under the MIT license.
See [LICENSE](LICENSE) for details.

## Similar projects
* Zsh's `vcs_info`
* olivierverdier's [zsh-git-prompt](https://github.com/olivierverdier/zsh-git-prompt)
* Some integrations for PowerShell
  - dahlbyk's [posh-git](https://github.com/dahlbyk/posh-git)
  - JeremySkinner's [posh-hg](https://github.com/JeremySkinner/posh-hg)
  - imobile3's [posh-svn](https://github.com/imobile3/posh-svn)
* My [go-git-prompt](https://github.com/ubnt-intrepid/go-git-prompt)
