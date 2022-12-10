window.SIDEBAR_ITEMS = {"enum":[["TimerError","An error that can occur when `JitterRng::test_timer` fails."]],"mod":[["adapter","Wrappers / adapters forming RNGs"],["mock","Mock random number generator"]],"struct":[["EntropyRng","An interface returning random data from external source(s), provided specifically for securely seeding algorithmic generators (PRNGs)."],["JitterRng","A true random number generator based on jitter in the CPU execution time, and jitter in memory access time."],["OsRng","A random number generator that retrieves randomness straight from the operating system."],["SmallRng","An RNG recommended when small state, cheap initialization and good performance are required. The PRNG algorithm in `SmallRng` is chosen to be efficient on the current platform, without consideration for cryptography or security. The size of its state is much smaller than for `StdRng`."],["StdRng","The standard RNG. The PRNG algorithm in `StdRng` is chosen to be efficient on the current platform, to be statistically strong and unpredictable (meaning a cryptographically secure PRNG)."],["ThreadRng","The type returned by [`thread_rng`], essentially just a reference to the PRNG in thread-local memory."]]};