# Trunk References Audit Report

This report identifies all references to Trunk in the documentation files that need to be updated to Dioxus CLI.

## Summary

Found Trunk references in 6 out of 7 documentation files checked. Most references are in build commands, configuration mentions, and CI/CD scripts.

## Detailed Findings

### 1. **reports/wasm-optimization-report.md**
- **Line 45**: `#### Trunk Configuration`
- **Line 46**: `- **Custom Build Script**: `build-optimized.sh``
- **Line 199**: `*Tools Used: wasm-opt, cargo-bloat, twiggy, trunk*`

**Update needed**: Yes - References to Trunk configuration and tools list

### 2. **articles/e2e-testing-with-playwright.md**
- **Line 24**: `trunk serve`
- **Line 27**: `# http://localhost:8080 にアクセスして自動テスト開始`
- **Line 185**: `- name: Install Trunk`
- **Line 186**: `run: cargo install trunk`
- **Line 193**: `trunk build --release`
- **Line 194**: `trunk serve --release &`

**Update needed**: Yes - All trunk serve/build commands need to be updated to dx serve/build

### 3. **reports/poc-results-report.md**
- **Line 74**: `ビルドツール = "Trunk"`
- **Line 193**: `trunk serve`
- **Line 206**: `trunk serve --open /benchmark/canvas/10000`

**Update needed**: Yes - Build tool reference and serve commands

### 4. **reports/rust-dioxus-evaluation.md**
- **Line 32**: `- Trunkの自動リロードが高速（通常1-2秒）`

**Update needed**: Yes - Reference to Trunk's auto-reload feature

### 5. **articles/wasm-optimization-techniques.md**
- **Line 66**: `trunk build --release`

**Update needed**: Yes - Build command

### 6. **articles/index.md**
- **Line 37**: `- **ビルド**: Trunk 0.21.14`

**Update needed**: Yes - Build tool version reference

### 7. **articles/development-tips.md**
- **Line 13**: `mise use cargo:trunk@latest`
- **Line 17**: `### wasm-packとTrunkの競合回避策`
- **Line 19**: `- Trunk.tomlでwasm-opt無効化でビルド競合解消`
- **Line 45**: `- **症状**: `trunk serveでdist/tailwind.cssが見つからない`
- **Line 47**: `- **対処**: `npm run build-css`実行後に`trunk serve`

**Update needed**: Yes - Multiple references to Trunk installation, configuration, and troubleshooting

## Recommended Actions

1. **Build Commands**: Replace all `trunk serve` with `dx serve` and `trunk build` with `dx build`
2. **Installation**: Update `cargo install trunk` to `cargo install dioxus-cli`
3. **Configuration**: References to Trunk.toml should be updated to Dioxus.toml
4. **Tool Names**: Update "Trunk" to "Dioxus CLI" in descriptive text
5. **Version References**: Update from "Trunk 0.21.14" to appropriate Dioxus CLI version

## Files Not Requiring Updates

The following file was not checked as it's the migration guide itself:
- docs/articles/trunk-to-dioxus-migration.md

## Notes

- Some historical references (like in optimization reports) might be kept as-is if they document what was actually used during the POC
- Consider adding notes about the migration where appropriate
- Update any scripts that reference trunk commands