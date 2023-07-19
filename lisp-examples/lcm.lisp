;; Create function that calcuates lcm
(defunc lcm (a b)
        (defunc get-lcm (a b i)
                (if (!= (% (/ (* a i) b) 1) 0)
                  (get-lcm a b (+ i 1))
                  (* a i)))


        (get-lcm a b 1))

;; Run lcm function and print result
(print (lcm 4 10))
