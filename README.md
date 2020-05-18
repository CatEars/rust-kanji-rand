# rust-kanji-rand

A simple program in rust that reads you `$XDG_CONFIG_HOME` for
`rust-kanji-rand/config.json` and prints a kanji from
`config.json["kanji"][config.json["idx"]]`.


Example bashrc config:

```bash
RED='\e[91m'
RESET='\e[0m'
RUST_KANJI_RAND=rust-kanji-rand
KANJI_RAND=$RUST_KANJI_RAND
if [ "$color_prompt" = yes ]; then
    PS1="${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u@\h\[\033[00m\]\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]: $RED\$($KANJI_RAND)$RESET "
else
    PS1='${debian_chroot:+($debian_chroot)}\u@\h:\w\$ '
fi
```
