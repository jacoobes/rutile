(defproject rutile "HEAD"
  :description "frontend language for kongoc written in clojure"
  :dependencies [[org.clojure/clojure "1.11.1"]
                 [instaparse "1.4.12"]
                 [org.clojure/tools.cli "1.0.219"]]
  :plugins [[cider/cider-nrepl "0.28.1"]]
  :global-vars {*warn-on-reflection* true}
  :source-paths ["src" "src/main/clojure"]
  :main lang.src.main)
