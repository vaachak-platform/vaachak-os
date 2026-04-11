# VaachakOS — Sidecar Format v1

**Status:** Draft v1  
**Purpose:** Offline reading-state exchange for VaachakOS  
**Transport:** File copy to and from SD card  
**Live sync/server dependency:** None

---

## 1. Purpose

The sidecar format exists to move reading state between systems without requiring live sync.

Examples:

- export reading progress from X4 to SD card
- copy that file to a desktop or mobile workflow
- generate or update a sidecar externally
- copy the updated sidecar back to the X4 SD card
- import the newer state on device

This format is intentionally simpler and less fragile than a live multi-device sync protocol.

---

## 2. Scope for v1

### Supported in v1

- book identity
- reading progress
- percent complete
- last updated timestamp
- source label
- optional bookmarks

### Explicitly deferred

- highlights
- notes
- encryption
- signatures
- live conflict resolution beyond simple merge rules
- account identity
- server tokens
- XPath precision anchors

---

## 3. Design Goals

1. **Portable** — can be generated or consumed outside the X4
2. **Human-inspectable** — JSON for v1
3. **Stable across renamed files** — uses book fingerprint, not file path
4. **Easy to reject when mismatched** — fingerprint validation is required
5. **Simple to merge** — explicit rules, no hidden heuristics
6. **Future-extensible** — versioned schema from day one

---

## 4. Storage Location

Recommended path on SD card:

```text
/Vaachak/State/<book_fingerprint>.json
```

Alternative import locations may be supported later, but v1 should treat the above as canonical.

---

## 5. Book Identity

Book identity must not depend on file path.

### Preferred fingerprint order

1. EPUB package identifier if it is present and trustworthy
2. normalized content hash of the source book
3. fallback metadata-based hash only if necessary

### Rules

- fingerprint must be stable across file rename or folder change
- fingerprint must be consistent between exporter and importer
- fingerprint mismatch means the sidecar must be rejected

---

## 6. File Naming

Canonical file name:

```text
<book_fingerprint>.json
```

Examples:

```text
epub_3f6b7b8a1c2d.json
txt_a91d102ef442.json
```

The exact fingerprint encoding may be normalized later, but the file name should remain deterministic and filesystem-safe.

---

## 7. Schema

### Top-level shape

```json
{
  "schema_version": 1,
  "book_fingerprint": "epub_3f6b7b8a1c2d",
  "format": "epub",
  "source": "vaachak-x4",
  "updated_at": 1718000000,
  "progress": {
    "chapter": 12,
    "page": 4,
    "percent": 61.2
  },
  "bookmarks": [
    {
      "id": "bm-1",
      "chapter": 10,
      "page": 3,
      "label": "Important scene",
      "updated_at": 1717999000
    }
  ]
}
```

---

## 8. Required Fields

### `schema_version`

- type: integer
- required: yes
- v1 value: `1`

### `book_fingerprint`

- type: string
- required: yes
- must match the local book fingerprint exactly

### `format`

- type: string
- required: yes
- allowed values in v1:
  - `epub`
  - `txt`

### `source`

- type: string
- required: yes
- examples:
  - `vaachak-x4`
  - `vaachak-mobile`
  - `vaachak-sidecar-tool`

### `updated_at`

- type: integer
- required: yes
- Unix timestamp in seconds
- used for progress merge

### `progress`

- type: object
- required: yes
- must contain reading progress fields

---

## 9. Progress Object

```json
{
  "chapter": 12,
  "page": 4,
  "percent": 61.2
}
```

### Fields

#### `chapter`
- type: integer
- required: yes
- zero-based or one-based indexing must be fixed consistently by implementation
- recommendation for v1: **zero-based internally**, but document clearly in tooling

#### `page`
- type: integer
- required: yes
- page index within the chapter or book-local pagination model

#### `percent`
- type: number
- required: yes
- 0.0 to 100.0
- used as a secondary aid, not as primary identity

### Notes

The v1 sidecar does not attempt render-engine-neutral precision. It represents the best practical portable approximation for the current local-first system.

---

## 10. Bookmarks Array

Bookmarks are optional in v1.

### Bookmark shape

```json
{
  "id": "bm-1",
  "chapter": 10,
  "page": 3,
  "label": "Important scene",
  "updated_at": 1717999000
}
```

