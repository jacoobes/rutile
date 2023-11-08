(ns rutile.writer
  (:require [instaparse.core :as insta :refer [defparser]]
            [rutile.bytes :refer [string->bytes]]))

; header of bytecode = (byte 127) KGC 
; writes version of kongoc compiler
(def current-version (byte-array [(byte 127)
                                  (byte 75)
                                  (byte 71)
                                  (byte 67)
                                  (byte 50)]))
(defn version [writer] 
  (do (.write writer current-version)))

; const table 
; 2 bytes that displays the length of the const table 
(defn const-table [writer hmap]
  (let [hmap-size (count hmap)] 
    (do (.write writer (int hmap-size))
        (doseq [[_ v] hmap] 
        (.write writer (byte-array (:bytes v)) )))))


(defn bytecode [writer 
                tree
                const-hmap] 
    tree
)



