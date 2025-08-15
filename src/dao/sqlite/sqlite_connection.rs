// use crate::dao::sqlite::create_sqlite_tables::create_sqlite_tables_if_not_exists;
// use sqlite;
// use std::env;
// use std::ops::{Deref, DerefMut};
// use std::sync::{
//     Condvar, Mutex, OnceLock,
//     atomic::{AtomicUsize, Ordering},
// };
//
// struct Pool {
//     conns: Mutex<Vec<sqlite::Connection>>,
//     available: Condvar,
//     max_size: usize,
//     created: AtomicUsize,
//     db_path: String,
// }
//
// impl Pool {
//     fn new(db_path: String, max_size: usize) -> Self {
//         Self {
//             conns: Mutex::new(Vec::new()),
//             available: Condvar::new(),
//             max_size,
//             created: AtomicUsize::new(0),
//             db_path,
//         }
//     }
//
//     fn checkout(&'static self) -> PooledConnection {
//         // Fast path: reuse an existing connection if available
//         if let Some(conn) = self.conns.lock().unwrap().pop() {
//             return PooledConnection {
//                 conn: Some(conn),
//                 pool: self,
//             };
//         }
//
//         // Try to create a new connection if we are under the max_size
//         if let Ok(prev) = self
//             .created
//             .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |cur| {
//                 if cur < self.max_size {
//                     Some(cur + 1)
//                 } else {
//                     None
//                 }
//             })
//         {
//             let conn = open_connection(&self.db_path);
//
//             // Run schema/table creation once, on the first created connection
//             if prev == 0 {
//                 create_sqlite_tables_if_not_exists(&conn);
//             }
//
//             return PooledConnection {
//                 conn: Some(conn),
//                 pool: self,
//             };
//         }
//
//         // Otherwise, wait until someone returns one
//         let mut guard = self.conns.lock().unwrap();
//         loop {
//             if let Some(conn) = guard.pop() {
//                 return PooledConnection {
//                     conn: Some(conn),
//                     pool: self,
//                 };
//             }
//             guard = self.available.wait(guard).unwrap();
//         }
//     }
// }
//
// pub struct PooledConnection {
//     conn: Option<sqlite::Connection>,
//     pool: &'static Pool,
// }
//
// impl Deref for PooledConnection {
//     type Target = sqlite::Connection;
//     fn deref(&self) -> &Self::Target {
//         self.conn.as_ref().expect("pooled connection missing")
//     }
// }
//
// impl DerefMut for PooledConnection {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.conn.as_mut().expect("pooled connection missing")
//     }
// }
//
// impl Drop for PooledConnection {
//     fn drop(&mut self) {
//         if let Some(conn) = self.conn.take() {
//             let mut list = self.pool.conns.lock().expect("pool mutex poisoned");
//             list.push(conn);
//             self.pool.available.notify_one();
//         }
//     }
// }
//
// static POOL: OnceLock<Pool> = OnceLock::new();
//
// fn get_pool() -> &'static Pool {
//     POOL.get_or_init(|| {
//         let db_path = env::var("INVOICE_DB_PATH").expect("INVOICE_DB_PATH not set");
//         let max_size: usize = env::var("INVOICE_DB_POOL_SIZE")
//             .ok()
//             .and_then(|v| v.parse().ok())
//             .unwrap_or(4);
//         Pool::new(db_path, max_size)
//     })
// }
//
// fn open_connection(db_path: &str) -> sqlite::Connection {
//     let conn = sqlite::Connection::open(db_path).expect("failed to open DB");
//
//     // Optional pragmas for performance/concurrency
//     let _ = conn.execute("PRAGMA journal_mode = WAL;");
//     let _ = conn.execute("PRAGMA synchronous = NORMAL;");
//     let _ = conn.execute("PRAGMA busy_timeout = 5000;");
//     let _ = conn.execute("PRAGMA foreign_keys = ON;");
//
//     conn
// }
//
// // Callers get a pooled connection that derefs to sqlite::Connection.
// // The connection remains open and is returned to the pool when the guard is dropped.
// pub fn get_connection() -> PooledConnection {
//     get_pool().checkout()
// }
