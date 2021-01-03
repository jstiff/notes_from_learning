## CPU stuff...

#### I/O

Cpu's are 'bound'...**i/o bound** or affected by the time it takes for i/o requests to finish. They are not blocked, but bound...

- as for as I/O is concerned. The CPU **initiates** all input/output requests....but, when the CPU issues an I/O request to the hard disk, the hard disk has its own specialized chip called a device (or hardware) controller designed solely for processing commands from the CPU, such as reading from the disk.
- Modern hardware controllers are basically their own microprocessors with firmware and everything, so they are capable of very complex operations without the main CPU's help. While the hard drive's controller is busy performing the request, the main CPU is free to do whatever it wishes.
- The controller is able to read and write directly to and from system RAM using what is called a Direct Memory Access (DMA) controller, a special unit that transfers data from the hardware controller to main RAM without the CPU needing to do anything.
- When the hard drive is done with the request and the relevant data has been loaded into RAM through DMA, it issues an interrupt request which informs the CPU that the data has been loaded into RAM. At this point the CPU can transfer control back to an originating process.
- CPU does not need to micromanage all tasks involved with I/O. At one time this used to be the case, but these tricks (interrupts, DMA, special controllers) were invented in order to improve CPU performance and make things more efficient.
