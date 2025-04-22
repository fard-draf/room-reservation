# 📦 Room Reservation API – Rust + Axum + SQLx

This project is a high-performance REST API for managing room bookings, built in Rust using the [Axum](https://github.com/tokio-rs/axum) web framework and PostgreSQL via [SQLx](https://github.com/launchbadge/sqlx).

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

---

## Benchmarks

Tests were performed with [`wrk`](https://github.com/wg/wrk) locally (127.0.0.1) on a homelab with sufficient free resources.

### Results – `POST /book` (with `HashSet` validation + SQL write)

```bash
wrk -t4 -c10 -d10s -s post_book.lua http://localhost:3000/book
```

| Threads | Connections | Req/s | Avg Latency | Non-2xx |
|---------|-------------|-------|-------------|----------|
| 4       | 10          | **320** | 24.92 ms    | 610      |
| 2       | 10          | ~296   | 39 ms       | ~300     |
| 8       | 10          | ~216   | 36 ms       | ~63      |

💡 The backend remains **stable and responsive**, handling hundreds of `POST` requests per second **with strong validation logic** (checking user/room/date).

---

## Lua Script used for the Benchmark

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
  local formatted_date = string.format("%02d.%02d.%02d", d.day, d.month, d.year % 100)

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
- Rust is **extremely performant** for backend development
- A simple architecture (`Axum` + `SQLx`) can sustain high loads
- Code remains **testable, validatable, and maintainable**

---

## Roadmap / Ideas

- Pagination for `/book`
- Rate limiting or auth middleware
- Export reservations to CSV
- Lightweight web interface (Tauri or WebAssembly)

---

## 🧑‍💻 Author

Developed as part of a Rust backend self-learning journey.  
Contributions and feedback are welcome!

