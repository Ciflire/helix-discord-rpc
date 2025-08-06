# Requirements

You need to

- Install the [helix plugin fork](https://github.com/mattwparas/steel) 
- Have [steel](https://github.com/mattwparas/steel) installed
- It should install forge with it but just in case... You need forge install

# Installation

It is not possible to install from git as of now, or at least it has not been tested. First clone the repo, then go inside the folder and run
```
  forge install
```

Then you need to add the plugin to your `init.scm`, i recommend that you prefix it.
```sheme
(require (prefix-in helix-discord-rpc. "helix-discord-rpc/helix-discord-rpc.scm"))
  
```

The server will auto-start and the activity will be setup once you OPEN a file.

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

