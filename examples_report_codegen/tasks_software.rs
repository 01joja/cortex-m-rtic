


// Software
// from report_rtic_sw_mess_expanded
/*
Dispatcher
*/

// enum of the tasks with a priority of one
pub enum P1_T {
    bar,
    foo,
}

// task request queue for foo
static __rtic_internal_foo_FQ: rtic::RacyCell<rtic::export::SCFQ<4>> = rtic::RacyCell::new(
    rtic::export::Queue::new(),
);

// holds task messages between spawning and dispatching
static __rtic_internal_foo_INPUTS: rtic::RacyCell<
    [core::mem::MaybeUninit<(i8, u8)>; 3],
> = rtic::RacyCell::new([
    core::mem::MaybeUninit::uninit(),
    core::mem::MaybeUninit::uninit(),
    core::mem::MaybeUninit::uninit(),
]);

// the request queue for software tasks with priority 1
// length is equal to the total sum of the capacity of all software tasks with a priority one
static __rtic_internal_P1_RQ: rtic::RacyCell<rtic::export::SCRQ<P1_T, 5>> = rtic::RacyCell::new(
    rtic::export::Queue::new(),
);

unsafe fn SSI0() {
    /// The priority of this interrupt handler
    const PRIORITY: u8 = 1u8; // priority of the task
    rtic::export::run(
        PRIORITY,
        || {
            // tries to dispatch new tasks as long as there are a task in the request queue
            while let Some((task, index)) = (&mut *__rtic_internal_P1_RQ.get_mut())
                .split()
                .1
                .dequeue()
            {
                // finds the correct task to dispatch by matching them.
                match task {
                    P1_T::bar => {
                        let _0 = (&*__rtic_internal_bar_INPUTS.get())
                            .get_unchecked(usize::from(index))
                            .as_ptr()
                            .read();
                        (&mut *__rtic_internal_bar_FQ.get_mut())
                            .split()
                            .0
                            .enqueue_unchecked(index);
                        let priority = &rtic::export::Priority::new(PRIORITY);
                        bar(bar::Context::new(priority), _0)
                    }
                    P1_T::foo => {
                        let (_0, _1) = (&*__rtic_internal_foo_INPUTS.get())
                            .get_unchecked(usize::from(index))
                            .as_ptr()
                            .read();
                        (&mut *__rtic_internal_foo_FQ.get_mut())
                            .split()
                            .0
                            .enqueue_unchecked(index);
                        let priority = &rtic::export::Priority::new(PRIORITY);
                        foo(foo::Context::new(priority), _0, _1)
                    }
                }
            }
        },
    );
}

// The spawn function for the software task foo
pub fn __rtic_internal_foo_spawn(_0: i8, _1: u8) -> Result<(), (i8, u8)> {
    // makes a tuple of the messages
    let input = (_0, _1);
    unsafe {
        // tries to pop and index from the task request queue
        if let Some(index) = rtic::export::interrupt::free(|_| {
            (&mut *__rtic_internal_foo_FQ.get_mut()).dequeue()
        }) {
            (&mut *__rtic_internal_foo_INPUTS.get_mut())
                .get_unchecked_mut(usize::from(index))
                .as_mut_ptr()
                .write(input);
            rtic::export::interrupt::free(|_| {
                (&mut *__rtic_internal_P1_RQ.get_mut())
                    .enqueue_unchecked((P1_T::foo, index));
            });
            rtic::pend(lm3s6965::interrupt::SSI0);
            Ok(())
        } else {
            Err(input)
        }
    }
}


// post init:
    // prepares the foo task request queues with indexes
    // foo has a capacity of 3 and therefore uses 3u8. 
    (0..3u8)
        .for_each(|i| {
            (&mut *__rtic_internal_foo_FQ.get_mut()).enqueue_unchecked(i)
        });
    // also uses the code from hardware task (see listing no X) to enable the dispatchers
