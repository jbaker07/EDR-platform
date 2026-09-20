package harness;

import org.spongepowered.asm.service.IMixinServiceBootstrap;

public class HarnessBootstrap implements IMixinServiceBootstrap {
    @Override public String getName() { return "harness"; }
    @Override public String getServiceClassName() { return "harness.HarnessService"; }
    @Override public void bootstrap() { }
}
