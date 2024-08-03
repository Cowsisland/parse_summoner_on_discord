# ParseSummonerOnDiscord

## 2024/08/03
SummonerV4の実装をひとまず通せるラインまで完了。エラー処理とunwrapについては課題。serde_json::Valueで一度引き受けて型変換しているが、enumのuntaggedで実装しなおすことを検討する。

## やること
1. ~~とりま一つのAPIを実装する。~~
2. enumのuntaggedを用いた実装に検討し直す。
3. それぞれのAPIを実装する。
4. respからロジック部分をまとめる
5. Discordとの連携
6. エラーコードの記述（Statusが200以外の時はserde_json::Valueで取得してエラー記述する？）
