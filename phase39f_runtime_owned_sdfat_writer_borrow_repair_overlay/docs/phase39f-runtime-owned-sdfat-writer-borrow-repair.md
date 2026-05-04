# Phase 39F Runtime-Owned SD/FAT Writer Borrow Repair

The original Phase 39F code called:

```rust
self.record_result(self.ops.write_record_*(...))
```

That creates overlapping mutable borrows of `self` and `self.ops`.

This repair stores the file-operation result in a local first:

```rust
let result = self.ops.write_record_*(...);
self.record_result(result)
```

No behavior changes.
