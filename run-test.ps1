
& cargo build
$PATH = Join-Path -Path $pwd -ChildPath target\debug\kongoc.exe

$BytecodePath = "a"

& $PATH --path $BytecodePath
