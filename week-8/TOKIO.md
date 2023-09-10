# Tokio

## Tokio의 Thread
### Thread 종류
- Worker Threads: `tokio::spawn`으로 spawn되는 task를 실행하는 쓰레드
  - 생성 시점: 런타임 생성 시 생성됨.
  - 쓰레드의 개수: CPU Core 수와 동일. 개수는 불변.
    - 코어 수 이상의 worker thread는 비효율적이기 때문이라고 함.
    - 단, Worker Threads에서 Blocking이 발생하는 경우에 한하여 코어 수보다 많은 쓰레드 수가 효율적일 수 있음.
  - 쓰레드당 실행 가능한 Task 수: 제한 없음.
- Blocking Threads: `tokio::tasks::spawn_blocking`으로 spawn되는 task를 실행하는 쓰레드
  - 생성 시점: `spawn_blocking`이 호출될 때 생성.
  - 쓰레드의 개수: 기본 0개. 최대 개수의 기본값은 500개.
  - 쓰레드당 실행 가능한 Task 수: 1개
    - 할당 가능한 쓰레드가 없는 경우, 해당 Task를 큐에 보관하다가 Task가 종료된 Thread가 생기면 할당.

## tokio::runtime::Builder
커스텀한 설정과 함께 Tokio 런타임을 빌드할 수 있다. [Docs](https://docs.rs/tokio/1.32.0/tokio/runtime/struct.Builder.html)
```rust
use tokio::runtime::Builder;

fn main() {
    // build runtime
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .max_blocking_threads(100)
        .thread_name("my-custom-name")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

    // use runtime ...
}
```

`TOKIO_WORKER_THREADS` 환경 변수를 통해서도 worker thread의 수를 변경할 수 있다.