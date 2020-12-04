```
$ cargo test

running 1 test
test tests::it_works ... FAILED

failures:

---- tests::it_works stdout ----
thread 'tests::it_works' panicked at 'assertion failed: self.inner.semaphore.is_idle()', /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/sync/mpsc/chan.rs:317:13
stack backtrace:
   0:     0x557b15396af0 - std::backtrace_rs::backtrace::libunwind::trace::h72c2fb8038f1bbee
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
   1:     0x557b15396af0 - std::backtrace_rs::backtrace::trace_unsynchronized::h1e3b084883f1e78c
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/../../backtrace/src/backtrace/mod.rs:66
   2:     0x557b15396af0 - std::sys_common::backtrace::_print_fmt::h3bf6a7ebf7f0394a
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:79
   3:     0x557b15396af0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2e8cb764b7fe02e7
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:58
   4:     0x557b153b902c - core::fmt::write::h7a1184eaee6a8644
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/fmt/mod.rs:1080
   5:     0x557b1530bb66 - std::io::Write::write_fmt::h277fdfd15dc0dc50
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/io/mod.rs:1516
   6:     0x557b15390c3d - std::io::impls::<impl std::io::Write for alloc::boxed::Box<W>>::write_fmt::hc6e3819d814f2f8f
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/io/impls.rs:179
   7:     0x557b1539901d - std::sys_common::backtrace::_print::h1d14a7f6ad632dc8
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:61
   8:     0x557b1539901d - std::sys_common::backtrace::print::h301abac8bb2e3e81
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:48
   9:     0x557b1539901d - std::panicking::default_hook::{{closure}}::hde0cb80358a6920a
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:208
  10:     0x557b15398c72 - std::panicking::default_hook::h9b1a691049a0ec8f
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:224
  11:     0x557b15399701 - std::panicking::rust_panic_with_hook::h2bdec87b60580584
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:577
  12:     0x557b15379f0c - std::panicking::begin_panic::{{closure}}::h4ca5b83cc6f7888a
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:506
  13:     0x557b15379af9 - std::sys_common::backtrace::__rust_end_short_backtrace::h836176ea98c3fa44
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:153
  14:     0x557b15379e37 - std::panicking::begin_panic::h873e7cb8d4b44652
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:505
  15:     0x557b152fb005 - tokio::sync::mpsc::chan::Rx<T,S>::recv::{{closure}}::h04c8805107837f1d
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/sync/mpsc/chan.rs:317
  16:     0x557b152f9b53 - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::h3fc5c57dcf3b08ae
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/loom/std/unsafe_cell.rs:14
  17:     0x557b152fad57 - tokio::sync::mpsc::chan::Rx<T,S>::recv::h5b6c9a80180fd9c6
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/sync/mpsc/chan.rs:290
  18:     0x557b152f04ea - tokio::sync::mpsc::bounded::Receiver<T>::poll_recv::h9a2770b83c240609
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/sync/mpsc/bounded.rs:155
  19:     0x557b152f07e6 - <tokio::sync::mpsc::bounded::Receiver<T> as futures_core::stream::Stream>::poll_next::hf440409e7180e281
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/sync/mpsc/bounded.rs:189
  20:     0x557b152f02ab - futures_util::stream::stream::StreamExt::poll_next_unpin::he64654107e9e23a9
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.8/src/stream/stream/mod.rs:1364
  21:     0x557b152ee722 - channel_repro::tests::it_works::{{closure}}::{{closure}}::{{closure}}::h0b34722f0a61b59e
                               at /home/luke/code/tokio-channel-repro/src/lib.rs:76
  22:     0x557b152f7fc1 - <futures_util::stream::poll_fn::PollFn<F> as futures_core::stream::Stream>::poll_next::h187661a9ac4d770b
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.8/src/stream/poll_fn.rs:54
  23:     0x557b15308038 - <futures_util::stream::stream::map::Map<St,F> as futures_core::stream::Stream>::poll_next::hfa2da8e814b0fe6d
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.8/src/stream/stream/map.rs:59
  24:     0x557b152f5e31 - <futures_util::stream::stream::Inspect<St,F> as futures_core::stream::Stream>::poll_next::hb761b70f6bcbe822
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.8/src/lib.rs:124
  25:     0x557b15307e68 - <futures_util::stream::stream::map::Map<St,F> as futures_core::stream::Stream>::poll_next::hb4711059d5ba4d5c
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.8/src/stream/stream/map.rs:59
  26:     0x557b152f3c5a - <futures_util::stream::stream::fuse::Fuse<S> as futures_core::stream::Stream>::poll_next::h8ee4b6151ba91f36
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.8/src/stream/stream/fuse.rs:54
  27:     0x557b152e5aad - <futures_util::stream::stream::forward::Forward<St,Si,Item> as core::future::future::Future>::poll::h21cb77e4603c3f70
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.8/src/stream/stream/forward.rs:63
  28:     0x557b152f5eac - <futures_util::stream::stream::Forward<St,Si> as core::future::future::Future>::poll::h5ec389cf670972be
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.8/src/lib.rs:113
  29:     0x557b152ee9d0 - channel_repro::tests::it_works::{{closure}}::{{closure}}::h289f06cce1ea0adb
                               at /home/luke/code/tokio-channel-repro/src/lib.rs:65
  30:     0x557b152e4079 - <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h74225d02583725e3
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:80
  31:     0x557b152ed843 - tokio::runtime::task::core::Core<T,S>::poll::{{closure}}::h1c50cf1df49d7d11
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/task/core.rs:173
  32:     0x557b152fa4b7 - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::ha4177241b82d931f
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/loom/std/unsafe_cell.rs:14
  33:     0x557b152ed521 - tokio::runtime::task::core::Core<T,S>::poll::hb9ff0a4ecdcd56dc
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/task/core.rs:158
  34:     0x557b152ea13b - tokio::runtime::task::harness::Harness<T,S>::poll::{{closure}}::h451bf113a61222a4
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/task/harness.rs:107
  35:     0x557b15301070 - core::ops::function::FnOnce::call_once::h27b84d6acf6e4293
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
  36:     0x557b152f448a - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h64a08ca4fb3d8c86
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:308
  37:     0x557b1530948d - std::panicking::try::do_call::h232fb37b2f132949
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:381
  38:     0x557b15309c9d - __rust_try
  39:     0x557b15308e35 - std::panicking::try::h22ef224ed669c277
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:345
  40:     0x557b152f453a - std::panic::catch_unwind::h55f5cd90b38fe5fa
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:382
  41:     0x557b152e9a19 - tokio::runtime::task::harness::Harness<T,S>::poll::h934b796dd02a515a
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/task/harness.rs:89
  42:     0x557b152ff950 - tokio::runtime::task::raw::poll::h1a7138a97d0d3d50
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/task/raw.rs:104
  43:     0x557b1537482f - tokio::runtime::task::raw::RawTask::poll::hf85dbedbbcf7438f
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/task/raw.rs:66
  44:     0x557b1530a3f1 - tokio::runtime::task::Notified<S>::run::hc1739f567335db65
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/task/mod.rs:169
  45:     0x557b152e4bfa - tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::{{closure}}::{{closure}}::hd43a2ab1f86d60ec
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:155
  46:     0x557b152e3b42 - tokio::coop::with_budget::{{closure}}::h3ff6abd671abd923
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/coop.rs:127
  47:     0x557b152f3058 - std::thread::local::LocalKey<T>::try_with::h42cc616b4120e2b4
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:272
  48:     0x557b152f2cb4 - std::thread::local::LocalKey<T>::with::h5749f5393625a5f4
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:248
  49:     0x557b152e4935 - tokio::coop::with_budget::hb9ad52dabd0c1c35
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/coop.rs:120
  50:     0x557b152e4935 - tokio::coop::budget::h7e74fdd97e95f374
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/coop.rs:96
  51:     0x557b152e4935 - tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::{{closure}}::hdad081a62290b9f2
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:155
  52:     0x557b152e504d - tokio::runtime::basic_scheduler::enter::{{closure}}::he23ca5ee973caf9f
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:213
  53:     0x557b15307aff - tokio::macros::scoped_tls::ScopedKey<T>::set::heb1ec62912e331a7
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/macros/scoped_tls.rs:63
  54:     0x557b152e4f36 - tokio::runtime::basic_scheduler::enter::h0c181d115f5618c7
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:213
  55:     0x557b152e42a5 - tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::h6e4013028f4bad3e
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:123
  56:     0x557b152f37f3 - tokio::runtime::Runtime::block_on::{{closure}}::h1bca6d0ed097685b
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/mod.rs:444
  57:     0x557b152f3533 - tokio::runtime::context::enter::h950e46b2963fcf25
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/context.rs:72
  58:     0x557b152f7731 - tokio::runtime::handle::Handle::enter::hd28a00a2975089cd
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/handle.rs:76
  59:     0x557b152f3775 - tokio::runtime::Runtime::block_on::he674958a1358936b
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/mod.rs:441
  60:     0x557b152fe5d4 - channel_repro::tests::it_works::h48b52800605d5478
                               at /home/luke/code/tokio-channel-repro/src/lib.rs:47
  61:     0x557b152ee24a - channel_repro::tests::it_works::{{closure}}::hd74ec9d5a0fd6b7d
                               at /home/luke/code/tokio-channel-repro/src/lib.rs:47
  62:     0x557b1530125e - core::ops::function::FnOnce::call_once::hf2379d5dda04cd19
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
  63:     0x557b15332a03 - core::ops::function::FnOnce::call_once::hb2ff33e91aac5799
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
  64:     0x557b15332a03 - test::__rust_begin_short_backtrace::hc8fd71b19bc78390
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/test/src/lib.rs:516
  65:     0x557b15330ed8 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hb8ab4b7e0de41b66
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/boxed.rs:1042
  66:     0x557b15330ed8 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hc48829c2ba8b55d6
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panic.rs:308
  67:     0x557b15330ed8 - std::panicking::try::do_call::h95feedb4d9e8dd5b
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:381
  68:     0x557b15330ed8 - std::panicking::try::hc4c23f5f7d3bdd43
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:345
  69:     0x557b15330ed8 - std::panic::catch_unwind::hf3a57d903cc5007a
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panic.rs:382
  70:     0x557b15330ed8 - test::run_test_in_process::h09c598ed2a1f6695
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/test/src/lib.rs:543
  71:     0x557b15330ed8 - test::run_test::run_test_inner::{{closure}}::h0723a32c882738ea
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/test/src/lib.rs:449
  72:     0x557b1530aee6 - std::sys_common::backtrace::__rust_begin_short_backtrace::h62223a55784345c9
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:137
  73:     0x557b1530ffe5 - std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}}::h376845da144562d0
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/thread/mod.rs:464
  74:     0x557b1530ffe5 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h8b2c6568b341d210
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panic.rs:308
  75:     0x557b1530ffe5 - std::panicking::try::do_call::hbf1adabf08b467b8
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:381
  76:     0x557b1530ffe5 - std::panicking::try::h9230baf803e9b8a8
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:345
  77:     0x557b1530ffe5 - std::panic::catch_unwind::h677d2c2f6c9d90f0
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panic.rs:382
  78:     0x557b1530ffe5 - std::thread::Builder::spawn_unchecked::{{closure}}::h33e2726c29b1a38b
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/thread/mod.rs:463
  79:     0x557b1530ffe5 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h57985ebe07762a1d
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
  80:     0x557b1539f89a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hbb39a3e615f69ef9
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/boxed.rs:1042
  81:     0x557b1539f89a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h79630a683aed732c
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/boxed.rs:1042
  82:     0x557b1539f89a - std::sys::unix::thread::Thread::new::thread_start::h4afaeade0da13617
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys/unix/thread.rs:87
  83:     0x7ff8806a1590 - start_thread
  84:     0x7ff880922223 - clone
  85:                0x0 - <unknown>
thread 'tests::it_works' panicked at 'called `Result::unwrap()` on an `Err` value: JoinError::Panic(...)', src/lib.rs:100:46
stack backtrace:
   0:     0x557b15396af0 - std::backtrace_rs::backtrace::libunwind::trace::h72c2fb8038f1bbee
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
   1:     0x557b15396af0 - std::backtrace_rs::backtrace::trace_unsynchronized::h1e3b084883f1e78c
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/../../backtrace/src/backtrace/mod.rs:66
   2:     0x557b15396af0 - std::sys_common::backtrace::_print_fmt::h3bf6a7ebf7f0394a
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:79
   3:     0x557b15396af0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2e8cb764b7fe02e7
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:58
   4:     0x557b153b902c - core::fmt::write::h7a1184eaee6a8644
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/fmt/mod.rs:1080
   5:     0x557b1530bb66 - std::io::Write::write_fmt::h277fdfd15dc0dc50
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/io/mod.rs:1516
   6:     0x557b15390c3d - std::io::impls::<impl std::io::Write for alloc::boxed::Box<W>>::write_fmt::hc6e3819d814f2f8f
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/io/impls.rs:179
   7:     0x557b1539901d - std::sys_common::backtrace::_print::h1d14a7f6ad632dc8
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:61
   8:     0x557b1539901d - std::sys_common::backtrace::print::h301abac8bb2e3e81
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:48
   9:     0x557b1539901d - std::panicking::default_hook::{{closure}}::hde0cb80358a6920a
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:208
  10:     0x557b15398c72 - std::panicking::default_hook::h9b1a691049a0ec8f
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:224
  11:     0x557b15399701 - std::panicking::rust_panic_with_hook::h2bdec87b60580584
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:577
  12:     0x557b153992a9 - std::panicking::begin_panic_handler::{{closure}}::h101ca09d9df5db47
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:484
  13:     0x557b15396f5c - std::sys_common::backtrace::__rust_end_short_backtrace::h3bb85654c20113ca
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:153
  14:     0x557b15399269 - rust_begin_unwind
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
  15:     0x557b153b7901 - core::panicking::panic_fmt::h48c31e1e3d550146
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
  16:     0x557b153b7723 - core::option::expect_none_failed::h6154dc750ae47ade
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/option.rs:1234
  17:     0x557b15306ae3 - core::result::Result<T,E>::unwrap::hb6996573ade7a50b
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:973
  18:     0x557b152efa05 - channel_repro::tests::it_works::{{closure}}::h2df70b44202d29ea
                               at /home/luke/code/tokio-channel-repro/src/lib.rs:100
  19:     0x557b152e41b7 - <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::hf79bf734356bc31f
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:80
  20:     0x557b152e4c43 - tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::{{closure}}::{{closure}}::hebca2c050700b249
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:131
  21:     0x557b152e3d76 - tokio::coop::with_budget::{{closure}}::h6d48364641ddae6d
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/coop.rs:127
  22:     0x557b152f32d1 - std::thread::local::LocalKey<T>::try_with::hca0cd55060a713e1
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:272
  23:     0x557b152f2d2d - std::thread::local::LocalKey<T>::with::h692b69c11f8c1f18
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:248
  24:     0x557b152e451f - tokio::coop::with_budget::hbfcc4ffdb8a15e1a
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/coop.rs:120
  25:     0x557b152e451f - tokio::coop::budget::hba73abd14f4fc635
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/coop.rs:96
  26:     0x557b152e451f - tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::{{closure}}::hdad081a62290b9f2
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:131
  27:     0x557b152e504d - tokio::runtime::basic_scheduler::enter::{{closure}}::he23ca5ee973caf9f
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:213
  28:     0x557b15307aff - tokio::macros::scoped_tls::ScopedKey<T>::set::heb1ec62912e331a7
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/macros/scoped_tls.rs:63
  29:     0x557b152e4f36 - tokio::runtime::basic_scheduler::enter::h0c181d115f5618c7
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:213
  30:     0x557b152e42a5 - tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::h6e4013028f4bad3e
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/basic_scheduler.rs:123
  31:     0x557b152f37f3 - tokio::runtime::Runtime::block_on::{{closure}}::h1bca6d0ed097685b
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/mod.rs:444
  32:     0x557b152f3533 - tokio::runtime::context::enter::h950e46b2963fcf25
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/context.rs:72
  33:     0x557b152f7731 - tokio::runtime::handle::Handle::enter::hd28a00a2975089cd
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/handle.rs:76
  34:     0x557b152f3775 - tokio::runtime::Runtime::block_on::he674958a1358936b
                               at /home/luke/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.23/src/runtime/mod.rs:441
  35:     0x557b152fe5d4 - channel_repro::tests::it_works::h48b52800605d5478
                               at /home/luke/code/tokio-channel-repro/src/lib.rs:47
  36:     0x557b152ee24a - channel_repro::tests::it_works::{{closure}}::hd74ec9d5a0fd6b7d
                               at /home/luke/code/tokio-channel-repro/src/lib.rs:47
  37:     0x557b1530125e - core::ops::function::FnOnce::call_once::hf2379d5dda04cd19
                               at /home/luke/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
  38:     0x557b15332a03 - core::ops::function::FnOnce::call_once::hb2ff33e91aac5799
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
  39:     0x557b15332a03 - test::__rust_begin_short_backtrace::hc8fd71b19bc78390
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/test/src/lib.rs:516
  40:     0x557b15330ed8 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hb8ab4b7e0de41b66
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/boxed.rs:1042
  41:     0x557b15330ed8 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hc48829c2ba8b55d6
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panic.rs:308
  42:     0x557b15330ed8 - std::panicking::try::do_call::h95feedb4d9e8dd5b
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:381
  43:     0x557b15330ed8 - std::panicking::try::hc4c23f5f7d3bdd43
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:345
  44:     0x557b15330ed8 - std::panic::catch_unwind::hf3a57d903cc5007a
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panic.rs:382
  45:     0x557b15330ed8 - test::run_test_in_process::h09c598ed2a1f6695
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/test/src/lib.rs:543
  46:     0x557b15330ed8 - test::run_test::run_test_inner::{{closure}}::h0723a32c882738ea
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/test/src/lib.rs:449
  47:     0x557b1530aee6 - std::sys_common::backtrace::__rust_begin_short_backtrace::h62223a55784345c9
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys_common/backtrace.rs:137
  48:     0x557b1530ffe5 - std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}}::h376845da144562d0
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/thread/mod.rs:464
  49:     0x557b1530ffe5 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h8b2c6568b341d210
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panic.rs:308
  50:     0x557b1530ffe5 - std::panicking::try::do_call::hbf1adabf08b467b8
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:381
  51:     0x557b1530ffe5 - std::panicking::try::h9230baf803e9b8a8
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:345
  52:     0x557b1530ffe5 - std::panic::catch_unwind::h677d2c2f6c9d90f0
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panic.rs:382
  53:     0x557b1530ffe5 - std::thread::Builder::spawn_unchecked::{{closure}}::h33e2726c29b1a38b
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/thread/mod.rs:463
  54:     0x557b1530ffe5 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h57985ebe07762a1d
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
  55:     0x557b1539f89a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hbb39a3e615f69ef9
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/boxed.rs:1042
  56:     0x557b1539f89a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h79630a683aed732c
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/boxed.rs:1042
  57:     0x557b1539f89a - std::sys::unix::thread::Thread::new::thread_start::h4afaeade0da13617
                               at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/sys/unix/thread.rs:87
  58:     0x7ff8806a1590 - start_thread
  59:     0x7ff880922223 - clone
  60:                0x0 - <unknown>


failures:
    tests::it_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```
