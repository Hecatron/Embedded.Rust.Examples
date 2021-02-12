# Options

## Type definitions

For nullable types we can wrap a value in a Option class
```
pub struct BindingSettings {
    pub idf_path: Option<PathBuf>,
    pub idf_target: Option<String>,
    pub idf_linker: Option<String>,
}
```

## Reading / Setting Options

This class can ether hold a value or None
To specify a wrapped value use Some
```
self.idf_path = Some(String::from("test"))
```

To unwrap an option
```
let x = self.idf_path.unwrap()
```

## Matching on a String

```
        match self.idf_target.as_deref() {
            Some("xtensa-esp32-none-elf") => { 
                let linker = env::var("RUSTC_LINKER")
                    .unwrap_or("xtensa-esp32-elf-ld".to_string());
                self.idf_linker = Some(linker);
            },
            Some("xtensa-esp8266-none-elf") => { 
                let linker = env::var("RUSTC_LINKER")
                    .unwrap_or("xtensa-lx106-elf-ld".to_string());
                self.idf_linker = Some(linker);
            }
            _ => {
                warn!("Generating ESP IDF bindings for target '{:?}' it not supported.", self.idf_target.unwrap());
            }
        }
```
