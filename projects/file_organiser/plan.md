### If you specifically want the flow for `populate_messy_dir`

I'd keep that function's diagram simpler:

```mermaid
flowchart TD
    A(["populate_messy_dir"]) --> B["Receive directory + file count"]

    B --> C{"Directory exists?"}

    C -->|No| D["Create directory"]
    D --> E{"Creation successful?"}
    E -->|No| F["Return Err"]
    E -->|Yes| G["Initialize RNG"]

    C -->|Yes| G["Initialize RNG"]

    G --> H["i = 0"]

    H --> I{"i < count?"}

    I -->|No| J["Return Ok(())"]

    I -->|Yes| K["Choose random extension"]

    K --> L["Generate filename"]

    L --> M["Create dummy file"]

    M --> N{"Creation successful?"}

    N -->|No| F
    N -->|Yes| O["i += 1"]

    O --> I
```

### A simpler version

```mermaid id="21859"
flowchart TD
    A(["Start"]) --> B["Get directory from args"]

    B --> C["Read directory"]

    C --> D{"Next entry?"}

    D -->|No| E(["Done"])

    D -->|Yes| F["Get path"]

    F --> G{"Is file?"}

    G -->|No| D

    G -->|Yes| H["Get extension"]

    H --> I{"Extension exists?"}

    I -->|No| J["others"]
    I -->|Yes| K["Match extension"]

    K --> L["images"]
    K --> M["videos"]
    K --> N["audio"]
    K --> O["documents"]
    K --> P["archives"]
    K --> Q["code"]
    K --> J

    L --> R["Create folder if needed"]
    M --> R
    N --> R
    O --> R
    P --> R
    Q --> R
    J --> R

    R --> S["Build destination path"]

    S --> T["fs::rename(source, destination)"]

    T --> U{"Success?"}

    U -->|Yes| V["Print moved file"]
    U -->|No| W["Print error"]

    V --> D
    W --> D
```

A good Rust mapping would be:

```text
.txt .pdf .docx → documents/
.jpg .png .gif  → images/
.mp3 .wav .flac → audio/
.mp4 .mkv .avi  → videos/
.zip .rar .7z   → archives/
.rs .py .js .ts → code/
everything else → others/
```
