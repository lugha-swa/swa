//! Ufafanuzi wa ABI (Application Binary Interface) kwa lugha ya Swa.
//!
//! ## Swa ABI v1.0 — muhtasari
//!
//! | Kanuni                              | Tabia                               |
//! |-------------------------------------|-------------------------------------|
//! | Kurudisha skala                     | Kwenye rejista (namba sahihi au float)|
//! | Kurudisha muundo, sehemu 1–2        | Moja kwa moja (sehemu kwenye rejista)|
//! | Kurudisha muundo, > sehemu 2        | Kielekezi fiche (sret)              |
//! | Hoja za muundo > sehemu 2           | Kwa kurejelea / kunakili            |
//!
//! Mantiki ya uainishaji iko kwenye moduli ndogo ya [`classify`].

pub mod classify;

pub use classify::classify_return;
