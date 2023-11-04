(define square (lambda (x) (* x x)))
(define fib (lambda (n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))
(define sum-n
  (lambda (n a)
    (if (= n 0) a
        (sum-n (- n 1) (+ n a)))))
