This is an example of a concurrent bruteforce algorithm written in rust with [Tokio](https://docs.rs/tokio/latest/tokio/).

This program will try to find working password based on the provided dictionary and an alphabet of similar characters.

Execution:
`cargo run -- --wordlist <path/to/wordlist> --alphabet <path/to/alphabet> --url <url>`

Example alphabet file context:

```txt
1iIlL$
0oOuU
```

Example wordlist file context:

```txt
qwerty
password
admin
welcome
```
