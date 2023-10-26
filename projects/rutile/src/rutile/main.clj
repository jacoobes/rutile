(ns rutile.main
  (:require [instaparse.core :as insta :refer [defparser]]
            [clojure.java.io :as io]
            [rutile.struct :as struct]
            [rutile.writer :as write :refer [version]]
            [clojure.walk :refer [postwalk]]) (:gen-class))

(defn read-file [file-path]
  (slurp file-path))


(defparser parser
    "block           ::= <'{'?> statement* <'}'?>

     <statement>     ::= assignment | conditional | loop | block | def
     defn            ::= defn identifier <'('> identifier {<','> identifier} <','?> <')'> statement
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
     (* Crazy regex for strings. Clojure does not allow raw string regexes which lead to this*)
     string          ::= #'`([^\\\"\\\\]|\\\\.)*`'
     bool            ::= 'true' | 'false'
     " 
     :input-format :ebnf 
     :output-format :enlive 
     :auto-whitespace :standard 
     :total true)


(defn walk [tree]   
  (let [scope-level (atom 0)]
    (postwalk (fn [node] 
                
                
      ) tree)))

(defn -main [& args]
  (let [tree (->> (read-file "./x.txt") 
                   parser)
        path "./x.lang"]
  (with-open [writer (io/output-stream path)]
    (let [const-table (struct/const-table tree)] 
      (write/version writer)
      (write/const-table writer const-table)
      const-table))))

