package fixtures.targets;

import harness.Trace;

public class A {
    public static void go() { new A().run(); }
    public void run() { Trace.log("A.run"); step(); }
    public void step() { Trace.log("A.step"); }
}
