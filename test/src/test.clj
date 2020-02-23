(ns test
  (:import Test))

;; Serialisation

(println (Test/ser 1))

;; Stress test to see if we leak references
(let [n 10000
      v (Test/ser n)]
  (assert (= (count v) n)))

;; Deserialisation

(Test/de (Test/ser 1))


;; roundtrip

;; we can't directly compare byte arrays
(defn fix-bytes [v]
  (map #(update % :bytes seq) v))

(let [v (Test/ser 1000)]
  (assert (= (fix-bytes v)
             (fix-bytes (Test/roundtrip v)))))
