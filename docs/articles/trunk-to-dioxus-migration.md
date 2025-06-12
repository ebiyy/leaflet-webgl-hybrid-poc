# TrunkからDioxus CLIへの移行ガイド

## 概要

2025年6月13日、本プロジェクトはビルドツールをTrunkからDioxus CLIに移行しました。この移行により、GitHub Pagesデプロイの問題が解決し、ビルド時間が大幅に短縮されました。

## 移行の背景

### 課題
1. **GitHub Pagesデプロイ問題**: Trunkはサブディレクトリデプロイに対する十分なサポートがなく、パスの手動修正が必要でした
2. **ビルド時間**: CI/CDでのビルド時間が5分以上かかっていました
3. **外部リソース管理**: Leaflet.jsやPixi.jsなどのCDNリソースの管理が煩雑でした

### 解決策
Dioxus CLI 0.6.3への移行により、これらの問題がすべて解決されました：
- `base_path`設定によるサブディレクトリデプロイの自動対応
- `taiki-e/install-action`によるCI/CDの高速化（5分→2分）
- `web.resource`設定による外部リソースの一元管理

## 主な変更点

### 1. ビルドコマンドの変更

```bash
# 旧（Trunk）
trunk serve
trunk build --release

# 新（Dioxus CLI）
dx serve
dx build --platform web --release
```

### 2. 設定ファイルの変更

#### 削除されたファイル
- `Trunk.toml` - Dioxus.tomlに置き換え
- `scripts/build-optimized.sh` - Dioxus内蔵の最適化機能を使用
- `scripts/build-quiet.sh` - 不要になりました

#### 新規作成されたファイル
- `Dioxus.toml` - プロジェクト設定ファイル

### 3. Dioxus.toml設定の詳細

```toml
[project]
name = "leaflet-webgl-hybrid-poc"
authors = ["ebiyy"]

[web.app]
# GitHub Pagesのサブディレクトリデプロイ対応
base_path = "leaflet-webgl-hybrid-poc"

[web.watcher]
index_on_404 = true  # SPAルーティング対応

[[hooks.build]]
stage = "pre"
command = "npm"
args = ["run", "build-css"]  # Tailwind CSSビルド

[web.resource]
# 外部リソースの一元管理
dev = [
    { rel = "script", src = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" },
    { rel = "script", src = "https://cdn.jsdelivr.net/npm/pixi.js@8.6.5/dist/pixi.min.js" },
    { rel = "stylesheet", href = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" },
    { rel = "stylesheet", href = "./assets/tailwind-generated.css" }
]
```

### 4. ビルド出力ディレクトリの変更

```bash
# 旧（Trunk）
dist/

# 新（Dioxus CLI）
target/dx/leaflet-webgl-hybrid-poc/release/web/public/
```

### 5. CI/CDの最適化

`.github/workflows/deploy-demo.yml`の主な変更：

```yaml
# 高速インストール
- uses: taiki-e/install-action@v2
  with:
    tool: dioxus-cli@0.6.3

# シンプルなビルドコマンド
- name: Build
  run: dx build --platform web --release

# 新しい出力パスからのコピー
- name: Copy files
  run: cp -r target/dx/leaflet-webgl-hybrid-poc/release/web/public/* .
```

## 移行手順

既存のTrunkプロジェクトをDioxus CLIに移行する場合：

1. **Dioxus CLIのインストール**
   ```bash
   mise use cargo:dioxus-cli@0.6.3
   ```

2. **Dioxus.tomlの作成**
   - プロジェクト情報の設定
   - base_pathの設定（GitHub Pagesの場合）
   - 外部リソースの定義
   - ビルドフックの設定

3. **ビルドスクリプトの更新**
   - Trunk関連のスクリプトを削除
   - package.jsonのスクリプトを更新

4. **CI/CDの更新**
   - GitHub Actionsワークフローの更新
   - ビルドコマンドとパスの変更

5. **開発環境の確認**
   ```bash
   dx serve
   # http://localhost:8080 でアクセス
   ```

## パフォーマンス改善

### ビルド時間の比較
- **Trunk**: 約5分（CI/CD）
- **Dioxus CLI**: 約2分（CI/CD）
- **改善率**: 60%短縮

### WASMサイズ
- 最適化は自動的に適用されます
- 手動のwasm-opt実行は不要になりました

## トラブルシューティング

### よくある問題

1. **`dx: command not found`**
   ```bash
   mise use cargo:dioxus-cli@0.6.3
   ```

2. **ビルドエラー**
   ```bash
   dx clean
   dx build --platform web
   ```

3. **Tailwind CSSが反映されない**
   ```bash
   npm install
   npm run build-css
   ```

## まとめ

TrunkからDioxus CLIへの移行により、以下の利点が得られました：

- ✅ GitHub Pagesデプロイの自動化
- ✅ ビルド時間の大幅短縮（60%改善）
- ✅ 外部リソース管理の簡素化
- ✅ 設定の一元化
- ✅ ビルドプロセスの簡素化

この移行により、開発体験が大幅に向上し、より効率的な開発が可能になりました。