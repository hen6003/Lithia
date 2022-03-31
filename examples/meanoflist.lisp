;; Mean calcuator function
(= mean (func (list)
	      (= total 0)
	      (= len 0)
	      
	      (while (!= list ())
		(= total (+ total (car list)))
		
		(= len (+ len 1))
		(= list (cdr list)))
	      
	      (/ total len)))

;; List to find mean of
(= list (quote (1 4 6 2)))

;; Print mean of list
(print (mean list))
