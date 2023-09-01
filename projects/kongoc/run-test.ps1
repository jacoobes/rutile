
& cargo build
$PATH = Join-Path -Path $pwd -ChildPath target\debug\kongoc.exe

$BytecodePath = "../lang/x.lang"

& $PATH --path $BytecodePath
