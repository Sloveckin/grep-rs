# grep-rs
A program similar to the standard console utility grep from UNIX written in the Rust programming language

## Dependencies
```toml
[dependencies]
clap = { version = "4.5.41", features = ["derive"] }
colored = "2"
```
## Arguments
```bash
Usage: grep-rs [OPTIONS] <SUBSTRING> <FILE>

Arguments:
  <SUBSTRING>  String that need to find
  <FILE>       File name

Options:
  -m, --mode <MODE>                Search from left side or right [default: all] [possible values: left, right, all, reverse, whole]
  -a, --algo <ALGO>                [default: kmp] [possible values: kmp, boyer-moore]
  -i, --ignore-case                Ignore case
  -n, --number                     Show number of line
  -c, --color <COLOR>              Color of selected part. Default = Red [default: red] [possible values: green, red, blue]
  -w, --window-size <WINDOW_SIZE>  Size of selected window in string. Default value = 10 [default: 10]
  -h, --help            
```
 
 ### Example 1
**Arguments:**``"aba" ./tmp/file1.txt --mode Left``
**File**
```bash
abacaba
notabacab
aba hehe
abacaba
```
**Output**

![](/images/LeftExample.png "First")

### Example 2
**Arguments:**``"aba" ./tmp/file1.txt --mode Right``
**File**
```bash
abacaba
notabacab
aba hehe
abacaba
```
**Output**

![](/images/RightExample.png "Second")

### Example 3
**Arguments:**``"aba" ./tmp/file1.txt --mode All``
**File**
```bash
abacaba
notabacab
aba hehe
abacaba
```
**Output**

![](/images/AllExample.png "Third")

### Example 4
**Arguments:**``"aba" ./tmp/file1.txt --mode Whole``
**File**
```bash
abacaba
notabacab
aba hehe
abacaba
```
**Output**

![](/images/WholeExample.png "Fourth")
