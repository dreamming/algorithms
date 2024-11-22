# Environment Overview
The SFC simulation functionality is implemented in Java. It can be executed on any machine with a Java runtime environment by following the instructions below.

# Running the SFC Simulator

1. Compile the code:
javac SFCSimulator.java

2. Execute the program
java SFCSimulator

# Feature Description
Due to the randomness of the SFC delay threshold and propagation delay, the method SFCSimulator#simulate may struggle to produce valid SFC results. To facilitate result validation, the program provides the SFCSimulator#showValidSFC method, which can be used to obtain different simulation results.