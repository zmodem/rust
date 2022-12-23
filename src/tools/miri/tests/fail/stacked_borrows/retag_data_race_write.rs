//! Make sure that a retag acts like a write for the data race model.
//@compile-flags: -Zmiri-preemption-rate=0
#[derive(Copy, Clone)]
struct SendPtr(*mut u8);

unsafe impl Send for SendPtr {}

fn thread_1(p: SendPtr) {
    let p = p.0;
    unsafe {
        let _r = &mut *p;
    }
}

fn thread_2(p: SendPtr) {
    let p = p.0;
    unsafe {
        *p = 5; //~ ERROR: Data race detected between Write on thread `<unnamed>` and Write on thread `<unnamed>`
    }
}

fn main() {
    let mut x = 0;
    let p = std::ptr::addr_of_mut!(x);
    let p = SendPtr(p);

    let t1 = std::thread::spawn(move || thread_1(p));
    let t2 = std::thread::spawn(move || thread_2(p));
    let _ = t1.join();
    let _ = t2.join();
}