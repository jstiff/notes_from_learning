## Scanner

When you are building a scanner you are essentially defining all of the possible /'legal' tokens in that given language.

From the perspective of the scanner a valid language is a sequence of valid tokens that are defined to be valid.

Finite Automota and RegEx are tools for 'recognising' the language....'recognize' means that a FSA has taken a string and processed it to determine if it is 'accepted' or not. Does the FSA **end** in an 'accept'?

- Remember the process essentially is Regex --> NFA --> DFA conversion.

Scanner is built with these concepts and a scanner is the stage of the compiler in which we recognise words(tokens). Not sentences(statements).

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

    Not having leading 0's is a design choice...when building a compiler we simply need to know what the designers of the language prefer so the compiler can be built accordingly.

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

### Algorithm for Subset Construction (powerset construction)

It is important in theory because it establishes that NFAs, despite their additional flexibility, are unable to recognize any language that cannot be recognized by some DFA. It is also important in practice for converting easier-to-construct NFAs into more efficiently executable DFAs. However, if the NFA has n states, the resulting DFA may have up to 2n states, an exponentially larger number, which sometimes makes the construction impractical for large NFAs.

- Full list: complete list of discoverable DFA states
- Work list: list of DFa states that have been discovered, but not processed yet.

### Implementation

There are tools that can translate our RegEx to DFA's for us. We would provide a file as such .

```

RegEx's
*******

- RegEx for Keywords (if, else, for , while)
- RegEx for Ints
- RegEx for Identifiers
- RegEx for floats



the union of all of these is whole programming language...

Notice that the 'keyword' regEx is first in the file...this is to set priority over the 'identifier'. This will help in avoiding if an Keyword like 'for' 'if' 'else' is accidentally used as in Identifier...

Each keyword will have it's own RegEx and be created as a seperate branch in an NFA...

Another way to recognise 'keywords' in a language is to process them as 'identifiers' and then check them for being a 'keyword' or not. This would eliminate all of the branching in the NFA for processing unique keywords.

There would need to be a 'Keyword Table' and the best way to implement it would be to make it a 'Hash Table'.
Because we know exactly how many enteries in the Hash Table there is a search on it will be of constant O(1).

This will reduce the amount of states involved with the FA , but then again there will be more 'identifier' written in a program than 'keywords', so do we want to check the keyword table for every indetifier processed? This is overhead that will effect the compile time.

```

### Scanner generator

```
                    Scanner generator
            -----------------------------------------------------------------------------------------
           |                                                                                         |
RegEx ---> | Thomspson                                                                               |
           | construction --> NFA --> Subset cont --> DFA --> DFA minimization algo --> code gen --> |-->  Scanner
           |                                                                                         |      code
            ------------------------------------------------------------------------------------------


    This process is for one aspect in the building of compiler. It is not part of the compilation process.  It generates the code that will make up the 'scanner'. Which will in turn process the language into accepted tokens. It's a tool...'scanner tool'.

    You can use 'Flex' to build a scanner.
```

### From DFA to Scanner Code

Once the DFA are created we need to generate the code that implements or simulates that DFA

- 2 ways to do it....

1.  Table Driven Approach
    - Table means 2-demensional array where rows are the 'states' and the columns are the 'symbols' of the language.
2.  Direct-coded

There needs to be an 'error' state that represents the end of a match.

- The '=' operator can be used as an error state. (???).
  When an error state is encountered you need to 'roll back' to the previous accept state. This is done on a 'stack', so to 'backtrack' you will need to 'pop' off the stack untill the accept state desired is reached.

- How do we determine what is an accept state? ...Prob an object that has a field 'AcceptState = true;' or something like this.

Tables can have lots of redundency that can be cleaned up...for example all letters [a-Z] will most likely have the same acceptence states and transitions. In order to simplify it... We can have two tables....one that establishes 'Categories' like 'numbers' which will represent symbols that all share the same DFA states.
The name of the game is to always reduce redundancy for effiency and logical processing. These are column redundancies....

there is also Row (states) redundancy that can be taken care of durring the scanner generator phase with **'DFA minimization'**. This is about mergin multiple states that have the same transitions into one state. If there is logical equivalence then we can simplify.

- The Price to pay for eleminating the redundancy of the table is now there are two Tables ....two lookup tables stored in memory. Must consider this.

But if the original Table is too big for the cache it will also cause a slowdown in the program. A trade off analysis needs to be done in order to decide on what is more appropriate. Every situation has it's own set of trade offs.

- How do we make all this work....?....then how do we make it the most efficient as possible...???

### Table Driven Scanner

```
Pseudo code

CatTable = [];
TransitionTable = [[]];    // two demensional array
State = S0;
Se = error state;
lexeme = empty            ///lexeme is the string being processed. the final 'accepted' string like an identifier
stack.push(NULL);         /// null for start of stack or beginning of lexeme
stack.clear();


while(state != Se){
    char = getNextChar();
    lexeme = lexeme + char;

    if(state 'belongs' to acceptState){
        stack.clear();
    }

    stack.push(state);
    cat = CatTable[char];
    state = TransitionTable[state][cat]
}

while(state belongs to AceeptState and !NULL){
    state = stack.pop();
    truncate lexeme;           //remove last character..lexeme is temp stored in a buffer
    ROLLBACK;

    if(state is member of AcceptState){
        return type[state]

    }else return error;
}
```

### Direct Coded

```
    No lookup tables...just direct each character to the appropriate state...
    This will look very ugly if you need to code it by hand, but a generator can do this for you.
    this version will avoid memory accesses.

```

There are many types of 'Scanner generators'....

- Flex (C, C++ DFA Table Driven)
- golex (go)
- Lex
- Ragel
- JFlex (java)
- re2c (C)
- Astir (C++ Table driven with braches)
- rustlex_codegen
- Etc.....
