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
            a * b                       load a -> R1            R1 is a virtual register
                                        load b -> R2
                                        mult R1 R2 -> R3

```

**Instruction Scheduling** is a method of 'hideing latency' between loading operations...say if 'mult R1 R2' takes
a long time you could load another value into a diff register while you're waiting for the loads to finnish.
