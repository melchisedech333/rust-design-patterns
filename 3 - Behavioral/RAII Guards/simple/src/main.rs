
/// RefCell é um tipo de RAII Guard.

use std::cell::RefCell;

fn main() {

    let ref_cell = RefCell::new(7);

    {
        // The RAII guard is created, which borrows the RefCell
        // and acquires a write lock.
        let mut guard = ref_cell.borrow_mut(); 
        *guard *= 6;
        
    } // The RAII guard is dropped, which releases the write lock.

    dbg!(ref_cell);

} // The RefCell is dropped.


