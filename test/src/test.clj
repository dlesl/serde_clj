(ns test
  (:import Test))

(println (Test/test 1))

;; Stress test to see if we leak references
(let [n 10000
      v (Test/test n)]
  (assert (= (count v) n)))
