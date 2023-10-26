(ns rutile.bytes)
(import '[java.nio ByteBuffer])

(def NULL (byte 0))

(def number-discrim [(byte 0) NULL NULL NULL])

(def string-discrim [(byte 2) NULL NULL NULL])

(def bool-discrim [(byte 1) NULL NULL NULL])


(defn pad [n coll val]
  (take n (concat coll (repeat val))))

(defn string->bytes [string] 
  (map (comp byte int) string))

(defn bool->bytes [data] 
  (if (= data "true") (byte 1) NULL))

(defn number->bytes [data]
  (let [buf (ByteBuffer/allocate (Double/BYTES))]
    (do (.putDouble buf data))
    (.array buf)))

; https://github.com/bincode-org/bincode/blob/trunk/docs/spec.md#varintencoding
(defn string [data] 
  "string to bytes" 
  (let [length-section (map byte (cons (count data) (repeat 7 NULL)))] 
    (concat string-discrim length-section (string->bytes data))))

(defn bool [data] 
  "boolean to bytes"
  (concat bool-discrim (seq [(bool->bytes data)])))

(defn number [data] 
  "number to bytes"
  (concat number-discrim (number->bytes data)))
