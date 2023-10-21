# Example /etc/shadow password cracker (Demonstration purposes only!)
I followed the [Michael's video](https://www.youtube.com/watch?v=7JVruUQqK7o&t=38s&ab_channel=MichaelMullin) to learn about hashing basics, the "/etc/shadow" file format, hashing passwords & cracking them using Rust.


## How it works?
The idea is that we leverages parallelism (provided by the Rayon crate) and hashing functions (sha512_crypt, we do 5k iterations per potential password) to "crack" passwords for a particular set of records inside the "/etc/shadow" file if possible. We also use the atomic vatiables (bool in this case) to stop threads from hashing passwords from the "wordlist" in case a password for a particular record was found (even though the iteration will be ongoing, but do no hashing with the words).
If a particular record has the "name:$hash_algo_type$salt$data" format, then we can work with it.


## Run code
• Run the "crack_shadow" with the following command. The reason we run this command with the `--release` flag is that in debug mode it would take a long time to run the hashing algorithm.
```
cargo run --release --bin cargo_shadow
```

• The out should look like this:
```
Found mm :: password123
Found root :: .
```

## Note
Don't do illegal stuff, please :3