package fixtures.modb;
import fixtures.targets.A; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(A.class) public class A_Inject {
    @Inject(method = "run", at = @At("HEAD")) private void head(CallbackInfo ci) { Trace.log("A2.head"); }
    @Inject(method = "run", at = @At("TAIL")) private void tail(CallbackInfo ci) { Trace.log("A2.tail"); }
}
