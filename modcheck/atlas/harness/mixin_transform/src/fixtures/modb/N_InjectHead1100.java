package fixtures.modb;
import fixtures.targets.N; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(value = N.class, priority = 1100) public class N_InjectHead1100 {
    @Inject(method = "run", at = @At("HEAD")) private void head(CallbackInfo ci) { Trace.log("N2.head.p1100"); }
}
