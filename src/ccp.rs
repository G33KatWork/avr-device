//! Configuration change protected (CCP) register definitions

pub use crate::generic::ProtectedWritable;

#[cfg(feature = "attiny417")]
pub mod attiny417 {
    use crate::generic::{UnlockRegister, Protected};

    // Mark the CPU.CCP register with the UnlockRegister trait so that it can be used to unlock the below defined registers
    impl UnlockRegister for crate::attiny417::cpu::ccp::CcpSpec { const PTR: *mut u8 = 0x34 as *mut u8; }

    // Configuration change protected registers in NVMCTRL
    impl Protected for crate::attiny417::nvmctrl::ctrla::CtrlaSpec { const MAGIC: u8 = 0x9D; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny417::nvmctrl::ctrlb::CtrlbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in CLKCTRL
    impl Protected for crate::attiny417::clkctrl::mclkctrlb::MclkctrlbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny417::clkctrl::mclklock::MclklockSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny417::clkctrl::xosc32kctrla::Xosc32kctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny417::clkctrl::mclkctrla::MclkctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny417::clkctrl::osc20mctrla::Osc20mctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny417::clkctrl::osc20mcaliba::Osc20mcalibaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny417::clkctrl::osc20mcalibb::Osc20mcalibbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny417::clkctrl::osc32kctrla::Osc32kctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in RSTCTRL
    impl Protected for crate::attiny417::rstctrl::swrr::SwrrSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in CPUINT
    impl Protected for crate::attiny417::cpuint::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in BOD
    impl Protected for crate::attiny417::bod::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in WDT
    impl Protected for crate::attiny417::wdt::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny417::wdt::status::StatusSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in TCD0
    impl Protected for crate::attiny417::tcd0::faultctrl::FaultctrlSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny417::cpu::ccp::CcpSpec; }
}

#[cfg(feature = "attiny817")]
pub mod attiny817 {
    use crate::generic::{UnlockRegister, Protected};

    // Mark the CPU.CCP register with the UnlockRegister trait so that it can be used to unlock the below defined registers
    impl UnlockRegister for crate::attiny817::cpu::ccp::CcpSpec { const PTR: *mut u8 = 0x34 as *mut u8; }

    // Configuration change protected registers in NVMCTRL
    impl Protected for crate::attiny817::nvmctrl::ctrla::CtrlaSpec { const MAGIC: u8 = 0x9D; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny817::nvmctrl::ctrlb::CtrlbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in CLKCTRL
    impl Protected for crate::attiny817::clkctrl::mclkctrlb::MclkctrlbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny817::clkctrl::mclklock::MclklockSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny817::clkctrl::xosc32kctrla::Xosc32kctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny817::clkctrl::mclkctrla::MclkctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny817::clkctrl::osc20mctrla::Osc20mctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny817::clkctrl::osc20mcaliba::Osc20mcalibaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny817::clkctrl::osc20mcalibb::Osc20mcalibbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny817::clkctrl::osc32kctrla::Osc32kctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in RSTCTRL
    impl Protected for crate::attiny817::rstctrl::swrr::SwrrSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in CPUINT
    impl Protected for crate::attiny817::cpuint::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in BOD
    impl Protected for crate::attiny817::bod::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in WDT
    impl Protected for crate::attiny817::wdt::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny817::wdt::status::StatusSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in TCD0
    impl Protected for crate::attiny817::tcd0::faultctrl::FaultctrlSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny817::cpu::ccp::CcpSpec; }
}

#[cfg(feature = "attiny1617")]
pub mod attiny1617 {
    use crate::generic::{UnlockRegister, Protected};

    // Mark the CPU.CCP register with the UnlockRegister trait so that it can be used to unlock the below defined registers
    impl UnlockRegister for crate::attiny1617::cpu::ccp::CcpSpec { const PTR: *mut u8 = 0x34 as *mut u8; }

    // Configuration change protected registers in NVMCTRL
    impl Protected for crate::attiny1617::nvmctrl::ctrla::CtrlaSpec { const MAGIC: u8 = 0x9D; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny1617::nvmctrl::ctrlb::CtrlbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in CLKCTRL
    impl Protected for crate::attiny1617::clkctrl::mclkctrlb::MclkctrlbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny1617::clkctrl::mclklock::MclklockSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny1617::clkctrl::xosc32kctrla::Xosc32kctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny1617::clkctrl::mclkctrla::MclkctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny1617::clkctrl::osc20mctrla::Osc20mctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny1617::clkctrl::osc20mcaliba::Osc20mcalibaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny1617::clkctrl::osc20mcalibb::Osc20mcalibbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny1617::clkctrl::osc32kctrla::Osc32kctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in RSTCTRL
    impl Protected for crate::attiny1617::rstctrl::swrr::SwrrSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in CPUINT
    impl Protected for crate::attiny1617::cpuint::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in BOD
    impl Protected for crate::attiny1617::bod::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in WDT
    impl Protected for crate::attiny1617::wdt::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny1617::wdt::status::StatusSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in TCD0
    impl Protected for crate::attiny1617::tcd0::faultctrl::FaultctrlSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1617::cpu::ccp::CcpSpec; }
}

#[cfg(feature = "attiny3217")]
pub mod attiny3217 {
    use crate::generic::{UnlockRegister, Protected};

    // Mark the CPU.CCP register with the UnlockRegister trait so that it can be used to unlock the below defined registers
    impl UnlockRegister for crate::attiny3217::cpu::ccp::CcpSpec { const PTR: *mut u8 = 0x34 as *mut u8; }

    // Configuration change protected registers in NVMCTRL
    impl Protected for crate::attiny3217::nvmctrl::ctrla::CtrlaSpec { const MAGIC: u8 = 0x9D; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny3217::nvmctrl::ctrlb::CtrlbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in CLKCTRL
    impl Protected for crate::attiny3217::clkctrl::mclkctrlb::MclkctrlbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny3217::clkctrl::mclklock::MclklockSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny3217::clkctrl::xosc32kctrla::Xosc32kctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny3217::clkctrl::mclkctrla::MclkctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny3217::clkctrl::osc20mctrla::Osc20mctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny3217::clkctrl::osc20mcaliba::Osc20mcalibaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny3217::clkctrl::osc20mcalibb::Osc20mcalibbSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny3217::clkctrl::osc32kctrla::Osc32kctrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in RSTCTRL
    impl Protected for crate::attiny3217::rstctrl::swrr::SwrrSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in CPUINT
    impl Protected for crate::attiny3217::cpuint::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in BOD
    impl Protected for crate::attiny3217::bod::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in WDT
    impl Protected for crate::attiny3217::wdt::ctrla::CtrlaSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
    impl Protected for crate::attiny3217::wdt::status::StatusSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }

    // Configuration change protected registers in TCD0
    impl Protected for crate::attiny3217::tcd0::faultctrl::FaultctrlSpec { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny3217::cpu::ccp::CcpSpec; }
}
