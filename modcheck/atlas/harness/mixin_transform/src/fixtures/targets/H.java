package fixtures.targets;

import harness.Trace;

public class H {
    public static void go() { new H().run(); }
    public void run() { Trace.log("H.run"); step(); }
    public void step() { Trace.log("H.step"); }
}
