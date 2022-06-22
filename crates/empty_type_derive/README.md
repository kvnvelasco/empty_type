# EmptyType Derive Macro

Used to derive a corresponding [`Container`] for an [`EmptyType`] implementation.
Will roughly produce equivalent code to:

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
    
    fn new_container() -> Self::Container {
        OptionalData {
            key: None
        }       
    }
}

impl Container for OptionalData {
    type Value = Data;
    
    fn try_open(&mut self) -> Result<Self::Value, Box<dyn std::error::Error>> {
        Ok(Data {
            key: self.key.open()
        })
    }
}
```