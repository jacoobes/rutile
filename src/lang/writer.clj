(ns lang.writer
  (:require [instaparse.core :as insta :refer [defparser]]))

; version 2.0
(def version (byte-array [(byte 50)
                          (byte 48)
                          (byte 48)]))

(defn string-to-byte [string] 
  (map (comp byte int) string))

; const table 
; 2 bytes that displays the length of the const table 
(defn const-table [writer, tree] 
  (insta/transform {
    :string (fn [data] (cons (byte 1) (string-to-byte data)))
    :bool (fn [b] (byte-array [3 (byte (if (= b "true") 1 0))]) )
  } tree))

; values are represented by 16 bytes.
; each value has disciminator and associated value.


(defn value [writer node] )


