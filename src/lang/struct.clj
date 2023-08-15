(ns lang.struct
  (:require 
    [instaparse.core :as insta :refer [defparser]]
    [lang.utils :refer [if-let*]]
    [clojure.walk :as walk]))


(defn boolstring-into [s tr fa] 
  (if (= s "true") tr fa))

(defn string-to-byte [string] 
  (map (comp byte int) string))

(def string-discrim 
 (byte-array [ (byte 2) (byte 0) (byte 0) (byte 0)]))
(def bool-discrim 
 (byte-array [ (byte 1) (byte 0) (byte 0) (byte 0)]))

; bytes will represent the bincode format
(def transform-map {
    :string (fn [data] (hash-map 
                         :data data,
                         :bytes (into string-discrim (string-to-byte data))))
    :bool (fn [b] (hash-map 
                    :data (boolstring-into b true false),
                    :bytes (into bool-discrim [ (byte (boolstring-into b 1 0)) ]))) 
  })


; Transform constant nodes into hashmaps using the transform-map
(def transform-consts
  (partial insta/transform transform-map))

(defn extract-nodes [node]
  "Flattens constant nodes into a sequence"
  (cond
    (map? node)
    (if (:data node)
      [node]
      (apply concat (map extract-nodes (:content node))))
    
    (coll? node)
    (apply concat (map extract-nodes node))
    :else []))  

; extracting constant nodes into a data structure that looks something 
; {
;  "sfasdfjkdsflkasdjflkd" { :bytes (1 2) ... } 
;   true  { :bytes (3, 1)  ... }
;   false { :bytes (3, 0) ... }
; }

(defn assoc-consts [li]
  (->> li 
    (map #(vector (% :data) %))
    (into {})))

(defn const-table [tree]
  ( -> tree 
       transform-consts
       extract-nodes
       assoc-consts))
; const table- turn tree into map of data 



