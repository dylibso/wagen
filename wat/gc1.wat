(module 
	(type $t (struct (field $a (mut i32)) (field $b (mut i32))))
	(func $init (param i32) (param i32) (result (ref $t))
		local.get 0
		local.get 1
		struct.new $t
		return
	)
	(func $sum (param (ref $t)) (result i32)
		;; Get a field
	    local.get 0
		struct.get $t $a

		;; Get b field
	    local.get 0
	    struct.get $t $b

	    i32.add
	    return
	)

	(func $add (param $a (ref $t)) (param $b (ref $t)) 
		;; Add `a` fields
		local.get 0
		local.get 0
		struct.get $t $a
		local.get 1
		struct.get $t $a
		i32.add
		struct.set $t $a

		;; Add `b` fields
		local.get 0
		local.get 0
		struct.get $t $b
		local.get 1
		struct.get $t $b
		i32.add
		struct.set $t $b
		return
	)

	(export "init" (func $init))
	(export "sum" (func $sum))
	(export "add" (func $add))
)
