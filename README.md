# egui Cross-Platform Starter Kit 🦀🚀

A minimalist, **[ready-to-use template](https://github.com/Laugharne/egui_cross_platform_starter_kit)** for building high-performance graphical applications in **Rust** using the **egui** library. This kit is designed to compile seamlessly for both **Native** (Windows, macOS, Linux) and **WebAssembly** (WASM).


## ✨ Features

  - **Cross-Platform**: Single codebase for Desktop (via `eframe`) and Web (via WASM).
  - **Immediate Mode**: Reactive and easy-to-code user interface.
  - **Dark/Light Support**: Built-in egui native themes.
  - **State Persistence**: Automatic app state saving (optional/configurable).
  - **Optimized Workflow**: Ready-made configuration for web deployment.


  ## 🛠️ Prerequisites

Before you begin, ensure you have the following installed:

1.  **Rust**: [https://rustup.rs/](https://rustup.rs/)
2.  **WASM Target** (for web builds):
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
3.  **Trunk** (the build tool for web):
    ```bash
    cargo install --locked trunk
    ```


## 🚀 Getting Started

### 🖥️ Run as a Native Desktop App

To launch the application on your system (Linux, macOS, Windows):

```bash
cargo run --release
```

### 🌐 Run as a Web App (WASM)

To compile and serve the application in your browser:

```bash
trunk serve
```

Then, open your browser at: `http://127.0.0.1:8080`


## 📂 Project Structure

```
.
├── assets
│   └── icon.png
│── src
│   ├── app.rs
│   └── main.rs
│── .gitignore
├── Cargo.toml
├── index.html
└── README.md

2 directories, 7 files
```

  - `assets/`: Folder for icons, fonts, and images.
  - `src/main.rs`: Entry point for the native binary.
  - `src/app.rs`: This is where your UI code and application state live.
  - `.gitignore`: Avoid to push on GitHub some files and directories.
  - `index.html`: Boilerplate for web rendering.


## 🏗️ Web Deployment

To generate static files ready for production (GitHub Pages, Netlify, Vercel, etc.):

```bash
trunk build --release
```

The output files will be located in the `dist/` directory.


## ⚠️ Potential Causes for Mouse Offset with egui/Web

### 1. **CSS on the Canvas (Most Frequent Cause)**

egui uses `getBoundingClientRect()` to calculate the mouse position. If the canvas has **padding**, a **border**, **margins**, or a **CSS transform**, the coordinates will be skewed.

**CSS Fix:** Ensure the canvas has no unintentional offsets:

```css
canvas {
    display: block;  /* Prevents inline space below the canvas */
    margin:  0;
    padding: 0;
    border:  none;
    /* Avoid transform: translate(...) */
}

body {
    margin:   0;
    padding:  0;
    overflow: hidden;
}
```

----

### 2. **Incorrect `devicePixelRatio` / Logical vs. Physical Pixel Confusion**

eframe can sometimes enter a resizing loop where it confuses logical pixels (CSS) and physical pixels (device), causing coordinate offsets.

The egui canvas has **two sizes**:
  - `canvas.width/height` → Physical pixels (rendering resolution)
  - `canvas.style.width/height` → CSS pixels (displayed size)

If these two dimensions do not match correctly via the `devicePixelRatio`, mouse coordinates (which are always in CSS pixels) will be misinterpreted.

**HTML Index Fix:** Force the canvas to occupy exactly the expected CSS space:

```html
<style>
    html, body {
        margin:   0;
        padding:  0;
        overflow: hidden;
        height:   100%;
    }
    canvas {
        display: block;
        width:   100%;
        height:  100%;
    }
</style>
```

----

### 3. **Canvas Embedded with Other HTML Elements**

When the canvas is embedded within a larger page and has a border or padding, egui uses `getBoundingClientRect()` but might not correctly subtract these values. This bug was fixed in recent versions of eframe, ensure you are using **eframe ≥ 0.28**.

----

### 4. **Firefox with `privacy.resistFingerprinting`**

Firefox with `privacy.resistFingerprinting=true` (often enabled by "Enhanced Tracking Protection") sends spoofed mouse coordinates to the application, causing this exact offset. The user-side solution is to disable this setting for the site or set `privacy.resistFingerprinting` to `false` in `about:config`.

This specific case cannot be fixed within the egui code itself.

----

### 5. **Browser Zoom Levels (Not 100%)**

If the user has zoomed in/out in their browser, the `devicePixelRatio` changes. This can cause offsets if egui doesn't recalibrate correctly. While eframe usually handles this automatically, ensure you are not manually overriding `pixels_per_point` with a fixed value:

```rust
// ❌ Avoid this if the zoom level can vary
// ctx.set_pixels_per_point(2.0);

// ✅ Let eframe handle it automatically
```

## 🧠 Why use mimalloc with egui?

Using **mimalloc** (developed by Microsoft) with **egui** is a common and excellent choice for Rust desktop applications. In an "Immediate Mode" GUI like egui, the UI is rebuilt every frame, leading to frequent memory allocations. A performance-oriented allocator can help keep the frame rate stable.

1.  **Lower Latency**: mimalloc is designed to minimize "stop-the-world" moments and fragmentation, which helps prevent micro-stutters in your 60+ FPS UI loop.
2.  **Immediate Mode Friendly**: egui constantly allocates and deallocates small objects (vertex buffers, strings, layout shapes). mimalloc handles these small, short-lived allocations much faster than the default system allocator (especially on Windows).
3.  **Efficiency**: It generally offers a smaller memory footprint over time due to better fragmentation management.

---

### Implementation

Integration is straightforward and only takes a few lines of code.

#### 1. Add the dependency
Add this to your `Cargo.toml`:

```toml
[dependencies]
mimalloc = "0.1"
```

----

#### 2. Set the Global Allocator
In your `main.rs` (or `lib.rs`), declare it as the global allocator. This must be done at the root of the file, outside of any function.

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui App with mimalloc",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    ).expect("Failed to run app");
}
```

**Requirements**

A **C** compiler is required for building [mimalloc](https://github.com/microsoft/mimalloc) with cargo.

#### [](https://crates.io/crates/mimalloc#usage-with-secure-mode)Usage with secure mode

Using secure mode adds guard pages, randomized allocation, encrypted free lists, etc. The performance penalty is usually around 10% according to [mimalloc](https://github.com/microsoft/mimalloc) own benchmarks.

To enable secure mode, put in `Cargo.toml`:

```toml
[dependencies]
mimalloc = { version = "*", features = ["secure"] }
```

#### [](https://crates.io/crates/mimalloc#usage-with-v3)Usage with v3

By default this library uses `mimalloc v2`. To enable `v3`, put in `Cargo.toml`:

```toml
[dependencies]
mimalloc = { version = "*", features = ["v3"] }
```

----

## ⚠️ Important Considerations

### 1. WebAssembly (WASM) Warning (!)
**Do not use mimalloc for the WASM target.**
WASM environments manage memory differently, and mimalloc either won't compile or won't provide any benefit. You should use **conditional compilation** to keep it desktop-only:

```rust
#[cfg(not(target_arch = "wasm32"))]
use mimalloc::MiMalloc;

#[cfg(not(target_arch = "wasm32"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

For web applications, the size of the `.wasm` file is a critical performance metric (Load Time).

  - Adding `mimalloc` (a large C library) significantly increases the binary size.
  - In the WASM community, the trend is actually toward "tiny" allocators like `wee_alloc` (though it is now unmaintained, it was designed to be the opposite of mimalloc: prioritizing size over speed).

----

### 2. Real-World Impact
While mimalloc is fast, it isn't a "magic wand" for performance:
- **CPU usage**: You might see a **5% to 15%** reduction in time spent on memory tasks.
6 **Frame Consistency**: The biggest win is usually the **reduction of frame-time spikes** (jitter), making the scrolling and animations feel smoother.

----

### 3. Alternative: `jemalloc` (?)
`jemalloc` is another popular alternative, often used in heavy Linux server environments. However, for cross-platform desktop apps (Windows/macOS/Linux), **mimalloc** is generally preferred because it is easier to link and performant across all three.

----


## ✨ Optmizations

### 1. Optimize `Cargo.toml`

```toml
[package]
name = "my_egui_application"
version = "0.1.0"
edition = "2021"

[dependencies]
# Disable unnecessary default features to reduce dependencies
eframe = { version = "0.27", default-features = false, features = [
    "accesskit",     # Accessibility
    "default_fonts", # Basic fonts (essential)
    "glow",          # Rendering via OpenGL (lighter than WGPU natively)
    "wayland",       # For Linux
    "x11",           # For Linux
] }

[profile.release]
# 's' is often a better trade-off than 'z' for GUIs
# because 'z' can slow down graphical rendering too much.
opt-level     = "s"
lto           = true
codegen-units = 1
panic         = "abort"
strip         = true

# CRUCIAL OPTIMIZATION: Optimize dependencies as much as possible
# even in debug mode or if the main profile is set to 's' or 'z'.
[profile.release.package."*"]
opt-level = 3
```


### 2. Minify Features in `Cargo.toml`

`egui` and `eframe` come with default features (like extra fonts or image formats) that you might not use. You can disable them to save space.

```toml
[dependencies]
egui   = { version = "0.27", default-features = false, features = ["default_fonts"] }
eframe = { version = "0.27", default-features = false, features = ["wgpu", "glow"] }
```

####  Optimizing Egui Native

To compile an **egui** application (generally using `eframe`) into a native binary with a minimal output profile, you need to be a bit more cautious. Unlike a command-line utility, a graphical application depends on heavy system libraries and font/image management.

Below is a suggested `Cargo.toml` optimized for native builds, balancing **binary size** and **rendering performance** (as a GUI must remain fluid).

```toml
[package]
name = "my_egui_app"
version = "0.1.0"
edition = "2021"

[dependencies]
# Disable unnecessary default features to reduce dependencies
eframe = { version = "0.27", default-features = false, features = [
    "accesskit",     # Accessibility
    "default_fonts", # Basic fonts (essential)
    "glow",          # Rendering via OpenGL (lighter than WGPU for native)
    "wayland",       # For Linux
    "x11",           # For Linux
] }

[profile.release]
# 's' is often a better compromise than 'z' for GUIs
# because 'z' can slow down graphical rendering too much.
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"
strip = true

# CRUCIAL OPTIMIZATION: Maximize optimization for dependencies
# even if the main profile is set to 's' or 'z'.
[profile.release.package."*"]
opt-level = 3
```

**Choosing `opt-level = "s"` instead of `"z"`**

For a graphical application, smoothness (60 FPS) is the priority. The `"z"` optimization can sometimes break critical loop optimizations required for pixel rendering. `"s"` seeks a compromise: reducing size without brutally sacrificing execution speed.

**The `[profile.release.package."*"]` trick**

This is the "secret" for complex Rust projects. It tells Cargo: *"Optimize my own functions for size, but compile all external libraries (like the graphical rendering engine) with maximum optimization (`3`)."* This keeps the interface ultra-responsive while reducing the weight of your business logic.

**Feature selection in `eframe`**

By default, `eframe` often includes `wgpu` for rendering. It is powerful but **very heavy** in terms of binary size (as it includes complex shader compilers).
* By using **`glow`** (OpenGL), your binary will be significantly lighter.
* Remember to disable `default-features` to keep only what is strictly necessary for your target platform.

**Caution with `panic = "abort"` and windows**

Using `panic = "abort"` is excellent for size, but keep in mind that in the event of a crash, the application will close instantly without leaving console logs or a proper error window. For native apps, this is often acceptable.

**A final tip for image weight**

If you display images in your egui application, use the **WebP** format or ensure you compress your assets before compilation, as they are often included directly in the binary via `include_bytes!`.

----

### 3. Shrinking .wasm Size

#### Optimize `Cargo.toml`

The most significant gains come from telling the compiler to prioritize binary size.

```toml
[profile.release]
# Optimize for size ('z' is more aggressive than 's')
opt-level = "z"

# Enable Link Time Optimization (LTO) to remove dead code across crates
lto = true

# Reduce parallel compilation to allow deeper optimization
codegen-units = 1

# Strip symbols and debug info from the binary
strip = true

# Immediately panic without stack unwinding (saves space)
panic = "abort"
```

----

#### Post-processing with `wasm-opt`

`wasm-opt` is part of the **Binaryen** toolkit. It performs passes on the generated WASM file that the Rust compiler cannot do. It can often reduce the size by another **20% to 40%**.

**Command:**
```bash
wasm-opt -Oz -o output_optimized.wasm input.wasm
```

See: [Binaryen (wasm-opt) on GitHub](https://github.com/WebAssembly/binaryen)

----

#### Use Compression (Brotli/Gzip)

This is the most effective way to reduce transfer size. WASM files are highly compressible. A 5MB file can often be served at around 1.2MB using Brotli.

- Brotli: Best compression ratio for web assets.
- Gzip: Faster but slightly larger than Brotli.
- Reference: [MDN - Content-Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding)

----

#### Analyze the Binary with `twiggy`

If your file is still too large, use twiggy to find out exactly which functions or libraries are taking up the most space.

```bash
cargo install twiggy
twiggy top -n 20 your_file.wasm
```

Reference: [Twiggy Documentation](https://rustwasm.github.io/twiggy/)

----

#### Trunk spécific

**Enabling `wasm-opt` in Trunk**

Trunk has built-in support for `wasm-opt`. If you have the tool installed on your system (or if Trunk downloads it automatically), it will run as part of the `--release` build.

**In your `Trunk.toml` (or as command line flags):** You don't usually need to change anything if you run with the release flag, but you can verify it:

```Bash
trunk build --release
```

Trunk will look for `wasm-opt` in your path. If it's missing, you can install it via your package manager (e.g., `brew install binaryen` or `sudo apt install binaryen`).

- **Reference:** [Trunk Documentation - Tools](https://trunkrs.dev/tools/%23wasm-opt)


**Trunk Asset Pipeline (Hashing and Minification)**

Trunk automatically handles cache busting by adding hashes to your `.wasm` filenames. This allows you to set long-term "Immutable" cache headers on your server, which improves perceived performance for returning users.

If you have a `index.html` file, ensure your link to the WASM/JS is handled by Trunk:

```html
<link rel="rust" data-bin="my_app" data-wasm-opt="z" />
```

The `data-wasm-opt="z"` attribute tells Trunk specifically which optimization level to pass to the optimizer.


**Automatic Brotli/Gzip with `trunk serve`**

When you use `trunk serve`, it doesn't necessarily compress files (as it's meant for local dev). However, for production, you should use the output of `trunk build --release` (the `dist/` folder) and serve it with a web server that supports compression.

**Pro-Tip: GitHub Pages / Vercel / Netlify** If you deploy your `dist/` folder to these platforms, they automatically apply **Gzip** or **Brotli** compression to `.wasm` files. You don't have to do anything!

**Final Cargo.toml for Trunk Users**

To make sure Trunk has the best "raw material" to work with, your `Cargo.toml` should look exactly like this for production:

```toml
[profile.release]
opt-level     = "z"     # Optimize for size
lto           = true    # Link Time Optimization
codegen-units = 1       # Maximum optimization potential
panic         = "abort" # Remove stack unwinding code
strip         = true    # Remove all symbols/debug info
```

- **Reference:** [The Rust Wasm Book - Shrinking .wasm Size](https://rustwasm.github.io/docs/book/reference/shrinking-size.html)

----



## 💾 Persistant storage

- On **desktop**, the state is saved in the [**RON format**](https://github.com/ron-rs) within the system configuration file (`~/.local/share/<app_name>/app.ron` on **Linux**).
- On the **web**, eframe uses the browser's `localStorage`.

----



## 📝 Resources

**Tools:**
- [egui Documentation](https://docs.rs/egui)
- [eframe Documentation](https://docs.rs/eframe)
- [egui Github](https://github.com/emilk/egui)
- [Trunk Documentation](https://trunkrs.dev/)
- [egui Web Demo](https://www.egui.rs/)

**Tutorials:**
- [Rust GUI with Neowin - YouTube](https://www.youtube.com/playlist?list=PLOeWRYj1QznUX08O4K1Ibh1YM9G_ew6iM)
- [GoCelesteAI / Repositories · GitHub](https://github.com/GoCelesteAI?tab=repositories&q=EGUI&type&language&sort)

**WASM:**
- https://rustwasm.github.io/

**mimalloc:**
- [The Power of jemalloc and mimalloc in Rust — and When to Use Them](https://medium.com/@syntaxSavage/the-power-of-jemalloc-and-mimalloc-in-rust-and-when-to-use-them-820deb8996fe)
- [crates.io: Rust Package Registry](https://crates.io/crates/mimalloc)
- [mimalloc 0.1.48 - Docs.rs](https://docs.rs/crate/mimalloc/latest)
- [GitHub - mi-malloc: mi-malloc](https://microsoft.github.io/mimalloc/)
- [microsoft/mimalloc: mimalloc is a compact general purpose allocator with excellent performance.](https://github.com/microsoft/mimalloc)
- [Link with -lrt for older glibc by jserv · Pull Request #140 · microsoft/mimalloc](https://github.com/microsoft/mimalloc/issues/140) ⚠

**misc...**
- [GitHub - ron-rs/ron: Rusty Object Notation](https://github.com/ron-rs/ron)


## 🤝 Contributing

Contributions are welcome ! Feel free to open an issue or submit a pull request to improve this starter kit.


----

*Developed with ❤️ using Rust.*

