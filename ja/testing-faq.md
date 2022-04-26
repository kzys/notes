# ソフトウェアの自動テストに関する FAQ

## 私のソフトウェアにはビジネスロジックと呼べるほどのものがありません。どこからテストを書けばいいでしょうか?

printf デバッグや、デバッガでステップ実行して動作を確認しているところがあれば、そこから書きはじめるのがおすすめです。目視で確認していたこと、例えばある時点での値が予測値と一致していたか、ゼロじゃないか、などがあれば、それをテストとして記述していけばいいでしょう。

すでに何度かリリースをして、バグを出してしまっているソフトウェアなら、バグを再現するテストを書くのもおすすめです。簡単すぎるコード (getter や setter など) にテストを書いても、労力の割に利はありません。バグが出てしまったところは、少なくとも一度は既存のチェック機構をすり抜けてしまっているので、十分な複雑さか、既存のチェック機構の抜け漏れか、その両方があるはずです。再発防止にテストを書くのは良い案といえるでしょう。

## 単体テスト (ユニットテスト) と結合テスト (インテグレーションテスト) のちがいは何ですか?

Rails のようなフレームワークが提供する区分や、チームや会社のなかでの合意はありますが、業界全体を通して「なにが単体テストで、なにが結合テストか」という問題についての合意はありません。これは日本に限ったことではなく、例えば Google Testing Blog の [Test Sizes](https://testing.googleblog.com/2010/12/test-sizes.html）(2010) にも

> What do you call a test that tests your application through its UI? An end-to-end test? A functional test? A system test? A selenium test? I’ve heard all them, and more. I reckon you have too. Tests running against less of the stack? The same equally frustrating inconsistency. Just what, exactly, is an integration test? A unit test? How do we name these things?

という一文があります。

ただ、一般に、一度にテストするコードが少ないものを単体テスト、多いものを結合テストと呼ぶのが一般的です。オブジェクト指向プログラミング言語を使っている場合「一つのクラスをテストするものが単体テストである」という人もいるかもしれませんが、そういう場合も、String や HashMap は一緒にテストしている (インターフェース経由でモックしたりしない) わけで、これを「一つのクラス」と数えるのは、やや恣意的だと思います。

## モックはどのくらい書くべきでしょうか?

テストに本番とは別の実装をつかう技法は、ひろく「モック」と呼ばれています。"xUnit Test Patterns" の著者 Gerard Meszaros は、これを"Test Doubles" と呼び、[以下のように5つに分離しています](https://martinfowler.com/articles/mocksArentStubs.html#TheDifferenceBetweenMocksAndStubs)。

> - Dummy objects are passed around but never actually used. Usually they are just used to fill parameter lists.
> - Fake objects actually have working implementations, but usually take some shortcut which makes them not suitable for production (an in memory database is a good example).
> - Stubs provide canned answers to calls made during the test, usually not responding at all to anything outside what's programmed in for the test.
> - Spies are stubs that also record some information based on how they were called. One form of this might be an email service that records how many messages it was sent.
> - Mocks are what we are talking about here: objects pre-programmed with expectations which form a specification of the calls they are expected to receive.

ここでいうスタブ、スパイ、そして狭義のモックは、実装と密に結合してしまいがちなので、できるだけ少なくするべきです。

フェイクオブジェクトを定義するか、あるいは実際の実装をパラメータを変えて使う、例えば `/` を削除する代わりに `/tmp/fakeroot` を削除できるようにして、ファイルの削除は実際に行ってしまうのがおすすめです。
