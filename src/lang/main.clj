(ns lang.main
  (:require [instaparse.core :as insta :refer [defparser]]
            [clojure.java.io :as io]
            [lang.writer :refer [version]]
            [clojure.walk :refer [postwalk]]))

(defn read-file [file-path]
  (slurp file-path))


(defn transform [tree] 
  (insta/transform {
    :number clojure.edn/read-string
    } tree))

(defparser parser
    "block           ::= <'{'?> statement* <'}'?>

     <statement>     ::= assignment | conditional | loop | block | def
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
     <primary>       ::= token | number | function | array | string | <'('> expr <')'>

     <token>         ::= bool | !bool identifier
     array           ::= <'['> expr { <','> expr } <','?> <']'>
     function        ::= <'fn'> <'('> identifier {<','> identifier} <','?> <')'> <'->'> statement 
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
  (let [scope-level 0]
    (insta/transform { 
      } tree)))

(defn -main [& args]
  (let [tree (->> (read-file "./x.txt") 
                  parser
                  transform)
        path "./x.lang"]
  (with-open [writer (io/output-stream path)]
    (do (.write writer version))
    tree))
  )

