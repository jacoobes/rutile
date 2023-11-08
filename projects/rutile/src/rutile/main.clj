(ns rutile.main
  (:require [instaparse.core :as insta :refer [defparser]]
            [clojure.java.io :as io]
            [rutile.struct :as struct]
            [rutile.writer :as write :refer [version]]
            [clojure.zip :as zip]
            [clojure.walk :refer [postwalk]]) (:gen-class))

(set! *warn-on-reflection* true)

(defn read-file [file-path]
  (slurp file-path))


(defparser parser
    "program         ::= statement* 
     <statement>     ::= defn | assignment | conditional | loop | def | block
     block           ::= &'{' statement* <'}'>
     defn            ::= <'defn'> identifier <'('> identifier? {<','> identifier} <','?> <')'> statement
     def             ::= varexpr
     assignment      ::= <'@'> varexpr
     <varexpr>       ::= identifier <'='> expr
     conditional     ::= 'if' expr statement ['else' statement]
     loop            ::= 'while' expr statement
     
     <expr>          ::= add-sub
     <add-sub>       ::= primary | add | add-sub
     add             ::= add-sub <'+'> mul-div
     sub             ::= add-sub <'-'> mul-div
     <mul-div>       ::= primary | mul | div 
     mul             ::= mul-div <'*'> primary
     div             ::= mul-div <'/'> primary
     <primary>       ::= token | number | array | string | <'('> expr <')'>

     <token>         ::= bool | !bool identifier
     array           ::= <'['> expr { <','> expr } <','?> <']'>
     identifier      ::= #'[a-zA-Z0-9]+'
     number          ::= #'[0-9]+'
     (* Crazy regex for strings. Clojure does not allow raw string regexes which lead to this *)
     string          ::= #'`([^\\\"\\\\]|\\\\.)*`'
     bool            ::= 'true' | 'false'
     " 
     :input-format :ebnf 
     :output-format :hiccup
     :auto-whitespace :standard 
     :total true)

(defn zip-map [f loc]
  " Map f over every node of the zipper.
    The function received has the form (f node-value loc),
    the node value and its location"
  (loop [z loc]
    (if (zip/end? z)
      (zip/root z) 
      (recur (zip/next (zip/edit z f z))))))

(defn -main [& args]
  (let [tree (->> (read-file "./x.txt") 
                   parser)
        path "./x.lang"]
  (with-open [writer (io/output-stream path)]
    (let [const-table (struct/const-table tree)
          loc (zip/vector-zip tree)] 
      (write/version writer)
      (write/const-table writer const-table)
      (write/bytecode writer tree const-table)))))

