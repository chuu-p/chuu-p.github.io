+++
title = "Polymorphic executables in fish shell"
date = 2025-04-27
category = "fish"

[extra]
comments = false
postid = "0"
miku_img = "miku_wai"
miku_q = "TODO QUOTE"
+++

I was inspired by [this video](https://www.youtube.com/watch?v=dv6NP7qjMS0) to start using polymorphic executables. I use fish-shell as my default shell, so this blog post explains what are polymorphic executables are and how to use polymorphic executables in fish shell.

## {{title}}

### What are polymorphic executables?

Polymorphic executables are one "main" shell script, that has a different behaviour depending on how it was called. The following script prints "hello" when called as ./hello (with a soft link) and prints "world" when called as ./world and prints "main" in any other case.

~~~sh
#!/usr/bin/env bash

# Determine how the script was called by checking the symbolic link.
# $0 contains the name of the script as it was called.

case "$(basename "$0")" in
  "hello")
    echo "hello"
    ;;
  "world")
    echo "world"
    ;;
  *)
    echo "main"
    ;;
esac
~~~

~~~sh
/tmp ❱ ./main
main
/tmp ❱ ./hello
hello
/tmp ❱ ./world
world
/tmp ❱ ll main hello world
lrwxrwxrwx 1 chuu users   4 27. Apr 15:50 hello -> main
-rwxr-xr-x 1 chuu users 333 27. Apr 15:51 main
lrwxrwxrwx 1 chuu users   4 27. Apr 15:50 world -> main
~~~

### Why use polymorphic executables?

As an example, git cleverly employs polymorphic executables, where a single git executable can morph its behavior based on the command-line arguments it receives. When you run git merge, git push, or any other git command, the core git program identifies the specific subcommand and dynamically executes the corresponding functionality. This design allows Git to have a modular structure while presenting a unified interface to the user.

Busybox exemplifies the power of a single, small executable providing a multitude of Unix utilities. Its polymorphic nature allows it to act as ls, cp, mv, and many other commands, significantly reducing the system's footprint, which is crucial in embedded systems. This project uses polymorphic executables, ex:

~~~sh
cat -> /bin/busybox
false -> /bin/busybox
kill -> /bin/busybox
ln -> /bin/busybox
ls -> /bin/busybox
lsattr -> /bin/busybox
zcat -> /bin/busybox
~~~

### How can I use polymorphic executables in fish shell?

My favorite shell is fish, I want to use this functionality, and did not find any resources online of people doing it, so I implemented it myself. This is how:

`poly.fish`

~~~fish
#!/usr/bin/env fish

set invoked_name (basename (status current-filename))
echo "invoked name: $invoked_name"

switch $invoked_name
    case hello
        echo hello

    case world
        echo world

    case "*"
        echo "Unknown invocation: $invoked_name"
end
~~~

In action:

~~~sh
/tmp ❱ hx poly.fish
/tmp ❱ ln -s poly.fish hello
/tmp ❱ ln -s poly.fish world
/tmp ❱ ./hello
invoked name: hello
hello
/tmp ❱ ./world
invoked name: world
world
/tmp ❱ ./poly.fish
invoked name: poly.fish
Unknown invocation: poly.fish
~~~

Thank you for reading, and have fun with polymorphic executables in fish shell!
