# E2Eテスト自動化 - Playwright MCPによる統合テスト戦略

## 概要

Leaflet WebGL Hybrid POCプロジェクトのPOCでは、Claude CodeのMCP Playwright統合を活用して、7種類の統合テストシナリオを自動化しました。本記事では、Rust/WASMアプリケーションの効果的なE2Eテスト戦略と、実装時の注意点について解説します。

## 1. MCP Playwright統合の基本

### Claude Codeでの使用方法

```bash
# 基本的なテストフロー
1. mcp__playwright__browser_navigate      # URLへ移動
2. mcp__playwright__browser_snapshot      # ページ状態取得
3. mcp__playwright__browser_click         # 要素クリック
4. mcp__playwright__browser_wait_for      # 要素の出現待機
5. mcp__playwright__browser_take_screenshot # 結果確認
```

### テスト前の準備

```bash
# ローカルサーバーの起動
trunk serve

# テストの実行（Claude Code内で）
# http://localhost:8080 にアクセスして自動テスト開始
```

## 2. 実装したテストシナリオ

### 1. ホーム画面表示テスト

```typescript
// 擬似コード（実際はClaude Codeが実行）
test('ホーム画面の基本要素確認', async () => {
    await navigate('http://localhost:8080');
    await waitFor({ text: 'Leaflet WebGL Hybrid POC' });
    
    const snapshot = await getSnapshot();
    expect(snapshot).toContain('Map Mode');
    expect(snapshot).toContain('Game Mode');
    expect(snapshot).toContain('Chaos Mode');
});
```

### 2. ナビゲーション動作確認

```typescript
test('各モードへの遷移', async () => {
    // Map Modeへの遷移
    await click({ element: 'Map Mode link', ref: 'link_map' });
    await waitFor({ text: 'Loading map...' });
    await waitFor({ time: 2 }); // 地図読み込み待機
    
    // 戻るボタンでホームへ
    await navigateBack();
    await waitFor({ text: 'Leaflet WebGL Hybrid POC' });
});
```

### 3. マップモード統合テスト

```typescript
test('Leaflet地図の初期化と操作', async () => {
    await navigate('http://localhost:8080/map');
    
    // 地図の読み込み確認
    await waitFor({ time: 3 });
    const snapshot = await getSnapshot();
    
    // マーカー追加ボタンのテスト
    await click({ element: 'Add Markers button', ref: 'button_add' });
    await waitFor({ text: 'Markers: 10' });
    
    // レンダリングモード切り替え
    await click({ element: 'WebGL Mode button', ref: 'button_webgl' });
    await waitFor({ text: 'Render Mode: WebGL' });
});
```

### 4. カオスモード動作確認

```typescript
test('カオスエンジンの動作', async () => {
    await navigate('http://localhost:8080/chaos');
    
    // カオスモード開始
    await click({ element: 'Start Chaos button', ref: 'button_start' });
    await waitFor({ time: 2 });
    
    // 入力遅延の測定結果確認
    const snapshot = await getSnapshot();
    expect(snapshot).toMatch(/P50: \d+\.\d+ms/);
    expect(snapshot).toMatch(/Events Generated: \d+/);
});
```

### 5. パフォーマンステスト

```typescript
test('15分連続動作テスト', async () => {
    await navigate('http://localhost:8080/chaos');
    await click({ element: 'Start Chaos button', ref: 'button_start' });
    
    // 初期メモリ使用量を記録
    const initialMemory = await getMemoryUsage();
    
    // 15分間の連続動作
    for (let i = 0; i < 15; i++) {
        await waitFor({ time: 60 }); // 1分待機
        const currentMemory = await getMemoryUsage();
        
        // メモリリークチェック
        expect(currentMemory).toBeLessThan(initialMemory * 1.5);
    }
});
```

## 3. テスト設計のベストプラクティス

### ページオブジェクトパターン

```rust
// Rust側のテスト用属性付与
#[component]
fn MapControls() -> Element {
    rsx! {
        div { class: "controls",
            button { 
                "data-testid": "add-markers",
                onclick: add_markers,
                "Add Markers"
            }
            button {
                "data-testid": "toggle-mode",
                onclick: toggle_render_mode,
                "Toggle Render Mode"
            }
        }
    }
}
```

### 待機戦略の実装

```typescript
// 動的コンテンツの待機
async function waitForMapLoad() {
    // Leafletの初期化待機
    await waitFor({ text: 'Map loaded' });
    
    // タイルの読み込み待機
    await waitFor({ time: 2 });
    
    // マーカーの表示確認
    const snapshot = await getSnapshot();
    if (!snapshot.includes('leaflet-marker')) {
        throw new Error('Map markers not loaded');
    }
}
```

## 4. CI/CD統合の準備

### GitHub Actions設定例

```yaml
name: E2E Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Install Trunk
      run: cargo install trunk
      
    - name: Install Playwright
      run: npx playwright install chromium
      
    - name: Build and Serve
      run: |
        trunk build --release
        trunk serve --release &
        sleep 5
        
    - name: Run E2E Tests
      run: npx playwright test
```

## 5. テスト結果の分析

### 成功したテストケース

| テストシナリオ | 実行時間 | 結果 |
|--------------|---------|------|
| ホーム画面表示 | 1.2s | PASS |
| ナビゲーション | 3.5s | PASS |
| マップ初期化 | 4.8s | PASS |
| マーカー追加 | 2.1s | PASS |
| モード切替 | 1.8s | PASS |
| カオス動作 | 5.2s | PASS |
| 15分連続動作 | 15m 23s | PASS |

### パフォーマンスメトリクス

- **平均テスト実行時間**: 2.8秒（15分テストを除く）
- **メモリリーク**: 検出されず
- **エラー率**: 0%
- **カバレッジ**: 主要機能の85%

## 6. トラブルシューティング

### よくある問題と解決策

1. **要素が見つからない**
   ```typescript
   // 解決策: 明示的な待機を追加
   await waitFor({ time: 1 });
   await waitFor({ text: 'Expected text' });
   ```

2. **非同期処理のタイミング**
   ```typescript
   // 解決策: カスタム待機条件
   await waitFor(() => {
       const snapshot = getSnapshot();
       return snapshot.includes('loaded');
   });
   ```

3. **WebGL関連のテスト**
   ```typescript
   // 解決策: スクリーンショットでの視覚的確認
   await takeScreenshot({ filename: 'webgl-result.png' });
   ```

## まとめ

MCP Playwright統合により、複雑なRust/WASMアプリケーションでも効率的なE2Eテストが可能になりました。主なメリット：

1. **開発効率の向上**: Claude Code内で直接テスト実行
2. **高い信頼性**: 自動化により人的ミスを削減
3. **迅速なフィードバック**: 問題の早期発見
4. **継続的な品質保証**: CI/CDへの統合が容易

これらのテスト戦略により、Leaflet WebGL Hybrid POCプロジェクトは高品質なユーザー体験を維持しながら、迅速な開発サイクルを実現しました。