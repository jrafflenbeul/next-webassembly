# Was benötigt wird
* Rust
* Node.js

# Extensions für VSC
* Better TOML
* Rust RLS

# Initialisiere neues Rust Projekt

```
cargo new <projectname>
```

# Konfiguriere Cargo.toml

```
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = { version = "0.2", features = ["serde-serialize"] } 
serde = { version = "1.0.125", features = ["derive"] }
serde_json = "1.0.64"
```

# Initialisiere neues Next.js Projekt

```
npx create-next-app <projectname>
```

# Konfiguriere package.json

```
"dependencies": {
    "antd": "^4.15.1",
    "next": "10.1.3",
    "react": "17.0.2",
    "react-dom": "17.0.2",
    "webassembly": "file:../pkg"
},
"devDependencies": {
    "hello-wasm-pack": "^0.1.0"
}
```

### Installiere Dependencies

```npm i```

# Implementiere Rust

```
touch /src/lib.rs
``` 

* Beispiel: *Zugriff auf JS Funktionen*
* Beispiel: *Aufruf der JS Funktionen in Rust*
* Beispiel: *Rust Funktion, welche in JS aufgerufen werden kann*

```
touch /src/invoice.rs
```

* implementiere /src/invoice.rs
* Beispiel: *Rechnungsoperationen in Rust*

# Build Rust

```
wasm-pack build
```

# Implementiere JS
* kopiere webassembly Komponente
* Live Implementierung von ```RustComponent```

