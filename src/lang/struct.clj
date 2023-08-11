(ns lang.struct
  (:require [instaparse.core :as insta :refer [defparser]]))


(defn const-table []
  (hash-map :true 1, :false 0, ) 
  )
; const table- turn tree into map of data 
