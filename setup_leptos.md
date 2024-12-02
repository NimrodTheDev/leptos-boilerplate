# Setup Leptos

1. Install Trunk
```bash 
cargo install trunk
```

2. Create a rust project
```bash
cargo init <project-name>
```

3. `cd` Into your project

4. Set rustup to nightly for the project

```bash
rustup override set nightly
```

5. Add leptos as a dependency
```bash
cargo add leptos --features=csr,nightly
```

6. Add WASM target so rust compiles code to WASM
```bash
rustup target add wasm32-unknown-unknown
```

7. Create an HTML in the root `index.html`

```html
<!DOCTYPE html>
<html>
  <head></head>
  <body></body>
</html>
```

8. Update the main.rs in `src/main.rs`
```bash
use leptos::*;

fn main() {
    mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
```

9. Directory should like below
```bash
leptos_tutorial
├── src
│   └── main.rs
├── Cargo.toml
├── index.html
```

1. Serve project
```bash
trunk serve --open
```

2. If you're having issues building project it's probably becuase of `wasm-bindgen` so use the command below to install
```bash
cargo install wasm-bindgen-cli
```


# To add Tailwind  CSS
- Install tailwindcss
```bash
npm install -D tailwindcss
```

- initialize tailwindcss
```bash
npx tailwindcss init
```

- configure template paths in `tailwind.config.js`
```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./dist/*.{html, js}", "./*.html", "./src/**/*.rs",  "./index.html", "*.html", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

- add tailwind directives to `input.css` in root
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```


- add this to package.json
```json
"scripts": {
  "build:css": "tailwindcss -i ./input.css -o ./dist/output.css"
}
```

- link tailwindcss to `index.html`
```html
<link href="output.css" rel="stylesheet" />
```

- run this to build css
```bash
npm run build:css
```
