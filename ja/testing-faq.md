# ソフトウェアの自動テストに関する FAQ

## 私のソフトウェアには「ビジネスロジック」と呼べるほどのものがありません。どこにテストを書けばいいでしょうか?

printf デバッグや、デバッガでステップ実行して動作を確認しているところから書きはじめるのがおすすめです。目視で確認できること、例えばある時点での値が予測値と一致しているかどうかなどがあれば、それをテストとして記述していけばいいでしょう。

もう一つの手は、バグが出たところからテストを書いていくことです。簡単すぎるコード (getter や setter など) にテストを書いても、あまり利はありません。バグが出てしまったところは、少なくとも一度はリリース前の確認に失敗しているので、再発防止にテストを書くのは良い案といえるでしょう。

## 単体テスト (ユニットテスト) と結合テスト (インテグレーションテスト) のちがいは何ですか?

Rails のようなフレームワークが提供する区分や、チームや会社のなかでの合意はありますが、業界全体を通して「なにが単体テストで、なにが結合テストか」という問題についての合意はありません。これは日本に限ったことではなく、例えば Google Testing Blog の [Test Sizes](https://testing.googleblog.com/2010/12/test-sizes.html）(2010) にも

> What do you call a test that tests your application through its UI? An end-to-end test? A functional test? A system test? A selenium test? I’ve heard all them, and more. I reckon you have too. Tests running against less of the stack? The same equally frustrating inconsistency. Just what, exactly, is an integration test? A unit test? How do we name these things?

という一文があります。

ただ、一般に、一度にテストするコードが少ないものを単体テスト、多いものを結合テストと呼ぶのが一般的です。オブジェクト指向プログラミング言語を使っている場合「一つのクラスをテストするものが単体テストである」という人もいるかもしれませんが、そういう場合も、String や HashMap は一緒にテストしている (インターフェース経由でモックしたりしない) わけで、これを「一つのクラス」と数えるのは、やや恣意的だと思います。

## モックはどのくらい書くべきでしょうか?

ひろく「モック」と呼ばれる、テストに本番とは別の実装をつかう技法ですが、これを Gerard Meszaros (TODO: who?) は "Test Doubles" と呼び、以下のように5つに分離しています。

> - Dummy objects are passed around but never actually used. Usually they are just used to fill parameter lists.
> - Fake objects actually have working implementations, but usually take some shortcut which makes them not suitable for production (an in memory database is a good example).
> - Stubs provide canned answers to calls made during the test, usually not responding at all to anything outside what's programmed in for the test.
> - Spies are stubs that also record some information based on how they were called. One form of this might be an email service that records how many messages it was sent.
> - Mocks are what we are talking about here: objects pre-programmed with expectations which form a specification of the calls they are expected to receive.

ここでいう狭義のモックは、メンテナンスが難しいので、できるだけ少なくするのをおすすめします。フェイクオブジェクトを定義するか、あるいは実際の実装をパラメータを変えて使う、例えば `/` を削除する代わりに `/tmp/fakeroot` を削除できるようにして、実際にファイル操作をおこなってしまうのがおすすめです。
