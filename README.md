Example test case for [cargo-mutant](https://github.com/sourcefrog/cargo-mutants) [issue #327](https://github.com/sourcefrog/cargo-mutants/issues/327)

## Desired behavior

```
$ cargo mutants
Found 4 mutants to test
ok       Unmutated baseline in 0.2s build + 0.1s test
 INFO Auto-set test timeout to 20s
MISSED   src/toplevel/nested.rs:2:10: replace + with * in add in 0.2s build + 0.1s test
4 mutants tested in 1s: 1 missed, 3 caught
```

## Failure behavior

```
$ cargo mutants
 WARN referent of mod not found parent_path="src/lib.rs" mod_name=nested tried_paths=["/home/ned/git/cargo-mutants-testcase/src/nested.rs", "/home/ned/git/cargo-mutants-testcase/src/nested/mod.rs"]
Found 0 mutants to test
 WARN No mutants found under the active filters

$ cargo mutants --version
cargo-mutants 24.3.0
```


## Details of the test case
Details of the change from "desired" to "failure" that triggers the issue

- `src/lib.rs`
    ```patch
    -pub mod toplevel;
    +pub mod toplevel {
    +    pub mod nested;
    +}

     #[cfg(test)]
     mod tests {
        use super::toplevel::nested::*;

        #[test]
        fn it_works() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }
    ```
- `src/toplevel.rs` (deleted)
    ```patch
    -pub mod nested;
    ```
- `tree` summary
    ```patch
     $ tree .
     .
     ├── Cargo.lock
     ├── Cargo.toml
     └── src
         ├── lib.rs
    -    ├── toplevel.rs
         └── toplevel
             └── nested.rs
    ```
