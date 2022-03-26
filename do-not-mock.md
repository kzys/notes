# Don't mock dependencies

In software testing, I believe mocking is a necessary evil. There are places where it works, but overall it is better to use real dependencies.

This is sometimes called [classical TDD style](https://martinfowler.com/articles/mocksArentStubs.html
) or "Detroit" style.

> The classical TDD style is to use real objects if possible and a double if it's awkward to use the real thing.
