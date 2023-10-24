(ns lang.src.utils)
;; if-let multiple bindings version
(defmacro if-let*
  ([bindings-vec then] `(if-let* ~bindings-vec ~then nil))
  ([bindings-vec then else]
   (if (seq bindings-vec)
     `(let ~bindings-vec
        (if (and ~@(take-nth 2 bindings-vec))
          ~then
          ~else)))))
