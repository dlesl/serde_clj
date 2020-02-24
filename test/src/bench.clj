(ns bench
  (:require [criterium.core :refer [quick-bench]]
            [cheshire.core :as json])
  (:import Test))

;; Serialisation

(def canada-rs (Test/canadaSer))

;; (def canada-json-raw (slurp "data/canada.json"))

(defn canada-json-decode [s] (json/parse-string s true))

(def canada-json (canada-json-decode (Test/canadaSerJson)))

(assert (= (count (:features canada-rs)) (count (:features canada-json))))

(println "Bench serde_clj ser")
(quick-bench (Test/canadaSer))

(println "Bench cheshire ser")
(quick-bench (canada-json-decode (Test/canadaSerJson)))

(println "Bench serde_clj de")
(quick-bench (Test/canadaDe canada-rs))

(println "Bench cheshire de")
(quick-bench (json/generate-string canada-json))



(def twitter-rs (Test/twitterSer))

(def twitter-json-raw (slurp "data/twitter.json"))

(defn twitter-json [] (json/parse-string twitter-json-raw true))

(assert (= (count (:statuses twitter-rs)) (count (:statuses (twitter-json)))))

(println "Bench serde_clj de twitter")
(quick-bench (Test/twitterSer))

(println "Bench cheshire de twitter")
(quick-bench (twitter-json))
