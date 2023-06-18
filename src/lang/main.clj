(ns lang.main
  (:require [instaparse.core :as insta :refer [defparser]]))

(defn read-file [file-path]
  (slurp file-path))


(defn transform-str [data]
(->> data
     str
     ))

(defn transform [tree] 
  (insta/transform {
    :string transform-str 
    :number (comp clojure.edn/read-string str)} tree))

(defparser parser
    "program         ::= block
     block           ::= '{'? statement* '}'?

     statement       ::= assignment | conditional | loop | block | def
     def             ::= varexpr
     assignment      ::= '@' varexpr
     <varexpr>       ::= identifier <'='> expression
     conditional     ::= 'if' expression statement ['else' statement]
     loop            ::= 'while' expression statement
     
     <expression>    ::= term { ('+' | '-') term }
     
     <term>          ::= factor { ('*' | '/') factor }
     <factor>        ::= token | number | function | array | string | '(' expression ')'
     <token>         ::= bool | !bool identifier
     array           ::= '[' expression { <','> expression } <','?> ']'
     function        ::= 'fn' '(' identifier {<','> identifier} <','?> ')' '->' statement 
     identifier      ::= #'[a-zA-Z0-9]+'
     number          ::= #'^[0-9]+$'
     (* Crazy regex for strings. Clojure does not allow raw string regexes which lead to this*)
     string          ::= #'`([^\\\"\\\\]|\\\\.)*`'
     bool            ::= 'true' | 'false'
     " 
     :input-format :ebnf 
     :output-format :enlive 
     :auto-whitespace :standard 
     :total true)

(defn -main [& args]
  (println (parser(read-file "./x.txt"))))

