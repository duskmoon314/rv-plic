#[doc = "Register `enable_s%s` reader"]
pub struct R(crate::R<ENABLE_S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `enable_s%s` writer"]
pub struct W(crate::W<ENABLE_S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ENABLE_S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_S_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable bits of source `s`\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_s](index.html) module"]
pub struct ENABLE_S_SPEC;
impl crate::RegisterSpec for ENABLE_S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable_s::R](R) reader structure"]
impl crate::Readable for ENABLE_S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable_s::W](W) writer structure"]
impl crate::Writable for ENABLE_S_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets enable_s%s to value 0"]
impl crate::Resettable for ENABLE_S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
