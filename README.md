```fish
cargo build | xargs -L 1 -r $EDITOR 2>/dev/null
exit $pipestatus[1]
```
