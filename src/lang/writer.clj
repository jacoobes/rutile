(ns lang.writer
  (:require [instaparse.core :as insta :refer [defparser]]))

; version 2.0
(def current-version (byte-array [(byte 50)
                          (byte 48)
                          (byte 48)]))
(defn version [writer] 
  (do (.write writer current-version)))

(defn string-to-byte [string] 
  (map (comp byte int) string))

; const table 
; 2 bytes that displays the length of the const table 
(defn const-table [writer hmap]
  (let [hmap-size (count hmap)] 
    (do (.write writer (int hmap-size))
        
        
        )))

; values are represented by 16 bytes.
; each value has disciminator and associated value.
(defn value [writer node] node)


