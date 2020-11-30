# Computation

-These are notes based on a youtube channel video series.
'Theory of Computation #01 Intro to TOC & Finite Automata | Examples' <br/>
<https://www.youtube.com/watch?v=Jbfto52zr0Q&list=PLEJxKK7AcSEEYrMd4G7Y3mjGlE651B7yc>

## Start by asking what is 'computation'?

- Computation is a 'process' or a means to solving a problem which can be executed on 'devices'.

  - a "device" can be anything from a computer to a piece of paper and pen. A stone tablet
    can be a means to solving a problem.(device)
  - The step by step instructions of this process to solving a problem is known as an "Algorithm".

<center><h2>Finite Automata</h2></center>

- In informal terms finite automaton (or finite state machine) is an abstract machine that has states and transitions between these states. It is always in one of its states and while it reads an input it switches from state to state. It has a start state and can have one or more end (accepting) states. Can be used to accept string values or map values to diff values. It's a mapping network of finite available states which 'language syntax value representations/entities' can traverse the 'machine' (computation entity) and ultimatly produce an intended outcome. In some cases that is 'acceptence' by the machine or value transistion/shift/translation.

  'Finite Automata' is a model of a computation with a finite amount of memory or
  states. Which can be exeuted on any number of devices. Finite automata are used to recognize patterns of strings, regular expressions are used to generate patterns of strings. Each can be converted between the other. Regex to FS and FS to Regex. **Deterministic Machines** (DFSM) mean there is an un-ambiguous mapping of a string element being processed to a definite state change. A string element being processed will never have more than one option in the next state change.

  We can say that a language that is accepted by a FSM is the set of alphanumeric values that the FSM will accept. Meaning that the FSM defines what language is by its 'computational acceptence' of it which is then regaurded as being Regular or FSM approved!!! I'm assuming then that a FSM can catch syntax errors in IDE's.

  ```
  DFSM = M
  Q = Set of States of M
  Sigma = alphabet used for the strings
  Delta = transition function

  Q x Sigma -> Q ..... state takes a string and moves into another state.

  ```

  They describe the same languages. Not 'human' languages, but 'Formal'languages (set of strings accepted by some rule) They describe a particular form of language called **"Regular language"**. A language is regular if there is a Deterministic Finite Automota for it....

  - Variables, Numbers, URL's, email, phone number can be described by Reg Lang.

  People prefer RegExpressions, but computers prefer Finite Autotomota. So there are programs that convert between the two.

```
ex) Electric switch.
      State is either 'on' or 'off'

```

![](images/switch.png)

```
    ex2) Fan regulator.
    	Has 4 states which can be incrementally approached as low->med->High-off states. With
    	push operations in 'clockwise' and 'ani-clockwise' directions. Each state can be represented
    	as 1 of any 4 possible 2 digit binary values.
    		00 <-> 10 <-> 11 <-> 01
```

![](images/four-way.png)

```
ex3) Machine that accepts only strings divisable by 4 It just so happens that if any Binary value ends in 2 consecutive 0's...that binary value is divisable by 4. So we need a Machine to accept only strings that end in two 0's as it's final state.

Processing a stream of Binary values. State represented as two bits...by shifting over to next
    	bit in the stream will represent a change in state. Feed string 10100 into the machine.
    		|10|100 ==> 1|01|00 ==> 10|10|0 ==> 101|00|
    		    |           |           |          |
    		    |           |           |          |
    		   rej         rej         rej        accept

			   If state ends in anything other than 00 it is rejected. This string is accepted.

```
