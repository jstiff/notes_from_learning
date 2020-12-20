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

In RegEx's you can have a combination of the alphabet based on three operations...

1. Concatenation
2. Union
3. \*

### Thompson Construction

We can assume that every NFA can have only one accept state...by haveing all of its possible accept state map to a final accept state.

- What if we have an NFA for a Language 1 (L1) and a seperate NFA for a L2 and we want an NFA that is the Union of these two languages? ----> L1 U L2

We can branch!

```
        *******************
        |     L1 U L2     |
        *******************


        / NFA 1 ----> ACCEPT STATE    \
       /                               \
---->                                     ACCEPT STATE
       \                               /
        \ NFA2  -----> ACCEPT STATE   /



        either each NFA has an accpet state or you can map both to a final Accept state


      **************************
      Concatenation L1 & L2
      **************************

      ----> NFA1 -----> NFA2 ------> ACCEPT STATE


      **************************
        NFA for *
      **************************

      ???research







```

We construct an NFA for each element in the Regex string. Break it up into many NFA's ...then combine them together.

### Subset construction

idea of converting NFA to DFA is to think of a combination of potential states as **one** state. For example, if an NFA has 3 possible choices for letter 'a'...we think of these 3 possibilities as one DFA state. Remember that DFA's are the easiest to program so this is why we want to convert the NFA to DFA.

If there are 3 NFA states you can have 8 DFA states.

- DFA = 2^n .... where n === Number of NFA states.

- The DFA's are actually the 'Powerset' (all subsets of a set) of the set of NFA states. All possible combinations of sets...'subsets'. The DFA 'power set' is the combination of all subsets including null set, of a given set.
- Each of these subsets derived from an NFA set is known as the 'Epsilon Closure' (???)

This is why is it called **Subset Construction**.

To traverse a graph in with the goal of building Epsilon Closure....we can use graph algoithms such as 'depth first' or 'breath first'. Traversing only the Epsilons and ignoring everythin else....(???)

- there seems to be 'redundency' when converting NFA to DFA that can be eliminated. (keep in mind!!!) when mapping the NFA states into DFA states.

- If there is an NFA with 10 states...this will mean that there could be up to 2^10 DFA states...(1024 states). Obviousbly this is unmanagable to draw out all possible combination and then eliminate the redundancy. So there is a way to start out with only the 'reachable' states....or process to 'discover' the reachable DFA states.

If you work out the epsilon closures you will only need 5 states.
