; -- Boolean functions --
(define not (lambda (x) (if x #f #t)))
(define xor (lambda (x y) (if (and x y) #f (or x y))))
(define and (lambda (x y) (if x y #f)))
(define or (lambda (x y) (if x #t y)))
(define nand (lambda (x y) (if (and x y) #f #t)))
(define nor (lambda (x y) (if (or x y) #f #t)))
(define xnor (lambda (x y) (if (xor x y) #f #t)))

; -- Arithmetic functions --
(define square (lambda (x) (* x x)))
(define cube (lambda (x) (* x x x)))
(define add (lambda (x y) (+ x y)))
(define sub (lambda (x y) (- x y)))
(define mul (lambda (x y) (* x y)))
(define div (lambda (x y) (/ x y)))
(define mod (lambda (x y) (% x y)))
(define inc (lambda (x) (+ x 1)))
(define dec (lambda (x) (- x 1)))
(define abs (lambda (x) (if (< x 0) (- x) x)))
(define min (lambda (x y) (if (< x y) x y)))
(define max (lambda (x y) (if (> x y) x y)))
(define even? (lambda (x) (= (% x 2) 0)))
(define odd? (lambda (x) (= (% x 2) 1)))
(define zero? (lambda (x) (= x 0)))
(define sum-n
  (lambda (n a)
    (if (= n 0) a
        (sum-n (- n 1) (+ n a)))))
(define mul-n
  (lambda (n a)
    (if (= n 0) a
        (mul-n (- n 1) (* n a)))))
(define fact
  (lambda (n)
      (mul-n n 1)))
