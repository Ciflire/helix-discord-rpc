(define package-name 'helix-discord-rpc)
(define version "0.1.6")

(define dependencies '())

(define dylibs
  '((#:name "helix-discord-rpc"
     #:urls
     (
      (
       #:platform
       "x86_64-windows"
       #:url
       "https://github.com/nik-rev/ghost-text.hx/releases/download/v0.1.6/ghost_text.dll")
      (
       #:platform
       "x86_64-macos"
       #:url
       "https://github.com/nik-rev/ghost-text.hx/releases/download/v0.1.6/libghost_text.dylib")
      (
       #:platform
       "x86_64-linux"
       #:url
       "https://github.com/nik-rev/ghost-text.hx/releases/download/v0.1.6/libghost_text.so")))))
