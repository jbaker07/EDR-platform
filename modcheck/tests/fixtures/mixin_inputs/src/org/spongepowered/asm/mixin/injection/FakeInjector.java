package org.spongepowered.asm.mixin.injection;

import java.lang.annotation.*;

/** NOT a real Mixin annotation: exists so the extractor's failure path is exercised by a compiled input. */
@Target(ElementType.METHOD) @Retention(RetentionPolicy.RUNTIME)
public @interface FakeInjector { String method(); }
