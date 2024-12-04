Set Shell zsh
# You'll likely want to add a Margin + MarginFill if you use BorderRadius.
Set Margin 20
Set Width 900
Set LoopOffset 95%
Set Height 650
Set MarginFill "#674EFF"
Set BorderRadius 10
Sleep 500ms
Type "cargo run --release --quiet -- memory_dump.txt"
Enter
Sleep 15s
Type "exit"
