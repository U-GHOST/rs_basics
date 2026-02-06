#![allow(
	unused_assignments,
	unused_imports,
	dead_code,
	non_camel_case_types,
)]

// Threading and atomic types

// Arc<T>: An Atomic Reference Counter is the thread-safe version of Rc, and uses atomic operations. It is slightly slower than Rc (obv.), and uses atomic ref counting.
// Arc is also immutable by default, but can be wrapped inside a Mutex<T> or a RwLock<T> for interior mutability.

use std::{sync::{
	Arc, Mutex, RwLock
}, thread};

fn check_arc() {
	let data = Arc::new( 5 ); // Arc<T> allows using the underlying type natively (i.e., we can use the i32 inside this Arc<T> natively as an i32)
	// Atomic borrowing. Both data or data_clone can be used inside another thread safely.
	let data_clone = Arc::clone( &data );

	std::thread::spawn( move || {
		// But using it inside a different thread moves the object ownership into the thread and dropped when the thread finishes
		println! ( "Data: {}.", data );
	});

	// println! ( "{data}" ); // This will throw a 'borrow of moved value' error, that's why we Arc::clone() the object and use the clone inside the thread instead
}

fn check_arc_2() {
	let data = Arc::new( 5 ); // This is for the current thread
	let data_clone = Arc::clone( &data ); // This is for the spawned thread

	std::thread::spawn( move || {
		println! ( "Data: {}.", data_clone );
	});

	println! ( "Data: {}.", data );
}

/*
Mutex<T> and RwLock<T>:
- Both types protect shared data across threads.

- Mutex (Mutual Exclusion) is a key, only one thread can hold the key, everyone else waits in line (for both reading and writing).
	- [ Thread A ] ---- LOCK ----> DATA <---- WAIT ---- [ Thread B ]

- RwLock (Read-Write Lock) is a type with two keys, read key (many allowed), write key (only one, exclusive).
*/
fn check_mutex() {
	let x: Mutex<i32> = Mutex::new( 5 );
	let mut guard: std::sync::MutexGuard<'_, i32> = x.lock().unwrap(); // This creates a MutexGuard around the Mutex variable, allowing one read or one write at a time, at all times!

	*guard += 1;

	println! ( "{}", x.lock().unwrap() );
}

fn check_mutex_th() {
	let counter: Arc<Mutex<i32>> = Arc::new( Mutex::new( 0 ) );
	let counter_arc = Arc::clone( &counter );

	std::thread::spawn( move || {
		*counter_arc.lock().unwrap() += 1;

		println! ( "{}", counter_arc.lock().unwrap() );
	});

	println! ( "{}", counter.lock().unwrap() );
}

fn check_rw() {
	let data = RwLock::new( 5 );

	// Multiple readers
	let r1 = data.read().unwrap();
	let r2 = data.read().unwrap();

	println! ( "{} {}", *r1, *r2 );

	// One writer
	let mut w = data.write().unwrap();
	*w += 1;
}

fn check_rw_th() {
	let data = Arc::new( RwLock::new( 5 ) );
	let data_clone = Arc::clone( &data );

	std::thread::spawn( move || {
		let mut data_clone_w = data_clone.write().unwrap();
		*data_clone_w += 1;

		println! ( "{}", data_clone_w ); // I think we can use the write lock for prints too ig
	});
}

// Using thread-safety traits: Send + Sync allow us to track "Can this value move to another thread?" and "Can this be shared accross threads?". T is Sync if &T is Send.

// Examples (mostly for Arc and RwLock, the ones I'll use frequently)
struct Server {
	addr: String, // std::net::SocketAddr ideally
	// Servers manage different clients in a different thread, hence the Arc<RwLock<T>>
	clients: Arc<RwLock<Vec<String>>>,
}

// Let use impl for internal mutation
impl Server {
	fn new() -> Self {
		Self { addr: String::new(), clients: Arc::new( RwLock::new( Vec::new() ) ) }
	}

	// We don't need a mut ref to self, we just get a read/write key for the RwLock (since the Arc let's us pass through)
	fn push_client( &self, data: String ) -> () {
		// Don't forget to clone the main arc object
		let cli_arc = Arc::clone( &self.clients );
		// Make the String thread-accessible (this won't work for String literals)
		let data_arc = Arc::new( data ); // It must persist throughout the method
		let data_arc_clone = Arc::clone( &data_arc ); // And throughout the thread (not required, just being extra safe, LoL)

		std::thread::spawn( move || { // This will be a Tokio::thread::spawn( async move {} );
			let mut rw_w_guard = cli_arc.write().unwrap(); // Don't forget the difference between r lock and w lock (and mut)!

			rw_w_guard.push( data_arc_clone.to_string() );
		});

		// Debug
		for cli in self.clients.read().unwrap().iter() {
			println! ( "{cli}" );
		}
	}

	fn pop_client( &self ) -> () {
		let cli_arc = Arc::clone( &self.clients );

		std::thread::spawn( move || {
			let mut rw_w_guard = cli_arc.write().unwrap();

			rw_w_guard.pop();
		});
	}
}

/*
Atomic Types:
- Atomic types in Rust are types that provide safe concurrent access without the need for a Mutex<T>.
- They operate using atomic operations to ensure that increments, decrements, and reads/writes are performed as a single atomic action, which means they cannot be interrupted.
*/

// Not fully sure on these!
fn check_atomic() {
	use std::sync::atomic::{AtomicUsize, Ordering};

	let counter = AtomicUsize::new( 0 );

	counter.fetch_add( 1 , Ordering::Relaxed );
}

/*
When to use which:
- Mostly reads: RwLock
- Frequest writes: Mutex
- No shared state: Channels
*/

/*
Channels (Message Passing):
- “Do not communicate by sharing memory; share memory by communicating.""
*/
