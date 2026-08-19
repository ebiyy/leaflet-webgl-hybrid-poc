# ADR-004: wee_allocの廃止とデフォルトアロケータへの回帰

**日付**: 2026-08-19  
**ステータス**: 承認済み  
**決定者**: Leaflet WebGL Hybrid POC開発チーム  

## コンテキスト

[ADR-002](002-wasm-optimization-strategy.md) では、WASMバイナリのサイズ最適化のため
`wee_alloc` をグローバルアロケータとしてデフォルト有効化していた。

その後、`wee_alloc` に対してセキュリティアドバイザリ
[GHSA-rc23-xxgq-x27g](https://github.com/advisories/GHSA-rc23-xxgq-x27g)
（[RUSTSEC-2022-0054](https://rustsec.org/advisories/RUSTSEC-2022-0054.html)、severity: critical）
が発行された。内容は以下の通り。

- メンテナ自身がメンテナンス終了を表明
- メモリリークを含むissueが未解決のまま
- 最終リリースから長期間が経過
- アドバイザリの推奨: 「wasm32ターゲットではRust標準のデフォルトアロケータへ切り替えるのが最善」

修正版（`first_patched_version`）は存在しない。バージョン更新では解消できず、
依存を外す以外に解決手段がない。

## 検討した選択肢

### 1. wee_allocのバージョン更新
- **評価**: ❌ 不可。脆弱バージョン範囲は `>= 0`、修正版なし

### 2. 代替の軽量アロケータ（talc / lol_alloc 等）へ置換
- **評価**: ❌ 不採用
- 削減できるサイズは数KB規模にとどまる
- サイズバジェット（640KB）に対して現状は十分な余裕がある
- 数KBのために別の小規模クレートへ依存を移すのは、同種の
  メンテナンスリスクを再度抱えることになり割に合わない

### 3. デフォルトアロケータ（dlmalloc）へ回帰
- **評価**: ✅ 採用
- アドバイザリの推奨そのもの
- 依存とfeatureフラグを削除するだけで、追加の複雑性がない
- dlmallocは成熟した実装で、割り当て性能・メモリ効率で優位

## 決定

`wee_alloc` の依存、`wee_alloc` featureフラグ、および
`src/main.rs` の `#[global_allocator]` 定義を削除する。
Rust標準のデフォルトアロケータ（wasm32では dlmalloc）を使用する。

## 結果

サイズ影響（`cargo build --release --target wasm32-unknown-unknown`、
wasm-bindgen/wasm-opt適用前の生バイナリ）:

| 構成 | サイズ | 差分 |
|------|--------|------|
| wee_alloc あり | 1,378,849 bytes | baseline |
| wee_alloc なし | 1,385,859 bytes | +7,010 bytes (+0.51%) |

最終的な配信サイズ（wasm-opt / wasm-snip / wasm-tools strip 適用後）は
サイズバジェット 640KB に対して十分な余裕があり、この増加は許容範囲。

副次的な効果として、割り当て性能とメモリ断片化耐性は改善する見込み。
これは10,000オブジェクト描画のような割り当て頻度の高い経路にとってはむしろ有利。

## 参考資料

- [GHSA-rc23-xxgq-x27g](https://github.com/advisories/GHSA-rc23-xxgq-x27g)
- [RUSTSEC-2022-0054](https://rustsec.org/advisories/RUSTSEC-2022-0054.html)
- [rustwasm/wee_alloc#107](https://github.com/rustwasm/wee_alloc/issues/107)
- [ADR-002: WASM最適化戦略](002-wasm-optimization-strategy.md)
