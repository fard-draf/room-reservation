## In-Memory Cache Summary

This version introduces an in-memory cache using `Arc<RwLock<...>>` to store known users, rooms, and booked (room, date) combinations. The cache avoids hitting the database for validation checks on every booking attempt.

### What was added:
- Shared caches for `UserName`, `RoomName`, and `(RoomName, BookDate)` using `HashMap` and `HashSet`
- Synchronized access with `tokio::RwLock` to allow concurrent reads and safe writes
- Refactored validation logic (`prepare_booking_input`) to run without any `.await`
- Updates to the cache only happen after successful DB inserts (for consistency)

### Does it improve performance?
Not really. Benchmarks show that the raw throughput (`requests/sec`) is **not significantly better** than the previous version without cache. In some cases, the version without cache even scored slightly higher.

###  Benchmarks Comparison

| Mode         | Avg Req/sec | Avg Latency | Conflict Rate (409) | Notes                        |
|--------------|--------------|--------------|----------------------|-----------------------------|
| With Cache   | ~295–310     | ~19–26 ms    | 45–55%               | Stable under load           |
| Without Cache| ~280–314     | ~19–31 ms    | 35–50%               | Slightly more volatile      |

> ⚠️ Benchmarks were done with `wrk` simulating random user + room bookings. 409 responses are expected due to duplicate booking attempts.

###  What it improves:
- **Less pressure on the database**: only real inserts generate SQL queries
- **More predictable response time**, especially under concurrent load
- **Cleaner validation logic**: handled in memory with `HashSet`, avoiding multiple `SELECT`s


This version is not faster, but it is **more stable** and **more efficient** for long-term scaling.

