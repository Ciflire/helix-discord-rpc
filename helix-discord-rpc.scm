;; Builtin imports
(require-builtin steel/strings)
(require-builtin steel/time)

;; Cogs imports
(require "helix/editor.scm")
(require "helix/misc.scm")
(require "helix/components.scm")

;; dylibs imports
(#%require-dylib "libhelix_discord_rpc"
  (only-in
    DiscordRPC::new
    DiscordRPC::connect
    DiscordRPC::set_activity
    DiscordRPC::set_idle))

(define server (DiscordRPC::new))
(define is-connected #false)
(define row 0)
(define col 0)

(define (get-cursor-row-col)
  (match (current-cursor)
    [#f ; checks whether if cursor is invisible?
      (set-status! "No primary cursor is visible")]
    [(list pos kind) ; when visible, it's a list of Position? & CursorKind (wtf is that? [normal/select/visual?])
      (set! row (position-row pos))
      (set! col (position-col pos))]))

(register-hook!
  'document-focus-lost
  (lambda
    (doc-id)
    (if
      is-connected
      (begin
        (get-cursor-row-col)
        (DiscordRPC::set_activity
          server
          (to-string (editor-document->path doc-id))
          (helix-find-workspace)
          row
          col)))))

;;@docs
; Connects the server to discord's websocket
(define (discord-rpc-connect) (
                               if
                               is-connected
                               "Websocket already connected"
                               (begin (DiscordRPC::connect server) (set! is-connected #true) "Websocket connected")))

(provide discord-rpc-connect)
