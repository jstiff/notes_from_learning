## Backend phase of a compilier.

- This is ideally where the different hardware architectures will be targeted.
  'gcc' has a pretty huge backend that targets all architectures.

  In this step the compilier

  1. Instruction selection
  2. Instruction Scheduling.
  3. Register allocation.

Instructions and **'Virtual registers'** are destermined and the **Register Allocator** takes the virtual ones and
maps them to real ones.

```

             AST        -------->      Instruction Selection (Still is an IR)

              *
            a   b




            a * b                       load a -> R1            R1 and R2 are virtual registers
                                        load b -> R2
                                        mult R1 R2 -> R3


  - 'load a -> R1' is a simplification of the syntax. This procedure is put into an 'activation record' that will have an address and each variable will be an 'offset'. There is a special register that will contain the address of the current 'activation record'.

```

**Depth First Algorithm** would be used to traverse the AST tree in order to convert it to this Instruction Selection repesentation.

**Instruction Selection** tries to utilize the 'instruction set' of a given machine in the most efficient way possible. For example, the 'a \* b' example above, if an instruction set has a built in multiplying instruction the right 'instruction selection' would be to use the built in mult to avoid the load a ....then load b...then mult...etc. Whereas another achitecture may not have a way of dealing with multiplication.

**Instruction Scheduling** is a method of 'hideing latency' between loading operations...say if 'mult R1 R2' takes
a long time you could load another value into a diff register while you're waiting for the loads to finnish. This optimization will put strain on the registers. Optimization is in essence taking full advantage of registers. But, in reality we only have so many registers.

```
                                    AST                              IR                       cycles (pipelining)
                                 -----------------------           -------------         ----------------------
  x = a * b + c/d                           assign                  load a -> R1            1 , 3
                                           /      \                 load b -> R2            2 , 4
                                          x        +                mult R1 R2 -> R3        5 , 6
                                                /     \             load c -> R4            6, 8
                                               *       /            load d -> R5            7, 9
                                              / \     / \           div R4, R5 -> R6        10, 14
                                             a   b   c   d          add R3, R6 ->R7         15, 15
                                                                    store R7 -> x;          16, 17

The variables a, b ,c etc are just offsets to the 'activation record' pointer.

Latencies
---------
Load: 3 cycles
mult: 2 cycles
div: 5 cycles
store: 2 cycles
add: 1 cycle
----------

This is executed in 17 clock cycles. Pipelining occurs whenever there are registers that do not depend on each other...instructions are able to 'overlap' each other.
```

**Register Allocation** will take all of the 'virtual registers' generated in previoius IR and map them to real registers on the machine. Let's say that there are only 3 registers. P1, P2, P3. But we have 10 virtual registers in the IR.

```
In the example above the Register allocator might do something like...

                 load a -> R1           ===>            load a -> P1
                 load b -> R2                           load b -> P2
                 mult R1 R2 -> R3                       mult P1 P2 -> P1


It reused P1 register because in reality it was not being used. If and only if there is no other place in the IR where R1 is used again. The IR gives us a picture of what can be optimized and what cannot.

```

'Live Range' of a register is the length of time it will be needed in the lifetime of the exectution. The longer the 'live ranges' these registers have the more 'Register Pressure' exists in the program. Their is essentially a stronger 'competition' for registers throughout the process.

So as part of the Register allocation ..the point is to analyse the intermediate representation of all the virtual registers and determine their 'live ranges' so as to allocate registers in the most efficient way possible. If there are not enough registers then some variables might be allocated out to memory somewhere...slowing things down.

'Spilling'...is when there are not enough registers and outside memory is needed. It is the job of the Register Allocator to minimize 'spilling'.

Most likely anything that is 'spilled' goes to the stack memory or the L1 cache. This is still slower than registers. Accessing L1 chache will take 2-3 cycles. Main memory is 100's of cycles.

Unfortunately, optimizations that are made to the code require more registers. So in the optimization phase it is important to understand your register limitations and how to accomidate the particular Register Allocator for that architecture.
