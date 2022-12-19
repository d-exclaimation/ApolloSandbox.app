# ApolloSandbox.app

> **Warning**
>
> This is not the official Apollo Sandbox App, this is just a rust wrapper
> around the Embeddable Apollo Explorer (https://github.com/apollographql/embeddable-explorer)

## Installing

### Check release builds

**Supported platforms**
- macOS Universal

### Building manually

1. Clone this repository

2. Install `tauri-cli`

```sh
cargo install tauri-cli
```

3. Run the build command

```sh
cargo tauri build
```

_This should build for your platform_

### Limitation

While the application is running natively as a desktop app, it seemed like the webview is running under the origin `tauri://localhost`

This meant your GraphQL server must have CORS functionally and allow the origin `tauri://localhost`
