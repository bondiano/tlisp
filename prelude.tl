; -- Boolean functions --
(define (not x) (if x #f #t))
(define (nil? x) (== x #nil))

; -- Arithmetic functions --
(define (square x) (* x x))
(define (cube x) (* x x x))
(define (inc x) (+ x 1))
(define (dec x) (- x 1))
(define (abs x) (if (< x 0) (- x) x))
(define (min x y) (if (< x y) x y))
(define (max x y) (if (> x y) x y))
(define (even? x) (= (% x 2) 0))
(define (odd? x) (= (% x 2) 1))
(define (zero? x) (= x 0))
(define (sum-n n a)
    (if (= n 0) a
        (sum-n (- n 1) (+ n a))))
(define (mul-n n a)
    (if (= n 0) a
        (mul-n (- n 1) (* n a))))
(define (fact n)
      (mul-n n 1))

; -- Utility functions --
(define (identity x)
    (x))
