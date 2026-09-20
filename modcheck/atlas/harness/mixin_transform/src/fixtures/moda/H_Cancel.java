package fixtures.moda;
import fixtures.targets.H; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(H.class) public class H_Cancel {
    @Inject(method = "run", at = @At("HEAD"), cancellable = true) private void head(CallbackInfo ci) { Trace.log("H1.head.cancel"); ci.cancel(); }
}
