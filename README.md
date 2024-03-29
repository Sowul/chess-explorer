# tauri-sveltekit-template

Everything you need to build a Tauri app using the SvelteKit frontend framework.

## Developing

Once you've cloned a project and installed dependencies with `pnpm install`, import database:

```bash
surreal import -e file:./src-tauri/bases/test.db -u root -p root --namespace test --database test.db ./src-tauri/bases/database.surql
```

Next you can start a development server:
```bash
pnpm tauri dev
```

## Building

To create a production version of your app, go to <https://tauri.app/v1/guides/building/> and read the docs.

## Docs

<https://tauri.app/v1/guides/getting-started/setup/sveltekit>
