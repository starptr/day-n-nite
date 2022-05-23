# day-n-nite

Instantly toggle light-dark theme scheme in any OS.

## Goals
- toggle light-dark theme with 1 command
- install default (day-n-nite) configs
- add/update/remove arbitrary scripts or day-n-nite-formatted configs

## Example Data Format
```toml
[[configs]]
path = "~/.config/alacritty/alacritty.yml" # configs should be home-relative unless specified as absolute
type = "config"
hash = "md5hashstringhere"

[[configs]]
path = "~/.config/nvim/theme.sh"
type = "script"
hash = "md5hashstringhere"
```
- this data should have `600` rwx perms
