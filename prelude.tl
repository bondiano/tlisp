; -- Boolean functions --
(defun not (x) (if x #f #t))
(defun nil? (x) (== x #nil))

; -- Arithmetic functions --
(defun square (x) (* x x))
(defun cube (x) (* x x x))
(defun inc (x) (+ x 1))
(defun dec (x) (- x 1))
(defun abs (x) (if (< x 0) (- x) x))
(defun min (x y) (if (< x y) x y))
(defun max (x y) (if (> x y) x y))
(defun even? (x) (= (% x 2) 0))
(defun odd? (x) (= (% x 2) 1))
(defun zero? (x) (= x 0))
(defun sum-n (n a)
    (if (= n 0) a
        (sum-n (- n 1) (+ n a))))
(defun mul-n (n a)
    (if (= n 0) a
        (mul-n (- n 1) (* n a))))
(defun fact (n)
      (mul-n n 1))
      (define add-n
          (lambda (n)
            (lambda (a) (+ n a))))

; -- Utility functions --
(defun identity (x)
  (x))
(defun const (n)
  (lambda (a) (n)))
