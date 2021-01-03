/// Based on Multi-producer, single-consumer FIFO queue communication primitives
/// (mpsc module). This is an asynchonous flavored implementation of a Rust channel.
///
/// Flavors of Channels... 'all channels are like Unix pipes'
/// - Synchronous channels: Channel were send() CAN block. Limited capacity.
///     -Mutex + Condvar + VexDeque
/// - Asynchronous Channels: Channel were send() CANNOT block. Unbounded capacity.
///     -Mutex + Condvar + LinkedList OR VecDeque(doubly linked list is best)
///     -Non-Mutex use Atomic-queue or Atomic-LinkedList
/// - Rendezvous channels: a sychromous Channel with capacity == 0. Used for thread
/// synchronization. You don't actually send things with it.
/// - Oneshot channels: Any capacity. But in practice only one call to send(). Lets say you
/// want a receiver to tell all the threads to shut down...A user hits ctl-c....
///
/// *******************************************
///
///
/// This is an implentation of a 'channel' data structure in the Rust language.
/// It is a code along based on the 'Crust of Rust' series by Jon Gjengset.

///
///  Many threads can send simultaneously messages to a single consumer.
/// Channels come in two flavors....Asyn and Sync. This one is Async.
/// There are many ways to go about making this, but we are doing a general
/// version that will show some of the 'useful concurrentcy primitives in Rust'

/// We will be useing a Mutex 'lock' from the 'mpsc' module. It has a 'lock'
/// method that returens a 'guard'. While you have the guard you are
/// guarenteed to be the only one with access to the <T> that owns the guard.
///
/// 'Arc. Mutex, Condvar' primitives will be used to create 'safe concurrent
/// channel'.
///
/// Arc...is thread-safe reference-counting pointer. 'Arc' stands for
///  'Atomically Reference Counted. Allows shared ownership of data on
/// the heap...has reference counter like a GC to manage memory clean-up,
/// Arc is a 'read-only' operation. to mutate the shared data you must use
/// a Mutex inside the Arc type. Each new reference to the Arc type increments
/// the counter by one....this is like reference counting in GC..but more
/// expensive to implement???? Maintaining atomicity is harder than in regular
/// 'ref-counting'..look into why I don't know.
///
///  Condvar....is a conditional variable that lets a diff thread know that it
///  has changed something it may care about. ex) It can notify a sleeping
///  thread that there is some data for it to read.
///
/// Mutex is like a 'semaphore'..(but in reality much diff).probably more
///  accuratly a mutex can be compared to a 'binary semaphore/. Mutex stands
///  for 'Mutual exclusion Object' . It essentially is a
/// variable
///  which aids in the sync-chronization of control access to the same resrorce
///  in multi-threaded envroments. Mutext is controlled by the Process that
///  owns it. Semaphore was invented by Edsger Dijkstra in
///  1962. The semaphore increments and decrements the semaphoric variable from
///  0-N intergers. It will block the threads that are trying to accesss a
///  reource that is being
/// accessed by another thread. Semaphore may be a signaling system while a
/// Mutex is a locking mechanism.

/// We want to make sure the Sender struct is clonable. Might have the instinct
/// to place '#[derive(Clone)]' before the declaration of Sender. But this is
///  just syntactic sugar that would de-sugar to something like ...
///         'impl <T: Clone> Clone for Sender<T>{
///                         fn clone(&self) -> Self
///                               ...compiler will add stuff here.
/// }
/// As you can see it will bound the Clone type to T...in our case Arc will
/// run clone on its own ....I don't understand what this really means....??????
/// Jon says., "often this is what we want because struct Sender may have
/// another type inside it which would need to be cloned"...in our case
/// Arc bounds a Clone type to its <T> automatically so we can leave it out.
/// But we will implement our own version of clone on the Sender struct
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};





/// 'self: &mut Self' === &mut self...Self is the type of the current object.
/// Used as the first argument in methods in 'trait' and 'impl'. The instance
/// of the struct the method is being called from. So, in the below example,  there is a mutable reference to the 'Sender' instance that this particular
/// method is being called against.
///
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}
// sending data down the channel transfers ownership of that data to the receiver.
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

/// We want to make a 'recv' function that blocks if there is nothing there...
/// the 'recv' function is designed intentionally to block main thread untill
/// a message is recieved with the value....?
/// we want it to wait. It waits for something to be in the channel. This is
/// were the 'Condvar' type comes into play.
///  The 'wait' function hands over a 'guard' to the sleeping thread. Wakes the
/// thread up and hands it the Mutex.
///
/// Optimization in the match statement...first arm of the statment...'std::mem::swap...' because
/// there is only one receiver when ever it has the 'mutex lock' we can take all of the
/// items that have been queued up. Nobody else will take them. This is a 1 receiver to
/// many senders model. We keep a local buffer and check it everytime the recv()
/// is called. If it's in the buffer we don't need to take the lock. just return the Some(t).
/// Only if the buffer is empty do we need to take the actual lock. If the buffer is empty we
/// swap out the queue buffer with the internal one which is empty...??? The lock is taken
/// fewer times with this setup and therefor it becomes easier to aquire.
///
/// We use Arc::clone syntax to make explicit that we want Arc to
/// be cloned and not what is inside of Arc.

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

// As Rust automatically calls the destructors of all contained fields, you don't have to implement Drop in most cases. But there are some cases where it is useful, for example for types which directly manage a resource. That resource may be memory, it may be a file descriptor, it may be a network socket. Once a value of that type is no longer going to be used, it should "clean up" its resource by freeing the memory or closing the file or socket. This is the job of a destructor, and therefore the job of Drop::drop.

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}
// 'push_back' is a method on the VecDeque type that appends to the back of the Vector...this is like a job queue. FIFO
// 'notify_one()' is a method on the Condvar Type.....Wakes up one blocked thread on this condvar.
impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                }
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}
/// Define an 'shared' type which will share the data between Sender/Reciever
/// types.
/// It will have a 'Condvar' which needs to live outside the Mutex
/// Lets say the current holder of the Mutex needs to wake up other threads
/// ....trying to avoid what is known as a 'dead lock'. "Double-locks" are the
/// root of a lot of deadlock bugs. As well and the misuse of channels.
/// First let go of the lock and then the Condvar needs proof you have the lock
/// then it will notify the other threads that the lock is free.

impl<T> Iterator for Receiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}



/// CHECK line 214. Not sure 'buffer: VexDeque::new() is even remotely correct. Tried to fix a
/// 'missing field buffer in intializer for Reciever' error

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::new(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }
    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }
    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}
