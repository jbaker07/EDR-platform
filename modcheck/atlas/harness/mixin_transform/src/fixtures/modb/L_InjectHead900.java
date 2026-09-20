package fixtures.modb;
import fixtures.targets.L; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(value = L.class, priority = 900) public class L_InjectHead900 {
    @Inject(method = "run", at = @At("HEAD")) private void head(CallbackInfo ci) { Trace.log("L2.head.p900"); }
    @Inject(method = "run", at = @At("TAIL")) private void tail(CallbackInfo ci) { Trace.log("L2.tail.p900"); }
}
