(ns test
  (:import Test))

;; Serialisation

(println (Test/ser 1))

;; Stress test to see if we leak references
(let [n 10000
      v (Test/ser n)]
  (assert (= (count v) n)))

;; Deserialisation

;; (Test/de [1 2 3 700])
;; (Test/de {:one "one" :two 45})


;; roundtrip

(let [v (Test/ser 1000)]
  (assert (= v (Test/roundtrip v))))

