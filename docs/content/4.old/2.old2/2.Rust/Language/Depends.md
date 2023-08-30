# Dependencies

To override the dependency for a local project to point to a directory for a local built version of a project
```
[build-dependencies]
embuild = { path = "<path to local patched version>" }
```

or
```
[dependencies]
embuild = { path = "<path to local patched version>" }
```

To override the crate for all sub dependencies we can use
```
[patch.crates-io]
embuild = { path = "<path to local patched version>" }
```
