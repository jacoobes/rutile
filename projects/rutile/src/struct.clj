(ns lang.src.struct
  (:require 
    [instaparse.core :as insta :refer [defparser]]
    [lang.src.utils :refer [if-let*]]
    [lang.src.bytes :as bytes]
    [clojure.walk :as walk]))


(defn string-to-byte [string] 
  (map (comp byte int) string))

(defn pad [n coll val]
  (take n (concat coll (repeat val))))


; bytes will represent the bincode format
; we transform nodes into hashmaps with data.
(def transform-map {
    :string (fn [data] 
      (let [trimmed-str (subs data 1 (- (count data) 1))] 
        (hash-map 
          :data trimmed-str,
          :bytes (bytes/string trimmed-str))))
    :bool (fn [b] (hash-map 
                    :data (read-string b),
                    :bytes (bytes/bool b)))
    :number (fn [n] 
      (let [parsed-num (Double/parseDouble n)]
        (hash-map 
          :data  parsed-num
          :bytes (bytes/number parsed-num))))
    :function (fn [n] 
      (hash-map 
          :data 1
          :bytes (seq [0]))) 
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



