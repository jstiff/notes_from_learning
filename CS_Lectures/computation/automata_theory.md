## Automata

- The purpose of Automata is to **define languages**...
- the study of abstract machines and automata, as well as the computational problems that can be solved using them.
- Automata is from the Greek meaning **'Self-making'**
  - an abstract self-propelled computing device which follows a predetermined sequence of operations.
  - An automaton with a finite number of states is called a Finite Automaton (FA) or Finite State Machine (FSM).
  - Automata theory is closely related to formal language theory.
  - Automata are often classified by the class of formal languages they can recognize, typically illustrated by the Chomsky hierarchy, which describes the relations between various languages and kinds of formalized logics
  - Automata play a major role in theory of **computation, compiler construction, artificial intelligence, parsing and formal verification**.
  - An automaton runs when it is given some sequence of inputs in discrete (individual) time steps or steps.
  - automaton is a mathematical object that takes a word as input and decides whether to accept it or reject it. Since all computational problems are reducible into the accept/reject question on inputs, (all problem instances can be represented in a finite length of symbols).
  - automata theory plays a crucial role in computational theory.

### Hierarchy of Automata...lowest to increasing power.

    0. Combinational logic (combinatorial)....('a type of digital logic which is implemented by Boolean circuits).
    1. Deterministic Finite Automaton (DFA)......'regular languages'
    2. Nondeterministic Finite Automaton (NFA)
    3. Deterministic Push Down Automaton (DPDA-I) with 1 push-down store(stack)....'context free languages'
    4. Nondeterministic Push Down Automaton (NPDA-I) with 1 push-down store (stack)
    5. Linear Bounded Automaton (LBA)...'context sensitive languages'
    6. Deterministic Push Down Automaton (DPDA-II) with 2 push-down stores.
    7. Nondeterministic Push Down Automaton (NPDA-II) with 2 push-down stores.
    8. Deterministic Turing Machine (DTM).
    9. Nondeterministic Turing Machine (NTM).
    10. Probabilistic Turing Machine (PTM).
    11. Multitape Turing Machine (MTM).
    12. Multidimensional Turing Machine.

### Each model in automata theory plays important roles in several applied areas.

    - Finite automata are used in text processing, compilers, and hardware design.
    - Context-free grammar (CFGs) are used in programming languages and artificial intelligence.
    - Cellular automata are used in the field of biology, the most common example being John Conway's Game of Life.
    - a theory suggesting that the whole universe is computed by some sort of a discrete automaton, is advocated by some scientists (The idea originated in the work of Konrad Zuse).
    -

### Boolean differential calculus (BDC)

    - a subject field of Boolean algebra discussing changes of Boolean variables and Boolean functions.
    -  Boolean differential calculus allows various aspects of dynamical systems theory such as
        - automata theory on finite automata
        - A Petri net, also known as a place/transition (PT) net, is one of several mathematical modeling languages for the description of distributed systems.
        - supervisory control theory (SCT)

### review

- remember that a 'regular' language is that can or will be proccessed and acccepted by a DFA. A very well known and widely used example of a regular language are the strings that make up **floating point numbers**.
- **Non-regular** languages have an unknowns....in languages that are 'balanced' for instance.
  An example of balanced would be a sequence of parentheses that **can appear in an arithmetic expression**.

  - {0,0,0,1,1,1} or {(), ()(), (()), (()()) ...}. There needs to be a mechnism that will allow for us to remember how many input values have come before...such as the 0's in the first example...in order to match the 1's.
