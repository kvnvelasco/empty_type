# Empty Type 

The [`EmptyType`] trait and [`Container`] trait work together
to create structures with optional members and provides an api 
convert between the two.

```rust
use empty_type::{EmptyType, Empty};

#[derive(EmptyType)]
#[empty(deserialize, fail_safe)]
struct Data {
    key: String,
    mismatch: usize
}

const JSON: &str = r#"{ "key": "value", "mismatch": "not a number" }"#; 


fn main() {
    let empty: Empty<Data> = serde_json::from_str(JSON).unwrap();
    assert!(empty.key.is_some());
    assert!(empty.mismatch.is_none());
    
    let resolved = empty.resolve();
    assert_eq!(resolved.key.as_str(), "value");
    // when the "default" flag is set, even serde errors get resolved
    assert_eq!(resolved.mismatch, 0);
}
```

The proc macro creates code roughly equivalent to the following
```rust
use empty_type::{EmptyType, Container};

struct Data {
    key: String
}

#[derive(Default)]
struct OptionalData {
   key: Option<String>
}

impl EmptyType for Data {
    type Container = OptionalData;
}

impl Container for OptionalData {
#    type Value = Data;
#    
#    fn try_open(&mut self) -> Result<Self::Value, Box<dyn std::error::Error>> {
#        Ok(Data {
#            key: self.key.open()
#        })
#    }
}

// This allows conversion between the two types 

fn main() {
    let mut empty = Data::new_empty();
    empty.key = Some(String::new());
    // should be the default value
    let resolved = empty.resolve();
}
```

## Proc Macro 
The behavior above is tedious and complicated. The `proc_macro` [`EmptyType`] creates 
the optional data structures for you with the feature `derive` enabled


## Serde 
Serde support is provided by the feature flag `serde` and a helper function [`deserialize_empty`]
is provided to deserialize empty values 

```rust
# use empty_type::{EmptyType, deserialize_empty};
# use serde::Deserialize;

#[derive(EmptyType)]
#[empty(deserialize)]
struct Data {
    value: String
}

const JSON: &str = r#" { "value": "data" } "#;

fn use_with_deserializer() {
    let mut de = serde_json::Deserializer::from_str(JSON);
    let empty_value = deserialize_empty::<Data, _>(&mut de).unwrap();
    
    let full_value = empty_value.resolve();
    
    assert_eq!(full_value.value.as_str(), "data");
}

fn use_implicit_deserialization() {
    let value: empty_type::Empty<Data> = serde_json::from_str(JSON).unwrap();
    let full_value = value.resolve();

    assert_eq!(full_value.value.as_str(), "data");
}

# fn main() {
#   use_with_deserializer();
# }
```


## Container 

Container is automatically implemented for [`Option<T>`] and `bool`. This allows 
container unwraps to propagate up through containers.

### Fallible
A special container types [`Fallible`] and [`Optional`] provide small variations to 
the way that types are opened. 

#### Optional 
Optional is a wrapper around source types that are initially option. Optional 
roughly represents `Some(Option<T>)`. Opening this container always results in an [`Option`]
This is distinct from `Option<Option<T>>` as it's impossible for the wrapping option
to be `None` with the semantics described.

```rust
use empty_type::Optional;
struct Data {
    optional_data: Option<String>
}

struct MaybeData {
    optional_data: Optional<Option<String>>
}
```

#### Fallible 
Fallible is similar to Optional except it requires that the underlying type implement [`Default`]. 
The semantics of fallible are to always return the default value of the underlying [`Container`].

Another important distinction is that Fallible will swallow serde Deserialize errors. Any
error in deserialization will result in the default type being emitted.

_