; -- Boolean functions --
(defun not (x) (cond x #t #t #f))
(defun nil? (x) (== x #nil))

; -- Arithmetic functions --
(defun square (x) (* x x))
(defun cube (x) (* x x x))
(defun inc (x) (+ x 1))
(defun dec (x) (- x 1))
(defun abs (x) (cond (< x 0) (- x) #t x))
(defun min (x y) (cond (< x y) x #t y))
(defun max (x y) (cond (> x y) x #t y))
(defun even? (x) (= (% x 2) 0))
(defun odd? (x) (= (% x 2) 1))
(defun zero? (x) (= x 0))
(defun sum-n (n a)
    (cond (= n 0) a
          #t (sum-n (- n 1) (+ n a))))
(defun mul-n (n a)
    (cond (= n 0) a
          #t (mul-n (- n 1) (* n a))))
(defun fact (n)
      (mul-n n 1))
(define add-n
    (lambda (n)
      (lambda (a) (+ n a))))

; -- Utility functions --
(defun identity (x)
  (x))
(defun const (n)
  (lambda (a) n))
