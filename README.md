# const-config-size

This is intended to be used with keeping the size of bounded-size buffers in sync with a JSON config file, but can be used to read a usize const from almost any sort of JSON config.

## usage example

let's say we have a config file `config.json` with these contents:
```json
[ { "a": 1 }, { "a": 2 } ]
```

we'd read the array length of that as a const like this:

```rust
use const_config_size::const_config_size;
use smallvec::SmallVec;

// will be equal to 2
const CONFIG_ITEM_COUNT: usize = const_config_size!("config.json");

// an example of the original intended use - keeping SmallVecs from spilling to heap
type UniqueItemBuffer<T> = SmallVec<[T; CONFIG_ITEM_COUNT]>;
```

it would also work the same with this config file, since there are 2 top-level keys in the object:

```json
{ "a": {}, "b": {} } 
```

### field nesting
if your data is nested in a JSON object, you can provide the path as a `.`-separated string.

for instance, with this `config.json` file:
```json
{
    "data": {
        "inner": [1, 2]
    }
}
```

you would use this invocation to access `data.inner`:

```rust
use const_config_size::const_config_size;

const CONFIG_ITEM_COUNT: usize = const_config_size!(("config.json", "data.inner"));
```