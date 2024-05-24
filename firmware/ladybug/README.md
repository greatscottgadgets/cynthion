## ladybug

A small library for triggering a logic analyzer from program events.

Define a `LogicAnalyzer` implementation:

```rust
use ladybug::{Channel, LogicAnalyzer};

pub struct LadybugImpl {
    ...
}

impl LogicAnalyzer for LadybugImpl {
    fn high(&self, channel: Channel, bit_number: u8) {
       ...
    }

    fn low(&self, channel: Channel, bit_number: u8) {
       ...
    }
}
```

Log events with `ladybug::trace()`:

```rust

    static LA: LadybugImpl = LadybugImpl::new(...);
    ladybug::set_analyzer(&la);

    ladybug::trace(Channel::B, 0, || {
        ...
    });

```
