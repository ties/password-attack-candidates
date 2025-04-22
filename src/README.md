# password-distance

A utility to generate derivatives from an input password according to specified parameters. This was used to help me brute-force a password derived from a password I know (a password with a typo).

Two derivations are implemented:
  * Edits (insert/replace/delete of any position. Any printable character is considered).
  * Transpositions (e.g. `apssword` is `password` with a transposition of distance 1 at position 0+1).

## Usage in brute forcing a passphrase:

```
# example of recovering a SSH key
$ python ~/bin/john/run/ssh2john.py ~/.ssh/id_ed25519_with_typo > ~/Desktop/id_ed25519_with_typo_john.txt   
$ ./target/release/password_distance -p password --max-distance=0 --transposition-distance=2 --max-transpositions=1 | ~/bin/john/run/john ~/Desktop/id_ed25519_with_typo_john.txt
```