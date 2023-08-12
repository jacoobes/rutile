(ns lang.struct
  (:require 
    [instaparse.core :as insta :refer [defparser]]
    [lang.utils :refer [if-let*]]
    [clojure.walk :as walk]))

;(defn evaluate [node]
;  (if-let* [const (= (:tag node) :bool)]
;    (:content node)
;    node))

(defn boolstring-into [s tr fa] 
  (if (= s "true") tr fa))

(defn string-to-byte [string] 
  (map (comp byte int) string))

(def transform-map {
    :string (fn [data] (hash-map 
                         :data data,
                         :bytes (cons (byte 1) (string-to-byte data))))
    :bool (fn [b] (hash-map 
                    :data (boolstring-into b true false),
                    :bytes (seq [(byte 3) (byte (boolstring-into b 1 0))]))) 
 })

; Transform constant nodes into hashmaps using the transform-map
(def nodes-to-hashmap 
  (partial insta/transform transform-map))


; extracting constant nodes into a data structure that looks something 
; {
;  "sfasdfjkdsflkasdjflkd" { :bytes (1 2) } 
;   true  { :bytes (3, 1) }
;   false { :bytes (3, 0) }
; }

(defn target-fn [tree]  
  (tree-seq map? (fn [node] 
                   (if (contains? node :data) (seq [node]) (seq[]))) tree))

(defn const-table [tree]
  ( -> tree 
       nodes-to-hashmap))
; const table- turn tree into map of data 
