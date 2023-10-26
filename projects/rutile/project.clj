(defproject rutile "HEAD"
  :description "frontend language for kongoc written in clojure"
  :dependencies [[org.clojure/clojure "1.11.1"]
                 [instaparse "1.4.12"]]
  :plugins [[ cider/cider-nrepl "0.22.1" :exclusions [nrepl] ]]
  :repl { 
    :dependencies [[org.clojure/tools.nrepl "0.2.13"]] 
  }
  :main rutile.main)
