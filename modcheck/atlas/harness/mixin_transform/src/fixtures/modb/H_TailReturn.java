package fixtures.modb;
import fixtures.targets.H; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(H.class) public class H_TailReturn {
    @Inject(method = "run", at = @At("TAIL")) private void tail(CallbackInfo ci) { Trace.log("H2.tail"); }
    @Inject(method = "run", at = @At("RETURN")) private void ret(CallbackInfo ci) { Trace.log("H2.return"); }
}
