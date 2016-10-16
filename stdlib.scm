; rScheme standard library.

(define pi 3.14159)

(define <= (lambda (a b)
  (or (< a b) (= a b))))

(define >= (lambda (a b)
  (or (> a b) (= a b))))

(define abs (lambda (x)
  (if (< x 0) (- x) x)))
