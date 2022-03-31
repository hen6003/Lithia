;; Create function that calcuates lcm
(= lcm (func (a b)
	     ;; Set i to 1
	     (= i 1)

	     ;; While a * i / b != an integer, increase i
	     (while (!= (% (/ (* a i) b) 1) 0)
	       (= i (+ i 1)))

	     ;; Return a * i
	     (* a i)))

;; Run lcm function and print result
(print (lcm 432 20))
