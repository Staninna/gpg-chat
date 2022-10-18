# GPG-Chat

A chat application that uses GPG to encrypt messages. It is a work in progress.

I had some difficulties with the tokio-rusqlite library on windows with the linker here below is the fix I used.

It can also been seen on those 2 sites [github gist](https://gist.github.com/zeljic/d8b542788b225b1bcb5fce169ee28c55) and [github issue](https://github.com/dlang-community/d2sqlite3/issues/49#issuecomment-694725249) I don't know if this is also a problem on Linux or macos.

Note that I included `sqlite3.lib` and `sqlite3.dll` in this repo so you don't have to compile it yourself. It may or may not work on your machine. If it doesn't work compile it yourself and replace the one in the repo.

How to build sqlite3.lib file on Windows 10

1. Download the code source from <https://www.sqlite.org/download.html>

2. Also download the binary from <https://www.sqlite.org/download.html>

3. Extract both archives to the same directory

4. Open **Developer Command Prompt for VS 2017** by typing _Developer Command_ in Windows Search

5. Go to directory where you've extracted **source code** and **binary** files (via opened cmd)

6. Run `lib /DEF:sqlite3.def /OUT:sqlite3.lib /MACHINE:x64`

7. Copy the compiled `sqlite3.lib` and the extracted `sqlite3.dll` files to the root of this project
