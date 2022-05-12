# Go Programming Language

* [Effective Go](https://go.dev/doc/effective_go)
* [Functional options for friendly APIs](https://dave.cheney.net/2014/10/17/functional-options-for-friendly-apis)
* [Practical Go: Real world advice for writing maintainable Go programs](https://dave.cheney.net/practical-go/presentations/gophercon-singapore-2019.html)
* [Package names](https://blog.golang.org/package-names)
* [How to write benchmarks in Go](https://dave.cheney.net/2013/06/30/how-to-write-benchmarks-in-go)

## Concurrency

* [Notes on structured concurrency, or: Go statement considered harmful](https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/)

## Pitfalls

### Go may be slow

[Why did you create a new language?](https://go.dev/doc/faq#creating_a_new_language);

> Finally, working with Go is intended to be fast: it should take at most a few seconds to build a large executable on a single computer. 

[Why does Go perform badly on benchmark X?](https://go.dev/doc/faq#Why_does_Go_perform_badly_on_benchmark_x);

> One of Go's design goals is to approach the performance of C for comparable programs, yet on some benchmarks it does quite poorly, including several in golang.org/x/exp/shootout. 
>
> ...
> Still, there is room for improvement. The compilers are good but could be better, many libraries need major performance work, and the garbage collector isn't fast enough yet. (Even if it were, taking care not to generate unnecessary garbage can have a huge effect.) 

[The Go compiler needs to be smarter](https://lemire.me/blog/2020/06/04/the-go-compiler-needs-to-be-smarter/) (2020) pointed out conservative inlining and the lack of runtime constant variables.

- LLVM-backed compilers (rustc, clang), GCC and JVM are optimized for decades.
- In addition to that, Go is okay to sacrifice its runtime perfomance to shorten the build time, which is different from other compilers.
- Go's FFI is also slow.

### Typed Nil

* https://go.dev/doc/faq#nil_error
* https://dave.cheney.net/2017/08/09/typed-nils-in-go-2
