# SPI Bus Arbitration Runtime Ownership

Status marker:

```text
spi_bus_arbitration_runtime_owner=ok
```

This deliverable is the first SPI executor-behavior migration slice after the hardware runtime ownership consolidation.

## What moved to Vaachak

Vaachak now owns the narrow shared-SPI arbitration runtime entrypoint in `target-xteink-x4`:

```text
target-xteink-x4/src/vaachak_x4/physical/spi_bus_arbitration_runtime_owner.rs
```

The moved authority covers:

```text
- logical SPI arbitration runtime ownership
- display vs SD transaction ownership metadata
- safe transaction grant metadata
- user-to-chip-select validation metadata
- shared bus dependency on VaachakSpiBusRuntimeOwner
```

The new owner intentionally records:

```text
SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true
SPI_BUS_ARBITRATION_POLICY_MOVED_TO_VAACHAK = true
ACTIVE_BACKEND_NAME = PulpCompatibility
```

## What remains in Pulp compatibility backend

The active physical executor remains the imported Pulp runtime:

```text
target-xteink-x4/src/vaachak_x4/physical/spi_bus_arbitration_pulp_backend.rs
```

The backend explicitly keeps these unmoved:

```text
- physical SPI transfer execution
- chip-select toggling
- SPI peripheral setup
- display draw/full-refresh/partial-refresh execution
- SD probe/mount execution
- SD/FAT executor behavior
- reader/file-browser behavior
```

## Dependency on SPI runtime ownership

This layer depends on the accepted SPI bus runtime ownership bridge:

```text
VaachakSpiBusRuntimeOwner::ownership_bridge_ok()
```

The arbitration owner uses the accepted Xteink X4 shared SPI map:

```text
SCLK GPIO8
MOSI GPIO10
MISO GPIO7
Display CS GPIO21
SD CS GPIO12
```

## Safety boundary

The arbitration owner is allowed to decide and expose metadata for which logical user is eligible for the next transaction slot. It is not allowed to execute hardware I/O.

Allowed in this slice:

```text
- request metadata
- grant metadata
- user and chip-select validation metadata
- priority metadata for display refresh vs SD probe/mount vs SD/FAT read-only operations
```

Not allowed in this slice:

```text
- embedded-hal or esp-hal SPI imports
- physical SPI transfer calls
- chip-select GPIO toggling
- SD card init/probe/mount execution
- FAT read/write/list implementation changes
- SSD1677 draw/refresh implementation changes
- reader/file browser changes
```

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_spi_bus_arbitration_runtime_owner.sh
cargo build
```

Expected:

```text
spi_bus_arbitration_runtime_owner=ok
```
