## Scanner

When you are building a scanner you are essentially defining all of the possible /'legal' tokens in that given language.

From the perspective of the scanner a valid language is a sequence of valid tokens that are defined to be valid.

Finite Automota and RegEx are tools for 'recognising' the language....'recognize' means that a FSA has taken a string and processed it to determine if it is 'accepted' or not. Does the FSA **end** in an 'accept'?

- Remember the process essentially is Regex --> NFA --> DFA conversion.

Scanner is built with these concepts and a scanner is the stage of the compilier in which we recognise words(tokens). Not sentences(statements).

```
                        human  |  machine
                        -------|---------
            scanner  =  words --> tokens
            parser   =  sentences -> statements


to recognise 'words' we can use NFA, DFA and RegEx.
The set of all words in a language is known as a 'Regular language'

Keywords, identifiers, numbers, operators....are elements of a regular language

Sigma = {0,1,2 ...9} U {a,b,c ...z} U { A, B, C ...Z } U { +, -, /, = ...}
           [0-9]                  [a-Z]


RegEx for unsigned int

    [0-9]*   ---> allows for leading 0's like 0000
    0 | [1-9][0-9]* ---> does not allow leading o's

    Not having leading 0's is a design choice...when building a compilier we simply need to know what the designers of the language prefer so the compilier can be built accordingly.

RegEx for language 'Identifiers'.
( _ | [a-z])([a-Z] | [0-9] | _ )*

        '_abn90' or 'bar_foo'.... would be accepted.

```
