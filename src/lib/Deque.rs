/**
 * A deque, for fun.  Untested as of yet.  Likely buggy.
 */

type t[T] = obj {
            fn size() -> uint;

            fn add_front(&T t);
            fn add_back(&T t);

            fn pop_front() -> T;
            fn pop_back() -> T;

            fn peek_front() -> T;
            fn peek_back() -> T;

            fn get(int i) -> T;
};

fn create[T]() -> t[T] {

    type cell[T] = Option.t[T];

    let uint initial_capacity = 32u; // 2^5

    /**
     * Grow is only called on full elts, so nelts is also len(elts), unlike
     * elsewhere.
     */
    fn grow[T](uint nelts, uint lo, vec[cell[T]] elts) -> vec[cell[T]] {
        assert (nelts == Vec.len[cell[T]](elts));

        fn fill[T](uint i, uint nelts, uint lo,
                   vec[cell[T]] old) -> cell[T] {
            if (i < nelts) {
                ret old.((lo + i) % nelts);
            } else {
                ret Option.none[T];
            }
        }

        let uint nalloc = UInt.next_power_of_two(nelts + 1u);
        let Vec.init_op[cell[T]] copy_op = bind fill[T](_, nelts, lo, elts);
        ret Vec.init_fn[cell[T]](copy_op, nalloc);
    }

    fn get[T](vec[cell[T]] elts, uint i) -> T {
        alt (elts.(i)) {
            case (Option.some[T](?t)) { ret t; }
            case (_) { fail; }
        }
        fail;   // FIXME: remove me when exhaustiveness checking works
    }

    obj deque[T](mutable uint nelts,
                 mutable uint lo,
                 mutable uint hi,
                 mutable vec[cell[T]] elts)
        {
            fn size() -> uint { ret nelts; }

            fn add_front(&T t) {
                let uint oldlo = lo;

                if (lo == 0u) {
                    lo = Vec.len[cell[T]](elts) - 1u;
                } else {
                    lo -= 1u;
                }

                if (lo == hi) {
                    elts = grow[T](nelts, oldlo, elts);
                    lo = Vec.len[cell[T]](elts) - 1u;
                    hi = nelts;
                }

                elts.(lo) = Option.some[T](t);
                nelts += 1u;
            }

            fn add_back(&T t) {
                if (lo == hi && nelts != 0u) {
                    elts = grow[T](nelts, lo, elts);
                    lo = 0u;
                    hi = nelts;
                }

                elts.(hi) = Option.some[T](t);
                hi = (hi + 1u) % Vec.len[cell[T]](elts);
                nelts += 1u;
            }

            /**
             * We actually release (turn to none()) the T we're popping so
             * that we don't keep anyone's refcount up unexpectedly.
             */
            fn pop_front() -> T {
                let T t = get[T](elts, lo);
                elts.(lo) = Option.none[T];
                lo = (lo + 1u) % Vec.len[cell[T]](elts);
                nelts -= 1u;
                ret t;
            }

            fn pop_back() -> T {
                if (hi == 0u) {
                    hi = Vec.len[cell[T]](elts) - 1u;
                } else {
                    hi -= 1u;
                }

                let T t = get[T](elts, hi);
                elts.(hi) = Option.none[T];
                nelts -= 1u;
                ret t;
            }

            fn peek_front() -> T {
                ret get[T](elts, lo);
            }

            fn peek_back() -> T {
                ret get[T](elts, hi - 1u);
            }

            fn get(int i) -> T {
                let uint idx = (lo + (i as uint)) % Vec.len[cell[T]](elts);
                ret get[T](elts, idx);
            }

        }
    let vec[cell[T]] v = Vec.init_elt[cell[T]](Option.none[T],
                                                initial_capacity);

    ret deque[T](0u, 0u, 0u, v);
}

// Local Variables:
// mode: rust;
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// compile-command: "make -k -C .. 2>&1 | sed -e 's/\\/x\\//x:\\//g'";
// End: