# web3 だめそう

いまのところ web3 はだめそうだと思っている。

個人や企業が web3 に向かうのは彼らの自由だけど、自由民主党までもが、[デジタル・ニッポン 2022](https://jimin.jp-east-2.storage.api.nifcloud.com/pdf/news/policy/203427_1.pdf)と題して、

> すなわち、巨大なプラットフォーマーが世界を席巻してきた Web2.0 の世界から、ブロックチェーン技術9に裏打ちされた NFT（非代替性トークン）等のイノベーションの到来によって、個と個がつながる分散化した Web3.0(ウェブ・スリー)の世界への移行が進んでいる。

とか

> Web3.0 や NFT を新しい資本主義の成長の柱に位置付け、Web3.0 担当大臣を置き、経済政策の推進、諸外国との連携の司令塔とすべき。省庁横断の相談窓口を置くべ
き

なんてうたっているのをみると、web3 をだめだと思っている、私もふくめた専門職の人々は、きちんとだめさを説明する責務を果たせていなかったと思う。

私が間違っていて [Andreessen Horowitz](https://a16zcrypto.com/) が正しくて、web3 がうまくいく可能性はある。あるけれど「移行が進んでいる」は2022年の現状認識として間違っているし「成長の柱に位置付ける」のは、リスクの高すぎる賭けで、有権者として同意できない。

(とはいえ、この文章は、いまのところきちんとだめさを説明しておらず、どちらかというとリンク集です)

## 暗号通貨

流量をコントロールできないエンティティを「通貨」とするのは、中央銀行が通貨の
流量をどうこうすることの否定に感じる。私の感覚では、同じように「採掘する」石油や鉱物に近いように思う。それらにも価格の安定のための機関はあるわけだけど...

## パブリックなデータベース

パブリックなデータベースがあるということ自体は面白い。パブリックなデータベースを運営する仕組みとして、政府や上場企業や NPO ではなく、ブロックチェーンが必要で、その利点がブロックチェーンの効率の悪さを上回る、という事例が思い浮かばない。

## 読んだもの/見たもの

### The Web3 Fraud

<https://www.usenix.org/publications/loginonline/web3-fraud>

- インターネットはすでに分散化システムである
- web3 がそこに追加するものはとくに無い
- Ethereum は普通の計算機 (Raspberry Pi, EC2) と比べても、普通のストレージ (S3, SSD) と比べても、性能価格比が悪すぎる
- 暗号通貨の価値をなんとか維持するための仕組みでしかない

### My first impressions of web3

<https://moxie.org/2022/01/07/web3-first-impressions.html>

- web1 -> web2 で中央集権化が起こったのは、人々はサーバーなんか維持したくないからであり、複数のアプリケーションの合意が必要なプロトコル (e.g. IRC) より、ひとつのアプリケーションしかないプラットフォーム (e.g. Slack) のほうが変えやすいからである。
- 実際にブロックチェーンに情報を保存する dApp を作ってみた
- 暗号通貨の世界では、モバイルアプリケーションや、Web アプリケーションのようなクライアントの存在が考慮されていなく、これらのクライアントは現在 Infura や Alchemy のような昔ながらのサーバーを通してブロックチェーンにアクセスしている。
- NFT がブロックチェーンに保存するのは、昔ながらのサーバーの URL である。URL がポイントするものは自由に変えられる (メッセージダイジェストすら保存されない) ので、実際に動的に変化する URL をさしている NFT を売ってみたら、OpenSea から削除されてしまった。
- OpenSea の API レベルで「削除」されてしまうと、ブロックチェーンにその API 経由でアクセスしているその他のアプリケーションからも削除されてしまう。ブロックチェーン上にデータがあることは実質無意味になってしまう。

### Line Goes Up – The Problem With NFTs

<https://www.youtube.com/watch?v=YQ_xWvX1n9g>

歴史についておもしろいけど、動画はみかえすのがきつい。

- 2008年の住宅ローンバブル
- (つづく)

### Why you can't just screenshot an NFT

<https://www.youtube.com/watch?v=451V-lBLfuo>

- 1973: Robert & Ethel Scull - まだ生きているアーティストの作品が高値で売れる
- アートの価格と「なにがアートか」は、それ以来ずっと、高く、広くなっている
- 人々は物体としてのアートを買っているわけではなく、それを買ったという証明を買っている
- ブロックチェーンはあまり重要ではなく、一定以上の人々が合意できる証明書のデータベースがブロックチェーン上にできただけ

### Exposing Jake Paul’s Scams

<https://www.youtube.com/watch?v=CAyE2NIhbDo>

取引記録がパブリックになっているのは (銀行の口座間のやりとりを洗うようなことを個人ができるのは) 面白い。

### I was wrong, we need crypto

<https://world.hey.com/dhh/i-was-wrong-we-need-crypto-587ccb03>

カナダのプロテストについて、政府が抗議者や支援者の銀行口座を差し押さえたことに関して、政府に対する不信と、それに依存しない暗号通貨への期待。

### 〈NFTアート〉への共同ステートメント

<https://nft-art-statement.github.io/>

> 〈NFTアート〉：作品に紐付いたNFTが発行されるという共通点だけを除き、簡潔には定義できない多様な活動です。このステートメントに署名する人々により、集合的に定義されます。

私が署名したあとに定義が変わってしまう可能性があるわけで、きびしいなと思うので書名していない。