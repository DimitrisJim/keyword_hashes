# tinyphf

An extremely stripped down version of rust-phf, just contains the core Map and the codegen Map to generate
it (And done this way mostly so I could understand how the codegen glue works, which is arguably very
straightforward).

## What I basically want to do during building

1. Start with an initiale table size.
2. Attempt to build it using a starting seed. The number of attempts is limited to 10000.
   - If it during one of the attempts, bump the seed.
   - If it fails to build after 10000 attempts, double the table size and try again

