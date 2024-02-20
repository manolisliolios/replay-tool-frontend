# Sui Replay Frontend

This tool generates an HTML file output that displays the transaction effects,
the gas analysis etc.

## Working in development mode

1. Install dependencies:

```
replay-tool-frontend$: pnpm install
```

2. Enable CSS watcher that prepares the CSS from tailwind in real time.

```
pnpm css:dev
```

3. Uncomment this on `index.html` file.
This loads the CSS file locally, while when including it in our binary, we print it out directly on the HTML, so we can move it around without needing
any external dependedncies.

```html
  <link rel="stylesheet" href="styles.css" />
```

4. Replace `const output = REPLACE_ME_WITH_THE_JSON_DATA_FROM_THE_REPLAY;`
with an actual JSON output.
e.g. `const output = {"effects": ...}`

5. Open `index.html` file in a browser.

## Building for the binary

1. Revert steps 3. + 4. from the dev mode. 
(TODO: We need to automate this)
2. Run `pnpm tailwind:css` which will create a minified version of the `styles.css` file.
3. Run: `cargo run` and it'll open up a demo file.



