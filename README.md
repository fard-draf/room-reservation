# 📦 Room Reservation API – Rust + Axum + SQLx

This project is a high-performance REST API for managing room bookings, built in Rust using the [Axum](https://github.com/tokio-rs/axum) web framework and PostgreSQL via [SQLx](https://github.com/launchbadge/sqlx).

---

## Goals

- Learn and apply modern backend development in Rust
- Handle HTTP requests, database access, and business logic validations
- Benchmark real-world performance of an async Rust backend

---

## Tech Stack

- **Rust** – Main programming language
- **Axum** – Asynchronous HTTP web framework
- **SQLx** – Type-safe, async ORM for PostgreSQL
- **PostgreSQL** – Relational database
- **wrk** – HTTP benchmarking tool

---

## Features

### Rooms
- Create / update / list / delete rooms

### Users
- Create / update / list / delete users

### Bookings
- Create a room booking for a given date
- Validate that room and user exist (via `HashSet`)
- Prevent bookings in the past
- Prevent double-booking on the same date

---

## Benchmarks & Performance Insights

All tests were conducted locally using [`wrk`](https://github.com/wg/wrk) with Lua script payloads.

### Summary of Key Metrics

| Threads | Conns | Req/sec | Latency Avg | 2xx | 409 | Notes |
|---------|-------|---------|-------------|-----|-----|-------|
| 4       | 10    | **362** | 22 ms       | 82% | 18% | Optimal load point |
| 8       | 20    | 370     | 47 ms       | 69% | 30% | Latency rising     |
| 8       | 80    | 387     | 204 ms      | 71% | 29% | Saturation reached |

### 📊 Analysis

- The API consistently delivers **350+ req/sec** under realistic load.
- At higher concurrency (`-c80`), **latency explodes** without meaningful gain in throughput.
- Logical conflicts (`409`) rise with parallelism — **integrity is preserved**.
- **No `5xx` errors** at any level → backend remains technically stable.

### 🔹 Graph (textual)

```
Load (conn)  →    10     20     80
Req/s        →   362    370    387
Latency (ms) →    22     47    204
```

---

## 📃 Lua Script Used for Benchmark

```lua
-- post_book.lua
math.randomseed(os.time())

wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

request = function()
  local room_id = math.random(0, 49)
  local user_id = math.random(0, 49)
  local offset_days = math.random(0, 30)

  local now = os.time()
  local d = os.date("!*t", now + offset_days * 86400)
  local formatted_date = string.format("%02d.%02d.%02d", d.day, d.month, d.year %% 100)

  local body = string.format(
    '{"room_name":"ROOM%d","user_name":"user%d","date":"%s"}',
    room_id, user_id, formatted_date
  )

  return wrk.format("POST", "/book", nil, body)
end
```

---

## Conclusion

This API demonstrates that:

- Rust is a top-tier choice for building fast and safe backends
- Even with a simple architecture (Axum + SQLx), high concurrency can be handled
- Data integrity and validation are preserved even under load
- Benchmarking with `wrk` helps identify optimal concurrency levels and limits



Built as part of a self-learning journey in Rust backend development.
Feedback, suggestions, and contributions are welcome!

