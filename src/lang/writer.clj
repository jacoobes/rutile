(ns lang.writer)

; version 2.0
(def version (.getBytes "2.0.0"))


; const table 
; 4 bytes for length
(defn write-const-table [writer, tree] 
  
  )

; values are represented by 16 bytes.
; each value has disciminator and associated value.


(defn write-value [writer node] 
    
  
  )


