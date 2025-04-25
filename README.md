# Room Reservation API Rust + Axum + SQLx

This project is a high-performance REST API for managing room bookings, built in Rust using the [Axum](https://github.com/tokio-rs/axum) web framework and PostgreSQL via [SQLx](https://github.com/launchbadge/sqlx).

---

## Goals

- Apply modern, production-grade backend development practices in Rust
- Learn to structure an efficient, scalable async API
- Benchmark real-world performance under load

---

## Tech Stack

- **Rust**  High-performance systems programming
- **Axum**  Minimal and powerful async HTTP framework
- **SQLx**  Type-safe, async database interactions
- **PostgreSQL**  Relational database
- **wrk**  HTTP benchmarking tool

---

## Features

### Rooms
- Create / update / list / delete rooms

### Users
- Create / update / list / delete users

### Bookings
- Create a room booking for a given date
- Validate room and user existence (via in-memory cache)
- Prevent bookings in the past
- Prevent double-bookings on the same day

---

## Architecture Highlights

- **Full thread safety**: shared state handled via `Arc<>` and [`DashSet`](https://docs.rs/dashmap/latest/dashmap/struct.DashSet.html)
- **Zero Mutex Bottlenecks**: no blocking even under high concurrency
- **Business rule validation**: all rejections are **409 Conflict**, never server errors
- **Predictable performance**: lock-free, multithreaded design scales cleanly

---

## Benchmarks & Performance Insights

All tests were conducted locally using [`wrk`](https://github.com/wg/wrk) with custom Lua script payloads.

### Latest Results

| Version | Threads | Conns | Req/sec | Latency Avg | 2xx | 409 | Notes |
|---------|---------|-------|---------|-------------|-----|-----|-------|
| Mutex   | 4       | 10    | 362     | 22 ms       | 82% | 18% | Acceptable but limited by locking |
| No Mutex + DashSet | 4 | 15 | **1373** | 12 ms | 90% | 10% | 3.7x faster, no server errors |

### Key Observations

- **+280% throughput** after removing mutexes and using lock-free structures
- **Stable and clean** under stress: 0 internal errors (`5xx`), only business logic errors (`409`)
- **Latency cut nearly in half** while multiplying request handling rate
- **High multithread scalability** thanks to efficient cache management

---

## Lua Script Used for Benchmark

```lua
-- post_book.lua
math.randomseed(os.time() + tonumber(string.sub(tostring(os.clock()), 5)))

wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

local base_day = 0
local user_id = 1
local room_id = 1

thread_offset = 0

request = function()
  local offset_days = base_day + thread_offset
  thread_offset = thread_offset + 1

  local timestamp = os.time() + offset_days * 86400
  local date = os.date("!*t", timestamp)
  local formatted_date = string.format("%02d.%02d.%02d", date.day, date.month, date.year % 100)

  local payload = {
    room_name = string.format("ROOM%d", room_id),
    user_name = string.format("user%d", user_id),
    date = formatted_date
  }

  local body = string.format(
    '{"room_name":"%s","user_name":"%s","date":"%s"}',
    payload.room_name, payload.user_name, payload.date
  )

  return wrk.format("POST", "/book", nil, body)
end
```

---

## Conclusion

This project proves that:

- Rust allows you to build **extremely efficient and resilient backends** even with minimal architecture
- **Smart state management** (DashSet, Arc) massively outperforms traditional mutex-based designs
- **Benchmarking early and often** is key to uncovering real performance bottlenecks
- **Business logic integrity** (no invalid bookings) is enforced even at peak concurrency

Whether you're building hobby projects or planning production-ready APIs, Rust's async ecosystem (Axum + SQLx) is a serious contender. 

---

> Built as part of a personal Rust backend learning journey.  
> Feedback, ideas, and improvements are welcome!