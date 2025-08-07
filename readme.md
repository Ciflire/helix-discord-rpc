# Requirements

You need to

- Install the [helix plugin fork](https://github.com/mattwparas/steel) 
- Have [steel](https://github.com/mattwparas/steel) installed
- It should install forge with it but just in case... You need forge install

# Installation

You can either clone this repository and run
```
forge install
```

Or you can just install from the git using
```
forge pkg install --git https://github.com/Ciflire/helix-discord-rpc.git
```

Then you need to add the plugin to your `init.scm`, i recommend that you prefix it.
```lisp
(require (prefix-in helix-discord-rpc. "helix-discord-rpc/helix-discord-rpc.scm"))
```

You may add

```
(discord-rpc-connect)
```
To you `init.scm` to set the activity when opening helix though i do not recommend it. There is an issue at the moment, multiple client cannot connect at the same and it takes some time to fail so do it at your own risk.

# Features

The idea is to match the features of [vscord](https://github.com/leonardssh/vscord) and to conserve the same configuration structures, options... to ease transitions if that somehow happens and to have some kind of guidelines.If it can be done in scheme it must be !

- [ ] Activity update
  - [ ] On workspace change
  - [x] On file change
- [ ] Language icons : some of them are supported for now, i'm just a bit lazy and don't want to look for all file extensions of all programming language
- [ ] Idle status
- [ ] Cursor Position
- [ ] Git status
- [ ] LSP workspace/file diagnostics
- [ ] Configuration ?

# Thanks

- Thanks to [vscord](https://github.com/leonardssh/vscord) for there assets
- Thanks to [presence.nvim](https://github.com/andweeb/presence.nvim) for giving the name to the project (hxcord was kind of a tongue twister)
