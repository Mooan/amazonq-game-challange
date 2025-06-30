# Terra Lock - Web版

蒼穹紅蓮隊風ロックオンレーザーゲーム「Terra Lock」のWeb版です。

## プレイ方法

このディレクトリのファイルをGitHub Pagesで配信することで、ブラウザでゲームをプレイできます。

### 操作方法
- **マウス移動**: 自機の移動
- **左クリック短押し**: 通常レーザー発射
- **左ボタン長押し**: ワイヤーフレーム展開とロックオン
- **左ボタンリリース**: ロックオン対象への一斉ホーミングレーザー発射

### 技術仕様
- **エンジン**: Rust + macroquad
- **WebAssembly**: wasm32-unknown-unknown
- **ファイルサイズ**: 482KB
- **対応ブラウザ**: Chrome, Firefox, Safari, Edge

## ファイル構成
- `index.html`: ゲーム実行用HTMLファイル
- `terra_lock.wasm`: WebAssemblyバイナリ
- `.nojekyll`: Jekyll処理を無効化
- `.htaccess`: WebAssembly用MIME設定
