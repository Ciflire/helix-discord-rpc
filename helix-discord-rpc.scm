;; Builtin imports
(require-builtin steel/strings)
(require-builtin steel/time)

;; Cogs imports
(require "helix/editor.scm")

;; dylibs imports
(#%require-dylib "libhelix_discord_rpc"
  (only-in
    DiscordRPC::new
    DiscordRPC::connect
    DiscordRPC::set_activity
    DiscordRPC::set_idle))

(define server (DiscordRPC::new))
(define is-connected #false)

(register-hook! 'document-focus-lost (lambda (doc-id) (if is-connected (begin (DiscordRPC::set_activity server (to-string (editor-document->path doc-id)) (helix-find-workspace))))))

;;@docs
; Connects the server to discord's websocket
(define (discord-rpc-connect) (
                               if
                               is-connected
                               "Websocket already connected"
                               (begin (DiscordRPC::connect server) (set! is-connected #true) "Websocket connected")))

(provide discord-rpc-connect)
