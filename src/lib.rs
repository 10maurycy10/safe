#![forbid(unsafe_code)]

pub use borrowfix::remember;
pub use borrowfix::remember_mut;

enum Helper<A,B> {
    A(A),
    B(B)
}


/// convert a refrence to one type into another.
/// The passed "val" is discarded.
pub fn transmute_ref_with_value<A, B>(input: &A, val: &'static B) -> &'static B {
    // create an enum
    let mut helper = Helper::B(val);
    // create a 'static pointer to contence.
    let refer = match &helper {
        Helper::A(_) => panic!(),
        Helper::B(b) => remember(b)
    };
    // change the contained data in helper
    // As helper is alocated on stack, this leaves refer pointing to the new data.
    helper = Helper::A(input);
    return *refer;
}

/// like transmute_ref_with_value but using ::default() as the dummy value
pub fn transmute_ref<A, B: Default>(input: &A) -> &'static B {
    let def = B::default();
    transmute_ref_with_value(input, remember(&def))
}

/// Convert a &'static into a &'mut static
/// taking a dummy ref
pub fn make_mut_with_value<'a, T>(input: &'static T, dummy: &'static mut T) -> &'static mut T {
    let mut helper: Helper<&'static T,&'static mut T> = Helper::B(dummy);
    let refer = match &mut helper {
        Helper::A(_) => panic!(),
        Helper::B(b) => remember_mut(b)
    };
    helper = Helper::A(input);
    return *refer;
}


/// transmute one type into another
/// The passed "val" is discarded
pub fn transmute_with_value<A, B: 'static + Clone>(input: A, val: &B) -> B {
    transmute_ref_with_value(&input, remember(val)).clone()
}

/// get the address from a refrence
pub fn addr_of<'a, T>(refer: &'a T) -> usize {
     *transmute_ref::<&T, usize>(&refer)
}

/// get the memory at an address
pub fn byte_at(refer: usize) -> &'static u8 {
    let dummy = remember(&remember(&0));
     *transmute_ref_with_value::<usize, &'static u8>(&refer,dummy)
}

pub fn byte_at_mut(refer: usize) -> &'static mut u8 {
    let dummy = remember(&remember(&0));
    let byte = *transmute_ref_with_value::<usize, &'static u8>(&refer,dummy);
    make_mut_with_value(byte,remember_mut(&mut 0))
}

#[cfg(test)]
mod tests {
    #[test]
    fn transmute() {
        use crate::transmute_ref;
        let result = transmute_ref::<u16,i16>(&24_u16);
        assert_eq!(result, &24_i16);
    }
    #[test]
    fn address_of() {
        use crate::addr_of;
        let slice = [1u8, 2u8, 3u8];
        // the address of slice[0] should be 2 less than the address of slice[2]
        assert_eq!(addr_of(&slice[0]) + 2, addr_of(&slice[2]));
    }
    #[test]
    fn byte_at_test() {
        use crate::byte_at;
        use crate::addr_of;
        let slice = [1u8, 2u8, 3u8];
        let addr = addr_of(&slice[0]);
        assert_eq!(byte_at(addr + 2), &3);
    }
    #[test]
    fn byte_at_mut() {
        use crate::byte_at_mut;
        use crate::addr_of;
        let slice = [1u8, 2u8, 3u8];
        let addr = addr_of(&slice[0]);
        *byte_at_mut(addr + 2) = 4;
        assert_eq!(slice, [1,2,4])
    }
}
