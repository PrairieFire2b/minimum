# Function Structure

The functions of Minimum is modular parts.

## Structure of Architecture

* debugger `(development tool)`
* renderer
* runtime `(functions)`
  * core
  > Core API, IPC, ...63+
  * modules
  > Script API (Bindings), Codecs, ...

## Structure of Source Code

```
Minimum
├── docs/ - Minimum's development documentation to contribute for Minimum.
├── libs/ - Minimum source code.
└── modules/ - MinimumRT module source code.
```
