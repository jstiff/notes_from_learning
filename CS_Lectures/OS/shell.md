### Shell [gnu docs 'what is a shell'](https://www.gnu.org/software/bash/manual/html_node/What-is-a-shell_003f.html#What-is-a-shell_003f)

- At its base, a shell is simply a macro processor that executes commands. The term macro processor means functionality where text and symbols are expanded to create larger expressions.
- A Unix shell is both a command interpreter and a programming language. (It's like a programming language that can interpret user input) As a command interpreter, the shell provides the user interface to the rich set of GNU utilities. The programming language features allow these utilities to be combined.
  - shell languages use a 'command based' syntax and semantics.
    - Process command: 'ls' is a process command. The shell looks for this command, by checking its PATH.
      - PATH is an enviroment variable listing the routes to all the bin directories that store all of the commands a shell will have access to. You can provide **direct paths** like /usr/bin/ls. If you simply type 'ls' then the shell will search the entire path untill it finds the **first** match.
- Like any high-level language, the shell provides variables, flow control constructs, quoting, and functions.
- The shell is your interface to the operating system. It acts as a command interpreter.
- It is named a shell because it is the outermost layer around the operating system.
- A terminal is an interface that runs a 'shell' program...or sends it the user input.
  - the shell is a way to access the Kernal.
- ex) command “ls” is a file, that when it is executed, the system creates a process corresponding to an instance of a running program and that has an ID or PID, which is not more than a unique number, used by several functions or system calls (syscalls) to manipulate these processes.
  - the "ls" is a child process that is spawned by the parent process(Shell itself).
- Every command starts up a **process** that is provided a default stdin, stdout,stderr to write or read too. YOu can direct or redirect data to other locations... > , < , | ...

### Terminal vs Shell

- The terminal emulator (often just called terminal) is “just the window”, yes. It runs a text based program, which by default is your login shell (which is bash in Ubuntu). When you type characters in the window, the terminal draws these characters in the window in addition to sending it to the shell’s (or other program’s) stdin. The characters the shell outputs to stdout and stderr get sent to the terminal, which in turn draws these characters in the window.

### Fun Fact about the 'cd' command..

- Shells also provide a small set of built-in commands (builtins) implementing functionality impossible or inconvenient to obtain via separate utilities. For example, cd, break, continue, and exec) cannot be implemented outside of the shell because they directly manipulate the shell itself. The history, getopts, kill, or pwd builtins, among others, could be implemented in separate utilities, but they are more convenient to use as builtin commands. All of the shell builtins are described in subsequent sections.
- 'cd' is a type of program that is run differently in the shell...there are certain commands that the shell cannot simply dispatch to another process. These are things which affect something internal to the shell, and thus must be implemented by the shell itself
  - Dennis Ritchie:
    - In the midst of our jubilation, it was discovered that the chdir (change current directory) command had stopped working. There was much reading of code and anxious introspection about how the addition of fork could have broken the chdir call. Finally the truth dawned: in the old system chdir was an ordinary command; it adjusted the current directory of the (unique) process attached to the terminal. Under the new system, the chdir command correctly changed the current directory of the process created to execute it, but this process promptly terminated and had no effect whatsoever on its parent shell! It was necessary to make chdir a special command, executed internally within the shell. It turns out that several command-like functions have the same property, for example login.
    - Source: Dennis M. Ritchie, “The Evolution of the Unix Time-sharing System”, AT&T Bell Laboratories Technical Journal 63(6), Part 2, Oct. 1984, pp.1577–93
      - Because a new process is created to execute each command, chdir would be ineffective if it were written as a normal command. It is therefore recognized and executed by the Shell.

### Shell Operation.

- description of the shell’s operation when it reads and executes a command..
  1. Reads its input from a file (see Shell Scripts), from a string supplied as an argument to the -c invocation option (see Invoking Bash), or from the user’s terminal.
  2. Breaks the input into words and operators, obeying the quoting rules described in Quoting. These tokens are separated by metacharacters. Alias expansion is performed by this step (see Aliases).
  3. Parses the tokens into simple and compound commands (see Shell Commands).
  4. Performs the various shell expansions (see Shell Expansions), breaking the expanded tokens into lists of filenames (see Filename Expansion) and commands and arguments.
  5. Performs any necessary redirections (see Redirections) and removes the redirection operators and their operands from the argument list.
  6. Executes the command (see Executing Commands).
  7. Optionally waits for the command to complete and collects its exit status (see Exit Status).
