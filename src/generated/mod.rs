#[cfg(feature = "damage")]
pub mod damage;
pub mod xproto;
#[cfg(feature = "xinput")]
pub mod xinput;
#[cfg(feature = "screensaver")]
pub mod screensaver;
#[cfg(feature = "xselinux")]
pub mod xselinux;
#[cfg(feature = "dri3")]
pub mod dri3;
#[cfg(feature = "xkb")]
pub mod xkb;
#[cfg(feature = "xv")]
pub mod xv;
#[cfg(feature = "res")]
pub mod res;
pub mod ge;
#[cfg(feature = "xfixes")]
pub mod xfixes;
#[cfg(feature = "xf86vidmode")]
pub mod xf86vidmode;
#[cfg(feature = "shape")]
pub mod shape;
#[cfg(feature = "shm")]
pub mod shm;
#[cfg(feature = "sync")]
pub mod sync;
pub mod bigreq;
#[cfg(feature = "xf86dri")]
pub mod xf86dri;
#[cfg(feature = "present")]
pub mod present;
#[cfg(feature = "xvmc")]
pub mod xvmc;
#[cfg(feature = "dri2")]
pub mod dri2;
#[cfg(feature = "glx")]
pub mod glx;
#[cfg(feature = "composite")]
pub mod composite;
#[cfg(feature = "randr")]
pub mod randr;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "record")]
pub mod record;
#[cfg(feature = "xprint")]
pub mod xprint;
#[cfg(feature = "xinerama")]
pub mod xinerama;
#[cfg(feature = "xevie")]
pub mod xevie;
#[cfg(feature = "dpms")]
pub mod dpms;
pub mod xc_misc;
#[cfg(feature = "xtest")]
pub mod xtest;
