# Good comments

Some are view/opinions. Some are amusing little facts about the implementations.

## seccomp

From [dhcpcd](https://github.com/NetworkConfiguration/dhcpcd/blob/dhcpcd-9.4.1/src/privsep-linux.c#L362-L365);

```c
	/* i386 needs this and demonstrates why SECCOMP
	 * is poor compared to OpenBSD pledge(2) and FreeBSD capsicum(4)
	 * as this is soooo tied to the kernel API which changes per arch
	 * and likely libc as well. */
```

The comment was [once linked from Wikipedia](https://en.wikipedia.org/w/index.php?title=Seccomp&diff=993963350&oldid=993501330) but later removed.


 ## Windows

 From [Go](https://github.com/golang/go/blob/go1.18.1/src/cmd/go/internal/test/test.go#L1058-L1063);

 ```go
	if cfg.Goos == "windows" {
		// There are many reserved words on Windows that,
		// if used in the name of an executable, cause Windows
		// to try to ask for extra permissions.
		// The word list includes setup, install, update, and patch,
		// but it does not appear to be defined anywhere.
		// We have run into this trying to run the
		// go.codereview/patch tests.
		// For package names containing those words, use test.test.exe
		// instead of pkgname.test.exe.
		// Note that this file name is only used in the Go command's
		// temporary directory. If the -c or other flags are
		// given, the code below will still use pkgname.test.exe.
		// There are two user-visible effects of this change.
		// First, you can actually run 'go test' in directories that
		// have names that Windows thinks are installer-like,
		// without getting a dialog box asking for more permissions.
		// Second, in the Windows process listing during go test,
		// the test shows up as test.test.exe, not pkgname.test.exe.
		// That second one is a drawback, but it seems a small
		// price to pay for the test running at all.
		// If maintaining the list of bad words is too onerous,
		// we could just do this always on Windows.
```

## Perl

From [Perl](https://github.com/Perl/perl5/blob/v5.35.11/toke.c#L4401-L4421);

```c
/* S_intuit_more
 * Returns TRUE if there's more to the expression (e.g., a subscript),
 * FALSE otherwise.
 *
 * It deals with "$foo[3]" and /$foo[3]/ and /$foo[0123456789$]+/
 *
 * ->[ and ->{ return TRUE
 * ->$* ->$#* ->@* ->@[ ->@{ return TRUE if postderef_qq is enabled
 * { and [ outside a pattern are always subscripts, so return TRUE
 * if we're outside a pattern and it's not { or [, then return FALSE
 * if we're in a pattern and the first char is a {
 *   {4,5} (any digits around the comma) returns FALSE
 * if we're in a pattern and the first char is a [
 *   [] returns FALSE
 *   [SOMETHING] has a funky algorithm to decide whether it's a
 *      character class or not.  It has to deal with things like
 *      /$foo[-3]/ and /$foo[$bar]/ as well as /$foo[$\d]+/
 * anything else returns TRUE
 */

/* This is the one truly awful dwimmer necessary to conflate C and sed. */
```

## sockaddr

From [Varnish](https://github.com/varnishcache/varnish-cache/blob/varnish-7.1.0/lib/libvarnish/vsa.c);

```c
/*
 * Struct sockaddr{|_in|_in6|_storage} is absolutely the worst data
 * structure I have ever seen gold-plated in international standards.
 *
 * Network addresses have multiple different forms, many fewer today
 * than in last century, but imagine that in addition to IPv4 and IPv6
 * we had 40 other protocols.  Actually, you don't need to imagine that
 * just count the AF_* macros in /usr/include/sys/socket.h.
 *
 * So what do we pass the kernel API for an address to bind(2), connect(2) &
 * listen(2) etc. etc ?
 *
 * We could define a struct which is big enough to hold any and all
 * of these addresses.  That would make it a fixed size argument.
 * obviously the struct would have to be something like:
 *	struct bla {
 *		int family;
 *		char address[MAX_ADDR_LEN];
 *	}
 * and MAX_ADDR_LEN would have to be quite large, 128 byte or so.
 *
 * Back in last century that was TOTALLY unacceptable waste of space.
 *
 * The way which was chosen instead, was to make a "generic" address,
 * and have per protocol "specific" addresses, and pass the length
 * argument explicitly to the KPI functions.
 *
 * The generic address was called "struct sockaddr", and the specific
 * were called "struct sockaddr_${whatever}".  All of these must have
 * a "family" field as first element, so the kernel can figure out
 * which protocol it is.
 *
 * The generic struct sockaddr was made big enough for all protocols
 * supported in the kernel, so it would have different sizes depending
 * on your machine and kernel configuration.
 *
 * However, that allowed you to write protocol-agnostic programs, by
 * using "struct sockaddr" throughout, and relying on libray APIs for
 * things like name to address (and vice versa) resolution, and since
 * nobody were in the business of shipping random UNIX binaries around
 * the lack of binary portability didn't matter.
 *
 * Along the way the BSD people figured out that it was a bother
 * to carry the length argument separately, and added that to the
 * format of sockaddr, but other groups found this unclean, as
 * the length was already an explicit parameter.
 *
 * The net result of this is that your "portable" code, must take
 * care to handle the "sa_len" member on kernels which have it,
 * while still tracking the separate length argument for all other
 * kernels.
 *
 * Needless to say, there were no neat #define to tell you which
 * was which, so each programmer found a different heuristic to
 * decide, often not understanding it fully, which caused the kind
 * of portability issues which lead to the autocrap tools.
 *
 * Then all the other protocols died, we were left with IP and
 * life were good, the dot-com madness multiplied the IT-business
 * by a factor 1000, by making any high-school student who had
 * programmed PERL for 6 weeks a "senior web-programmer".
 *
 * Next IPv6 happened, in a rush even, (no seriously, I'm not kidding!),
 * and since IPv6 addresses were HUGE, like 16 bytes HUGE, the generic
 * struct sockaddr was not increased in size.
 *
 * At least "not yet", because it would break all the shitty code written
 * by the dot-com generation.
 *
 * Nobody used IPv6 anyway so that didn't matter that much.
 *
 * Then people actually started using IPv6 and its struct sockaddr_in6,
 * and realized that all the code which used "struct sockaddr" to allocate
 * space at compile time were broken.
 *
 * Some people took to using sockaddr_in6, since that was known to
 * be big enough for both IPv4 and IPv6, but "purist" found that
 * ugly and "prone to future trouble".
 *
 * So instead they came up with a "clean solution":  The added
 * "struct sockaddr_storage" which is defined to be "Large enough
 * to accommodate all supported protocol-specific address structures".
 *
 * Since we cannot possibly know what zany protocols will exist in
 * the future, and since some people think that we will add future
 * protocols, while retaining ABI compatibility, (totally overlooking
 * the fact that no code for name-resolution supports that) it is
 * usually defined so it can cope with 128 byte addresses.
 *
 * Does that ring a bell ?
 *
 * Only, not quite:  Remember that all APIs require you to track
 * the address and the length separately, so you only get the
 * size of the specific protocols sockaddr_${whatever} from API
 * functions, not a full sockaddr_storage, and besides the
 * prototype for the KPI is still "struct sockaddr *", so you
 * cannot gain C type-safety back by using sockaddr_storage
 * as the "generic network address" type.
 *
 * So we have come full circle, while causing maximum havoc along
 * the way and for the forseeable future.
 *
 * Do I need to tell you that static code analysis tools have a
 * really hard time coping with this, and that they give a lot of
 * false negatives which confuse people ?
 *
 * I have decided to try to contain this crap in this single
 * source-file, with only minimum leakage into the rest of Varnish,
 * which will only know of pointers to "struct suckaddr", the naming
 * of which is my of the historical narrative above.
 *
 * And you don't need to take my word for this, you can see it all
 * in various #include files on your own system.   If you are on
 * a Solaris derivative, don't miss the beautiful horror hidden in the
 * variant definition of IPv6 addresses between kernel and userland.
 *
 */
 ```
