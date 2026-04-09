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
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── index.html
├── README.md
└── src
    ├── app.rs
    └── main.rs

2 directories, 8 files
```

  - `src/main.rs`: Entry point for the native binary.
  - `src/lib.rs`: Shared application logic (required for WASM).
  - `src/app.rs`: This is where your UI code and application state live.
  - `assets/`: Folder for icons, fonts, and images.
  - `index.html`: Boilerplate for web rendering.
  - `.gitignore`: Avoid to push on GitHub some files and directories.


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

## 🚀 Why use mimalloc with egui?

Using **mimalloc** (developed by Microsoft) with **egui** is a common and excellent choice for Rust desktop applications. In an "Immediate Mode" GUI like egui, the UI is rebuilt every frame, leading to frequent memory allocations. A performance-oriented allocator can help keep the frame rate stable.

1.  **Lower Latency**: mimalloc is designed to minimize "stop-the-world" moments and fragmentation, which helps prevent micro-stutters in your 60+ FPS UI loop.
2.  **Immediate Mode Friendly**: egui constantly allocates and deallocates small objects (vertex buffers, strings, layout shapes). mimalloc handles these small, short-lived allocations much faster than the default system allocator (especially on Windows).
3.  **Efficiency**: It generally offers a smaller memory footprint over time due to better fragmentation management.

---

## 🛠️ Implementation

Integration is straightforward and only takes a few lines of code.

### 1. Add the dependency
Add this to your `Cargo.toml`:

```toml
[dependencies]
mimalloc = "0.1"
```

----

### 2. Set the Global Allocator
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

### 3. Alternative: jemalloc (?)
`jemalloc` is another popular alternative, often used in heavy Linux server environments. However, for cross-platform desktop apps (Windows/macOS/Linux), **mimalloc** is generally preferred because it is easier to link and performant across all three.


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

**mimalloc:**
- [The Power of jemalloc and mimalloc in Rust — and When to Use Them](https://medium.com/@syntaxSavage/the-power-of-jemalloc-and-mimalloc-in-rust-and-when-to-use-them-820deb8996fe)
- https://crates.io/crates/mimalloc
- https://docs.rs/crate/mimalloc/latest
- https://microsoft.github.io/mimalloc/
- https://github.com/microsoft/mimalloc
- https://github.com/microsoft/mimalloc/issues/140 ⚠

## 🤝 Contributing

Contributions are welcome\! Feel free to open an issue or submit a pull request to improve this starter kit.


----

*Developed with ❤️ using Rust.*

