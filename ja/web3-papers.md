# web3 読んだ論文

## Bitcoin: A Peer-to-Peer Electronic Cash System

https://bitcoin.org/bitcoin.pdf

すべてのはじまりであるところの、Satoshi Nakamoto 論文。

> We define an electronic coin as a chain of digital signatures.

とか、突然で面白い。私は、発行量の制御なしかつ、GPU でどんどん採掘しているものは「通貨」というより鉱物の類ではと思っているけれど、

> Once a predetermined number of coins have entered circulation, the incentive can transition entirely to transaction fees and be completely inflation free.

著者としては、発行上限に達する前のことはあんまり心配していない、のか?

プライバシーについては「トランザクションの記録が公開されていても、それとアイデンティティは結びつけなくていいよね」とされているけれど、うーん、これは Ethereum の現状をみると、ばりばり結びつけられているように思う。見せる消費と隠す消費を分けるのが正しい姿なのかなあ。しかし、ある ID に色々トランザクションが結びつけられていたら、それだけで結構特定できてしまうような。

## Hashcash - A Denial of Service Counter-Measure

http://www.hashcash.org/papers/hashcash.pdf

Bitcoin 論文から参照されていたので読んだ。

> Hashcash was originally proposed as a mechanism to throttle systematic abuse of un-metered internet resources such as email, and anonymous remailers in May 1997. 

スパムとか送信コストの低さゆえに起こるので、無意味な計算を人為的に強いることで、送信コストを高くすればいいのでは、というはなし。

## Ethereum: A Next-Generation Smart Contract and Decentralized Application Platform.

https://ethereum.org/669c9e2e2027310b6b3cdce6e1c52962/Ethereum_Whitepaper_-_Buterin_2014.pdf

ホワイトペーパーという性格からか、上記のふたつよりだいぶ親切で長い。

NFT として売られている画像が中央サーバーにある2022年に、

> Ethereum contracts can allow for the development of a decentralized
file storage ecosystem, where individual users can earn small quantities of money by renting out their own hard drives and unused space can be used to further drive down the costs of file storage.

なんていっているのを読むのはちょっと笑える。

一方で、"Financial derivatives and Stable-Value Currencies" とか、"Decentralized Autonomous Organizations" までこれに盛り込まれているのは驚いた。もっと後から開発されたものだと思っていた。
