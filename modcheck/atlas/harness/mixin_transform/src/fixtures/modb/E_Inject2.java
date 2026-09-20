package fixtures.modb;
import fixtures.targets.E; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(E.class) public class E_Inject2 {
    @Inject(method = "run", at = @At("HEAD")) private void head(CallbackInfo ci) { Trace.log("E2.head"); }
    @Inject(method = "run", at = @At("TAIL")) private void tail(CallbackInfo ci) { Trace.log("E2.tail"); }
}
