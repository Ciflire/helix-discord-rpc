;; Builtin imports
(require-builtin steel/strings)

;; Cogs imports
(require "helix/editor.scm")

;; dylibs imports
(#%require-dylib "libhelix-discord-rpc"
  (only-in
    DiscordRPC::new
    DiscordRPC::connect))

(define server (DiscordRPC::new))

(register-hook! 'document-opened (lambda (doc-id) (DiscordRPC::connect server)))
