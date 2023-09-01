(ns lang.writer
  (:require [instaparse.core :as insta :refer [defparser]]
            [lang.bytes :refer [string->bytes]]))

; version 2.0
(def current-version (byte-array [(byte 50)
                                  (byte 48)
                                  (byte 48)]))
(defn version [writer] 
  (do (.write writer current-version)))

; const table 
; 2 bytes that displays the length of the const table 
(defn const-table [writer hmap]
  (let [hmap-size (count hmap)] 
    (do 
      (.write writer (int hmap-size))
      (doseq [[_ v] hmap] 
        (.write writer (byte-array (:bytes v)) ))  
      )))

; values are represented by 16 bytes.
; each value has disciminator and associated value.
(defn value [writer node] node)

