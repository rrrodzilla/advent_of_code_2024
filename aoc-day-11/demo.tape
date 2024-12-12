Set Shell zsh
Set Margin 20
Set Width 1300
Set LoopOffset 95%
Set Height 650
Set MarginFill "#674EFF"
Set BorderRadius 10
Sleep 500ms
Type "time cargo run --quiet --release -- --redact < stones.txt"
Enter
Sleep 5s
Type "tokei . -t=Rust -C"
Enter
Sleep 5s
Type "exit"