### Fields

#### `id`
- type: string
- required: yes
- stable bookmark identifier within the book

#### `chapter`
- type: integer
- required: yes

#### `page`
- type: integer
- required: yes

#### `label`
- type: string
- required: no
- human-visible bookmark label

#### `updated_at`
- type: integer
- required: yes
- used for bookmark merge

---

## 11. Import Rules

When importing a sidecar file:

1. parse JSON
2. verify `schema_version`
3. verify `book_fingerprint` matches the local book
4. verify `format` matches the local book type
5. validate progress values
6. merge progress according to rules below
7. merge bookmarks according to rules below
8. persist local state

### Import rejection conditions

Reject the file if:

- JSON is invalid
- schema version is unsupported
- fingerprint does not match
- format does not match
- required fields are missing
- progress fields are malformed

---

## 12. Export Rules

When exporting a sidecar file:

1. calculate or retrieve the book fingerprint
2. read current local progress
3. read bookmarks if present
4. stamp `updated_at`
5. write canonical JSON to `/Vaachak/State/<fingerprint>.json`

### Export requirements

- export must not depend on network
- export must succeed even when optional bookmark data is absent
- export should be deterministic in field names and shape

---

## 13. Merge Rules

### Progress merge

For v1:

- compare top-level `updated_at`
- newer progress wins
- if timestamps are equal, keep local state

This is intentionally simple and explicit.

### Bookmark merge

For v1:

- bookmarks are keyed by `id`
- if bookmark id does not exist locally, add it
- if bookmark id exists locally, compare bookmark `updated_at`
- newer bookmark wins
- if timestamps are equal, keep local bookmark

### No silent heuristic merge

The importer must not invent merge logic beyond what is defined above.

---

## 14. Versioning Rules

The schema is versioned from day one.

### v1 rule

- importer supports only `schema_version = 1`
- unknown versions are rejected cleanly
- future versions must document compatibility rules explicitly

### Forward compatibility expectation

Future versions may add optional fields, but v1 import/export should keep the schema minimal.

---

## 15. Error Handling

The importer should surface human-readable error reasons when possible.

Examples:

- `unsupported schema version`
- `book fingerprint mismatch`
- `invalid progress payload`
- `malformed bookmark entry`

Errors should not corrupt existing local state.

---

## 16. Security Model

### v1 position

The sidecar format is **not encrypted** and **not authenticated**.

That is acceptable in v1 because the goal is local-first offline exchange, not hostile-network transport.

### Consequence

Users should treat sidecar files as plain reading-state metadata.

### Deferred for later

- encryption
- signing
- tamper detection
- secure transport wrapping

---

## 17. Example Minimal File

```json
{
  "schema_version": 1,
  "book_fingerprint": "epub_3f6b7b8a1c2d",
  "format": "epub",
  "source": "vaachak-x4",
  "updated_at": 1718000000,
  "progress": {
    "chapter": 12,
    "page": 4,
    "percent": 61.2
  }
}
```

---

## 18. Example With Bookmarks

```json
{
  "schema_version": 1,
  "book_fingerprint": "epub_3f6b7b8a1c2d",
  "format": "epub",
  "source": "vaachak-mobile",
  "updated_at": 1718000000,
  "progress": {
    "chapter": 12,
    "page": 4,
    "percent": 61.2
  },
  "bookmarks": [
    {
      "id": "bm-1",
      "chapter": 10,
      "page": 3,
      "label": "Important scene",
      "updated_at": 1717999000
    },
    {
      "id": "bm-2",
      "chapter": 12,
      "page": 1,
      "label": "Start of part two",
      "updated_at": 1717999500
    }
  ]
}
```

---

## 19. Deferred Fields

These fields should not appear in v1 unless the schema is intentionally revised:

- highlights
- note bodies
- XPath positions
- device auth tokens
- sync server URLs
- ciphertext payloads

This keeps v1 tooling and device code small.

---

## 20. Summary

The sidecar format is the offline state-exchange layer for VaachakOS v1.

It is deliberately:

- simple
- file-based
- versioned
- fingerprint-driven
- human-inspectable
- independent of any live server

That simplicity is the main reason it is suitable for the X4-first plan.
