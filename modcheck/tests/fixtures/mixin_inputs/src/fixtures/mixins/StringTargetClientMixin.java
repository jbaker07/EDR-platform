package fixtures.mixins;

import net.fabricmc.api.EnvType;
import net.fabricmc.api.Environment;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Pseudo;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

@Environment(EnvType.CLIENT)
@Pseudo
@Mixin(targets = "fixtures.target.Target$Inner", priority = 1100)
public class StringTargetClientMixin {
    @Inject(method = "inner", at = @At("HEAD"))
    private void onInner(CallbackInfo ci) { }
}
