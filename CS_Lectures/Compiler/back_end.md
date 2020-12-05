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
a long time you could load another value into a diff register while you're waiting for the loads to finnish.

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
