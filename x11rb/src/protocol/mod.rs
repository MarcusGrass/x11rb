// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the X11 protocol.
//!
//! Each sub-module of this module corresponds to one X11 extension. It contains all the
//! definitions from that extension. The core X11 protocol is in [`xproto`](xproto/index.html).

// Clippy does not like some names from the XML.
#![allow(clippy::upper_case_acronyms)]
// This is not easy to fix, so ignore it.
#![allow(clippy::needless_borrow, clippy::needless_lifetimes)]

use crate::errors::ParseError;
use crate::x11_utils::ExtensionInfoProvider;
use crate::x11_utils::{TryParse, X11Error};
use core::convert::TryInto;
#[allow(unused_imports)]
use std::borrow::Cow;

pub mod bigreq;
#[cfg(feature = "composite")]
pub mod composite;
#[cfg(feature = "damage")]
pub mod damage;
#[cfg(feature = "dpms")]
pub mod dpms;
#[cfg(feature = "dri2")]
pub mod dri2;
#[cfg(feature = "dri3")]
pub mod dri3;
pub mod ge;
#[cfg(feature = "glx")]
pub mod glx;
#[cfg(feature = "present")]
pub mod present;
#[cfg(feature = "randr")]
pub mod randr;
#[cfg(feature = "record")]
pub mod record;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "res")]
pub mod res;
#[cfg(feature = "screensaver")]
pub mod screensaver;
#[cfg(feature = "shape")]
pub mod shape;
#[cfg(feature = "shm")]
pub mod shm;
#[cfg(feature = "sync")]
pub mod sync;
pub mod xc_misc;
#[cfg(feature = "xevie")]
pub mod xevie;
#[cfg(feature = "xf86dri")]
pub mod xf86dri;
#[cfg(feature = "xf86vidmode")]
pub mod xf86vidmode;
#[cfg(feature = "xfixes")]
pub mod xfixes;
#[cfg(feature = "xinerama")]
pub mod xinerama;
#[cfg(feature = "xinput")]
pub mod xinput;
#[cfg(feature = "xkb")]
pub mod xkb;
#[cfg(feature = "xprint")]
pub mod xprint;
pub mod xproto;
#[cfg(feature = "xselinux")]
pub mod xselinux;
#[cfg(feature = "xtest")]
pub mod xtest;
#[cfg(feature = "xv")]
pub mod xv;
#[cfg(feature = "xvmc")]
pub mod xvmc;

/// Get the name of a request from its extension name and opcodes.
pub(crate) fn request_name(
    extension: Option<&str>,
    major_opcode: u8,
    minor_opcode: u16,
) -> Option<&'static str> {
    // Check if this is a core protocol request.
    match major_opcode {
        1 => return Some("CreateWindow"),
        2 => return Some("ChangeWindowAttributes"),
        3 => return Some("GetWindowAttributes"),
        4 => return Some("DestroyWindow"),
        5 => return Some("DestroySubwindows"),
        6 => return Some("ChangeSaveSet"),
        7 => return Some("ReparentWindow"),
        8 => return Some("MapWindow"),
        9 => return Some("MapSubwindows"),
        10 => return Some("UnmapWindow"),
        11 => return Some("UnmapSubwindows"),
        12 => return Some("ConfigureWindow"),
        13 => return Some("CirculateWindow"),
        14 => return Some("GetGeometry"),
        15 => return Some("QueryTree"),
        16 => return Some("InternAtom"),
        17 => return Some("GetAtomName"),
        18 => return Some("ChangeProperty"),
        19 => return Some("DeleteProperty"),
        20 => return Some("GetProperty"),
        21 => return Some("ListProperties"),
        22 => return Some("SetSelectionOwner"),
        23 => return Some("GetSelectionOwner"),
        24 => return Some("ConvertSelection"),
        25 => return Some("SendEvent"),
        26 => return Some("GrabPointer"),
        27 => return Some("UngrabPointer"),
        28 => return Some("GrabButton"),
        29 => return Some("UngrabButton"),
        30 => return Some("ChangeActivePointerGrab"),
        31 => return Some("GrabKeyboard"),
        32 => return Some("UngrabKeyboard"),
        33 => return Some("GrabKey"),
        34 => return Some("UngrabKey"),
        35 => return Some("AllowEvents"),
        36 => return Some("GrabServer"),
        37 => return Some("UngrabServer"),
        38 => return Some("QueryPointer"),
        39 => return Some("GetMotionEvents"),
        40 => return Some("TranslateCoordinates"),
        41 => return Some("WarpPointer"),
        42 => return Some("SetInputFocus"),
        43 => return Some("GetInputFocus"),
        44 => return Some("QueryKeymap"),
        45 => return Some("OpenFont"),
        46 => return Some("CloseFont"),
        47 => return Some("QueryFont"),
        48 => return Some("QueryTextExtents"),
        49 => return Some("ListFonts"),
        50 => return Some("ListFontsWithInfo"),
        51 => return Some("SetFontPath"),
        52 => return Some("GetFontPath"),
        53 => return Some("CreatePixmap"),
        54 => return Some("FreePixmap"),
        55 => return Some("CreateGC"),
        56 => return Some("ChangeGC"),
        57 => return Some("CopyGC"),
        58 => return Some("SetDashes"),
        59 => return Some("SetClipRectangles"),
        60 => return Some("FreeGC"),
        61 => return Some("ClearArea"),
        62 => return Some("CopyArea"),
        63 => return Some("CopyPlane"),
        64 => return Some("PolyPoint"),
        65 => return Some("PolyLine"),
        66 => return Some("PolySegment"),
        67 => return Some("PolyRectangle"),
        68 => return Some("PolyArc"),
        69 => return Some("FillPoly"),
        70 => return Some("PolyFillRectangle"),
        71 => return Some("PolyFillArc"),
        72 => return Some("PutImage"),
        73 => return Some("GetImage"),
        74 => return Some("PolyText8"),
        75 => return Some("PolyText16"),
        76 => return Some("ImageText8"),
        77 => return Some("ImageText16"),
        78 => return Some("CreateColormap"),
        79 => return Some("FreeColormap"),
        80 => return Some("CopyColormapAndFree"),
        81 => return Some("InstallColormap"),
        82 => return Some("UninstallColormap"),
        83 => return Some("ListInstalledColormaps"),
        84 => return Some("AllocColor"),
        85 => return Some("AllocNamedColor"),
        86 => return Some("AllocColorCells"),
        87 => return Some("AllocColorPlanes"),
        88 => return Some("FreeColors"),
        89 => return Some("StoreColors"),
        90 => return Some("StoreNamedColor"),
        91 => return Some("QueryColors"),
        92 => return Some("LookupColor"),
        93 => return Some("CreateCursor"),
        94 => return Some("CreateGlyphCursor"),
        95 => return Some("FreeCursor"),
        96 => return Some("RecolorCursor"),
        97 => return Some("QueryBestSize"),
        98 => return Some("QueryExtension"),
        99 => return Some("ListExtensions"),
        100 => return Some("ChangeKeyboardMapping"),
        101 => return Some("GetKeyboardMapping"),
        102 => return Some("ChangeKeyboardControl"),
        103 => return Some("GetKeyboardControl"),
        104 => return Some("Bell"),
        105 => return Some("ChangePointerControl"),
        106 => return Some("GetPointerControl"),
        107 => return Some("SetScreenSaver"),
        108 => return Some("GetScreenSaver"),
        109 => return Some("ChangeHosts"),
        110 => return Some("ListHosts"),
        111 => return Some("SetAccessControl"),
        112 => return Some("SetCloseDownMode"),
        113 => return Some("KillClient"),
        114 => return Some("RotateProperties"),
        115 => return Some("ForceScreenSaver"),
        116 => return Some("SetPointerMapping"),
        117 => return Some("GetPointerMapping"),
        118 => return Some("SetModifierMapping"),
        119 => return Some("GetModifierMapping"),
        127 => return Some("NoOperation"),
        _ => (),
    }
    // Check the extension
    match (extension, minor_opcode) {
        (Some(bigreq::X11_EXTENSION_NAME), 0) => Some("Enable"),
        #[cfg(feature = "composite")]
        (Some(composite::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "composite")]
        (Some(composite::X11_EXTENSION_NAME), 1) => Some("RedirectWindow"),
        #[cfg(feature = "composite")]
        (Some(composite::X11_EXTENSION_NAME), 2) => Some("RedirectSubwindows"),
        #[cfg(feature = "composite")]
        (Some(composite::X11_EXTENSION_NAME), 3) => Some("UnredirectWindow"),
        #[cfg(feature = "composite")]
        (Some(composite::X11_EXTENSION_NAME), 4) => Some("UnredirectSubwindows"),
        #[cfg(feature = "composite")]
        (Some(composite::X11_EXTENSION_NAME), 5) => Some("CreateRegionFromBorderClip"),
        #[cfg(feature = "composite")]
        (Some(composite::X11_EXTENSION_NAME), 6) => Some("NameWindowPixmap"),
        #[cfg(feature = "composite")]
        (Some(composite::X11_EXTENSION_NAME), 7) => Some("GetOverlayWindow"),
        #[cfg(feature = "composite")]
        (Some(composite::X11_EXTENSION_NAME), 8) => Some("ReleaseOverlayWindow"),
        #[cfg(feature = "damage")]
        (Some(damage::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "damage")]
        (Some(damage::X11_EXTENSION_NAME), 1) => Some("Create"),
        #[cfg(feature = "damage")]
        (Some(damage::X11_EXTENSION_NAME), 2) => Some("Destroy"),
        #[cfg(feature = "damage")]
        (Some(damage::X11_EXTENSION_NAME), 3) => Some("Subtract"),
        #[cfg(feature = "damage")]
        (Some(damage::X11_EXTENSION_NAME), 4) => Some("Add"),
        #[cfg(feature = "dpms")]
        (Some(dpms::X11_EXTENSION_NAME), 0) => Some("GetVersion"),
        #[cfg(feature = "dpms")]
        (Some(dpms::X11_EXTENSION_NAME), 1) => Some("Capable"),
        #[cfg(feature = "dpms")]
        (Some(dpms::X11_EXTENSION_NAME), 2) => Some("GetTimeouts"),
        #[cfg(feature = "dpms")]
        (Some(dpms::X11_EXTENSION_NAME), 3) => Some("SetTimeouts"),
        #[cfg(feature = "dpms")]
        (Some(dpms::X11_EXTENSION_NAME), 4) => Some("Enable"),
        #[cfg(feature = "dpms")]
        (Some(dpms::X11_EXTENSION_NAME), 5) => Some("Disable"),
        #[cfg(feature = "dpms")]
        (Some(dpms::X11_EXTENSION_NAME), 6) => Some("ForceLevel"),
        #[cfg(feature = "dpms")]
        (Some(dpms::X11_EXTENSION_NAME), 7) => Some("Info"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 1) => Some("Connect"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 2) => Some("Authenticate"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 3) => Some("CreateDrawable"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 4) => Some("DestroyDrawable"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 5) => Some("GetBuffers"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 6) => Some("CopyRegion"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 7) => Some("GetBuffersWithFormat"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 8) => Some("SwapBuffers"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 9) => Some("GetMSC"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 10) => Some("WaitMSC"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 11) => Some("WaitSBC"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 12) => Some("SwapInterval"),
        #[cfg(feature = "dri2")]
        (Some(dri2::X11_EXTENSION_NAME), 13) => Some("GetParam"),
        #[cfg(feature = "dri3")]
        (Some(dri3::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "dri3")]
        (Some(dri3::X11_EXTENSION_NAME), 1) => Some("Open"),
        #[cfg(feature = "dri3")]
        (Some(dri3::X11_EXTENSION_NAME), 2) => Some("PixmapFromBuffer"),
        #[cfg(feature = "dri3")]
        (Some(dri3::X11_EXTENSION_NAME), 3) => Some("BufferFromPixmap"),
        #[cfg(feature = "dri3")]
        (Some(dri3::X11_EXTENSION_NAME), 4) => Some("FenceFromFD"),
        #[cfg(feature = "dri3")]
        (Some(dri3::X11_EXTENSION_NAME), 5) => Some("FDFromFence"),
        #[cfg(feature = "dri3")]
        (Some(dri3::X11_EXTENSION_NAME), 6) => Some("GetSupportedModifiers"),
        #[cfg(feature = "dri3")]
        (Some(dri3::X11_EXTENSION_NAME), 7) => Some("PixmapFromBuffers"),
        #[cfg(feature = "dri3")]
        (Some(dri3::X11_EXTENSION_NAME), 8) => Some("BuffersFromPixmap"),
        (Some(ge::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 1) => Some("Render"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 2) => Some("RenderLarge"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 3) => Some("CreateContext"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 4) => Some("DestroyContext"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 5) => Some("MakeCurrent"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 6) => Some("IsDirect"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 7) => Some("QueryVersion"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 8) => Some("WaitGL"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 9) => Some("WaitX"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 10) => Some("CopyContext"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 11) => Some("SwapBuffers"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 12) => Some("UseXFont"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 13) => Some("CreateGLXPixmap"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 14) => Some("GetVisualConfigs"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 15) => Some("DestroyGLXPixmap"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 16) => Some("VendorPrivate"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 17) => Some("VendorPrivateWithReply"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 18) => Some("QueryExtensionsString"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 19) => Some("QueryServerString"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 20) => Some("ClientInfo"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 21) => Some("GetFBConfigs"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 22) => Some("CreatePixmap"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 23) => Some("DestroyPixmap"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 24) => Some("CreateNewContext"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 25) => Some("QueryContext"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 26) => Some("MakeContextCurrent"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 27) => Some("CreatePbuffer"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 28) => Some("DestroyPbuffer"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 29) => Some("GetDrawableAttributes"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 30) => Some("ChangeDrawableAttributes"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 31) => Some("CreateWindow"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 32) => Some("DeleteWindow"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 33) => Some("SetClientInfoARB"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 34) => Some("CreateContextAttribsARB"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 35) => Some("SetClientInfo2ARB"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 101) => Some("NewList"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 102) => Some("EndList"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 103) => Some("DeleteLists"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 104) => Some("GenLists"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 105) => Some("FeedbackBuffer"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 106) => Some("SelectBuffer"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 107) => Some("RenderMode"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 108) => Some("Finish"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 109) => Some("PixelStoref"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 110) => Some("PixelStorei"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 111) => Some("ReadPixels"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 112) => Some("GetBooleanv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 113) => Some("GetClipPlane"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 114) => Some("GetDoublev"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 115) => Some("GetError"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 116) => Some("GetFloatv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 117) => Some("GetIntegerv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 118) => Some("GetLightfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 119) => Some("GetLightiv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 120) => Some("GetMapdv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 121) => Some("GetMapfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 122) => Some("GetMapiv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 123) => Some("GetMaterialfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 124) => Some("GetMaterialiv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 125) => Some("GetPixelMapfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 126) => Some("GetPixelMapuiv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 127) => Some("GetPixelMapusv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 128) => Some("GetPolygonStipple"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 129) => Some("GetString"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 130) => Some("GetTexEnvfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 131) => Some("GetTexEnviv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 132) => Some("GetTexGendv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 133) => Some("GetTexGenfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 134) => Some("GetTexGeniv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 135) => Some("GetTexImage"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 136) => Some("GetTexParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 137) => Some("GetTexParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 138) => Some("GetTexLevelParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 139) => Some("GetTexLevelParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 140) => Some("IsEnabled"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 141) => Some("IsList"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 142) => Some("Flush"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 143) => Some("AreTexturesResident"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 144) => Some("DeleteTextures"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 145) => Some("GenTextures"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 146) => Some("IsTexture"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 147) => Some("GetColorTable"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 148) => Some("GetColorTableParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 149) => Some("GetColorTableParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 150) => Some("GetConvolutionFilter"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 151) => Some("GetConvolutionParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 152) => Some("GetConvolutionParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 153) => Some("GetSeparableFilter"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 154) => Some("GetHistogram"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 155) => Some("GetHistogramParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 156) => Some("GetHistogramParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 157) => Some("GetMinmax"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 158) => Some("GetMinmaxParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 159) => Some("GetMinmaxParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 160) => Some("GetCompressedTexImageARB"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 161) => Some("DeleteQueriesARB"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 162) => Some("GenQueriesARB"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 163) => Some("IsQueryARB"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 164) => Some("GetQueryivARB"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 165) => Some("GetQueryObjectivARB"),
        #[cfg(feature = "glx")]
        (Some(glx::X11_EXTENSION_NAME), 166) => Some("GetQueryObjectuivARB"),
        #[cfg(feature = "present")]
        (Some(present::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "present")]
        (Some(present::X11_EXTENSION_NAME), 1) => Some("Pixmap"),
        #[cfg(feature = "present")]
        (Some(present::X11_EXTENSION_NAME), 2) => Some("NotifyMSC"),
        #[cfg(feature = "present")]
        (Some(present::X11_EXTENSION_NAME), 3) => Some("SelectInput"),
        #[cfg(feature = "present")]
        (Some(present::X11_EXTENSION_NAME), 4) => Some("QueryCapabilities"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 2) => Some("SetScreenConfig"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 4) => Some("SelectInput"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 5) => Some("GetScreenInfo"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 6) => Some("GetScreenSizeRange"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 7) => Some("SetScreenSize"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 8) => Some("GetScreenResources"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 9) => Some("GetOutputInfo"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 10) => Some("ListOutputProperties"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 11) => Some("QueryOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 12) => Some("ConfigureOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 13) => Some("ChangeOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 14) => Some("DeleteOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 15) => Some("GetOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 16) => Some("CreateMode"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 17) => Some("DestroyMode"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 18) => Some("AddOutputMode"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 19) => Some("DeleteOutputMode"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 20) => Some("GetCrtcInfo"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 21) => Some("SetCrtcConfig"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 22) => Some("GetCrtcGammaSize"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 23) => Some("GetCrtcGamma"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 24) => Some("SetCrtcGamma"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 25) => Some("GetScreenResourcesCurrent"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 26) => Some("SetCrtcTransform"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 27) => Some("GetCrtcTransform"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 28) => Some("GetPanning"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 29) => Some("SetPanning"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 30) => Some("SetOutputPrimary"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 31) => Some("GetOutputPrimary"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 32) => Some("GetProviders"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 33) => Some("GetProviderInfo"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 34) => Some("SetProviderOffloadSink"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 35) => Some("SetProviderOutputSource"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 36) => Some("ListProviderProperties"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 37) => Some("QueryProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 38) => Some("ConfigureProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 39) => Some("ChangeProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 40) => Some("DeleteProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 41) => Some("GetProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 42) => Some("GetMonitors"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 43) => Some("SetMonitor"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 44) => Some("DeleteMonitor"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 45) => Some("CreateLease"),
        #[cfg(feature = "randr")]
        (Some(randr::X11_EXTENSION_NAME), 46) => Some("FreeLease"),
        #[cfg(feature = "record")]
        (Some(record::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "record")]
        (Some(record::X11_EXTENSION_NAME), 1) => Some("CreateContext"),
        #[cfg(feature = "record")]
        (Some(record::X11_EXTENSION_NAME), 2) => Some("RegisterClients"),
        #[cfg(feature = "record")]
        (Some(record::X11_EXTENSION_NAME), 3) => Some("UnregisterClients"),
        #[cfg(feature = "record")]
        (Some(record::X11_EXTENSION_NAME), 4) => Some("GetContext"),
        #[cfg(feature = "record")]
        (Some(record::X11_EXTENSION_NAME), 5) => Some("EnableContext"),
        #[cfg(feature = "record")]
        (Some(record::X11_EXTENSION_NAME), 6) => Some("DisableContext"),
        #[cfg(feature = "record")]
        (Some(record::X11_EXTENSION_NAME), 7) => Some("FreeContext"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 1) => Some("QueryPictFormats"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 2) => Some("QueryPictIndexValues"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 4) => Some("CreatePicture"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 5) => Some("ChangePicture"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 6) => Some("SetPictureClipRectangles"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 7) => Some("FreePicture"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 8) => Some("Composite"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 10) => Some("Trapezoids"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 11) => Some("Triangles"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 12) => Some("TriStrip"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 13) => Some("TriFan"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 17) => Some("CreateGlyphSet"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 18) => Some("ReferenceGlyphSet"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 19) => Some("FreeGlyphSet"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 20) => Some("AddGlyphs"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 22) => Some("FreeGlyphs"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 23) => Some("CompositeGlyphs8"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 24) => Some("CompositeGlyphs16"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 25) => Some("CompositeGlyphs32"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 26) => Some("FillRectangles"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 27) => Some("CreateCursor"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 28) => Some("SetPictureTransform"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 29) => Some("QueryFilters"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 30) => Some("SetPictureFilter"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 31) => Some("CreateAnimCursor"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 32) => Some("AddTraps"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 33) => Some("CreateSolidFill"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 34) => Some("CreateLinearGradient"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 35) => Some("CreateRadialGradient"),
        #[cfg(feature = "render")]
        (Some(render::X11_EXTENSION_NAME), 36) => Some("CreateConicalGradient"),
        #[cfg(feature = "res")]
        (Some(res::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "res")]
        (Some(res::X11_EXTENSION_NAME), 1) => Some("QueryClients"),
        #[cfg(feature = "res")]
        (Some(res::X11_EXTENSION_NAME), 2) => Some("QueryClientResources"),
        #[cfg(feature = "res")]
        (Some(res::X11_EXTENSION_NAME), 3) => Some("QueryClientPixmapBytes"),
        #[cfg(feature = "res")]
        (Some(res::X11_EXTENSION_NAME), 4) => Some("QueryClientIds"),
        #[cfg(feature = "res")]
        (Some(res::X11_EXTENSION_NAME), 5) => Some("QueryResourceBytes"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::X11_EXTENSION_NAME), 1) => Some("QueryInfo"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::X11_EXTENSION_NAME), 2) => Some("SelectInput"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::X11_EXTENSION_NAME), 3) => Some("SetAttributes"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::X11_EXTENSION_NAME), 4) => Some("UnsetAttributes"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::X11_EXTENSION_NAME), 5) => Some("Suspend"),
        #[cfg(feature = "shape")]
        (Some(shape::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "shape")]
        (Some(shape::X11_EXTENSION_NAME), 1) => Some("Rectangles"),
        #[cfg(feature = "shape")]
        (Some(shape::X11_EXTENSION_NAME), 2) => Some("Mask"),
        #[cfg(feature = "shape")]
        (Some(shape::X11_EXTENSION_NAME), 3) => Some("Combine"),
        #[cfg(feature = "shape")]
        (Some(shape::X11_EXTENSION_NAME), 4) => Some("Offset"),
        #[cfg(feature = "shape")]
        (Some(shape::X11_EXTENSION_NAME), 5) => Some("QueryExtents"),
        #[cfg(feature = "shape")]
        (Some(shape::X11_EXTENSION_NAME), 6) => Some("SelectInput"),
        #[cfg(feature = "shape")]
        (Some(shape::X11_EXTENSION_NAME), 7) => Some("InputSelected"),
        #[cfg(feature = "shape")]
        (Some(shape::X11_EXTENSION_NAME), 8) => Some("GetRectangles"),
        #[cfg(feature = "shm")]
        (Some(shm::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "shm")]
        (Some(shm::X11_EXTENSION_NAME), 1) => Some("Attach"),
        #[cfg(feature = "shm")]
        (Some(shm::X11_EXTENSION_NAME), 2) => Some("Detach"),
        #[cfg(feature = "shm")]
        (Some(shm::X11_EXTENSION_NAME), 3) => Some("PutImage"),
        #[cfg(feature = "shm")]
        (Some(shm::X11_EXTENSION_NAME), 4) => Some("GetImage"),
        #[cfg(feature = "shm")]
        (Some(shm::X11_EXTENSION_NAME), 5) => Some("CreatePixmap"),
        #[cfg(feature = "shm")]
        (Some(shm::X11_EXTENSION_NAME), 6) => Some("AttachFd"),
        #[cfg(feature = "shm")]
        (Some(shm::X11_EXTENSION_NAME), 7) => Some("CreateSegment"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 0) => Some("Initialize"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 1) => Some("ListSystemCounters"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 2) => Some("CreateCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 3) => Some("SetCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 4) => Some("ChangeCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 5) => Some("QueryCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 6) => Some("DestroyCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 7) => Some("Await"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 8) => Some("CreateAlarm"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 9) => Some("ChangeAlarm"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 10) => Some("QueryAlarm"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 11) => Some("DestroyAlarm"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 12) => Some("SetPriority"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 13) => Some("GetPriority"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 14) => Some("CreateFence"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 15) => Some("TriggerFence"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 16) => Some("ResetFence"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 17) => Some("DestroyFence"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 18) => Some("QueryFence"),
        #[cfg(feature = "sync")]
        (Some(sync::X11_EXTENSION_NAME), 19) => Some("AwaitFence"),
        (Some(xc_misc::X11_EXTENSION_NAME), 0) => Some("GetVersion"),
        (Some(xc_misc::X11_EXTENSION_NAME), 1) => Some("GetXIDRange"),
        (Some(xc_misc::X11_EXTENSION_NAME), 2) => Some("GetXIDList"),
        #[cfg(feature = "xevie")]
        (Some(xevie::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xevie")]
        (Some(xevie::X11_EXTENSION_NAME), 1) => Some("Start"),
        #[cfg(feature = "xevie")]
        (Some(xevie::X11_EXTENSION_NAME), 2) => Some("End"),
        #[cfg(feature = "xevie")]
        (Some(xevie::X11_EXTENSION_NAME), 3) => Some("Send"),
        #[cfg(feature = "xevie")]
        (Some(xevie::X11_EXTENSION_NAME), 4) => Some("SelectInput"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 1) => Some("QueryDirectRenderingCapable"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 2) => Some("OpenConnection"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 3) => Some("CloseConnection"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 4) => Some("GetClientDriverName"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 5) => Some("CreateContext"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 6) => Some("DestroyContext"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 7) => Some("CreateDrawable"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 8) => Some("DestroyDrawable"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 9) => Some("GetDrawableInfo"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 10) => Some("GetDeviceInfo"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::X11_EXTENSION_NAME), 11) => Some("AuthConnection"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 1) => Some("GetModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 2) => Some("ModModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 3) => Some("SwitchMode"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 4) => Some("GetMonitor"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 5) => Some("LockModeSwitch"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 6) => Some("GetAllModeLines"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 7) => Some("AddModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 8) => Some("DeleteModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 9) => Some("ValidateModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 10) => Some("SwitchToMode"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 11) => Some("GetViewPort"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 12) => Some("SetViewPort"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 13) => Some("GetDotClocks"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 14) => Some("SetClientVersion"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 15) => Some("SetGamma"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 16) => Some("GetGamma"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 17) => Some("GetGammaRamp"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 18) => Some("SetGammaRamp"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 19) => Some("GetGammaRampSize"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::X11_EXTENSION_NAME), 20) => Some("GetPermissions"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 1) => Some("ChangeSaveSet"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 2) => Some("SelectSelectionInput"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 3) => Some("SelectCursorInput"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 4) => Some("GetCursorImage"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 5) => Some("CreateRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 6) => Some("CreateRegionFromBitmap"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 7) => Some("CreateRegionFromWindow"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 8) => Some("CreateRegionFromGC"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 9) => Some("CreateRegionFromPicture"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 10) => Some("DestroyRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 11) => Some("SetRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 12) => Some("CopyRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 13) => Some("UnionRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 14) => Some("IntersectRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 15) => Some("SubtractRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 16) => Some("InvertRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 17) => Some("TranslateRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 18) => Some("RegionExtents"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 19) => Some("FetchRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 20) => Some("SetGCClipRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 21) => Some("SetWindowShapeRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 22) => Some("SetPictureClipRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 23) => Some("SetCursorName"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 24) => Some("GetCursorName"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 25) => Some("GetCursorImageAndName"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 26) => Some("ChangeCursor"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 27) => Some("ChangeCursorByName"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 28) => Some("ExpandRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 29) => Some("HideCursor"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 30) => Some("ShowCursor"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 31) => Some("CreatePointerBarrier"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::X11_EXTENSION_NAME), 32) => Some("DeletePointerBarrier"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::X11_EXTENSION_NAME), 1) => Some("GetState"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::X11_EXTENSION_NAME), 2) => Some("GetScreenCount"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::X11_EXTENSION_NAME), 3) => Some("GetScreenSize"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::X11_EXTENSION_NAME), 4) => Some("IsActive"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::X11_EXTENSION_NAME), 5) => Some("QueryScreens"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 1) => Some("GetExtensionVersion"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 2) => Some("ListInputDevices"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 3) => Some("OpenDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 4) => Some("CloseDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 5) => Some("SetDeviceMode"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 6) => Some("SelectExtensionEvent"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 7) => Some("GetSelectedExtensionEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 8) => Some("ChangeDeviceDontPropagateList"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 9) => Some("GetDeviceDontPropagateList"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 10) => Some("GetDeviceMotionEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 11) => Some("ChangeKeyboardDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 12) => Some("ChangePointerDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 13) => Some("GrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 14) => Some("UngrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 15) => Some("GrabDeviceKey"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 16) => Some("UngrabDeviceKey"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 17) => Some("GrabDeviceButton"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 18) => Some("UngrabDeviceButton"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 19) => Some("AllowDeviceEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 20) => Some("GetDeviceFocus"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 21) => Some("SetDeviceFocus"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 22) => Some("GetFeedbackControl"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 23) => Some("ChangeFeedbackControl"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 24) => Some("GetDeviceKeyMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 25) => Some("ChangeDeviceKeyMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 26) => Some("GetDeviceModifierMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 27) => Some("SetDeviceModifierMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 28) => Some("GetDeviceButtonMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 29) => Some("SetDeviceButtonMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 30) => Some("QueryDeviceState"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 31) => Some("SendExtensionEvent"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 32) => Some("DeviceBell"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 33) => Some("SetDeviceValuators"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 34) => Some("GetDeviceControl"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 35) => Some("ChangeDeviceControl"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 36) => Some("ListDeviceProperties"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 37) => Some("ChangeDeviceProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 38) => Some("DeleteDeviceProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 39) => Some("GetDeviceProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 40) => Some("XIQueryPointer"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 41) => Some("XIWarpPointer"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 42) => Some("XIChangeCursor"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 43) => Some("XIChangeHierarchy"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 44) => Some("XISetClientPointer"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 45) => Some("XIGetClientPointer"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 46) => Some("XISelectEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 47) => Some("XIQueryVersion"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 48) => Some("XIQueryDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 49) => Some("XISetFocus"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 50) => Some("XIGetFocus"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 51) => Some("XIGrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 52) => Some("XIUngrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 53) => Some("XIAllowEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 54) => Some("XIPassiveGrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 55) => Some("XIPassiveUngrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 56) => Some("XIListProperties"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 57) => Some("XIChangeProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 58) => Some("XIDeleteProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 59) => Some("XIGetProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 60) => Some("XIGetSelectedEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::X11_EXTENSION_NAME), 61) => Some("XIBarrierReleasePointer"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 0) => Some("UseExtension"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 1) => Some("SelectEvents"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 3) => Some("Bell"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 4) => Some("GetState"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 5) => Some("LatchLockState"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 6) => Some("GetControls"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 7) => Some("SetControls"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 8) => Some("GetMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 9) => Some("SetMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 10) => Some("GetCompatMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 11) => Some("SetCompatMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 12) => Some("GetIndicatorState"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 13) => Some("GetIndicatorMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 14) => Some("SetIndicatorMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 15) => Some("GetNamedIndicator"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 16) => Some("SetNamedIndicator"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 17) => Some("GetNames"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 18) => Some("SetNames"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 21) => Some("PerClientFlags"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 22) => Some("ListComponents"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 23) => Some("GetKbdByName"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 24) => Some("GetDeviceInfo"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 25) => Some("SetDeviceInfo"),
        #[cfg(feature = "xkb")]
        (Some(xkb::X11_EXTENSION_NAME), 101) => Some("SetDebuggingFlags"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 0) => Some("PrintQueryVersion"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 1) => Some("PrintGetPrinterList"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 2) => Some("CreateContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 3) => Some("PrintSetContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 4) => Some("PrintGetContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 5) => Some("PrintDestroyContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 6) => Some("PrintGetScreenOfContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 7) => Some("PrintStartJob"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 8) => Some("PrintEndJob"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 9) => Some("PrintStartDoc"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 10) => Some("PrintEndDoc"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 11) => Some("PrintPutDocumentData"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 12) => Some("PrintGetDocumentData"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 13) => Some("PrintStartPage"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 14) => Some("PrintEndPage"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 15) => Some("PrintSelectInput"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 16) => Some("PrintInputSelected"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 17) => Some("PrintGetAttributes"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 18) => Some("PrintSetAttributes"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 19) => Some("PrintGetOneAttributes"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 20) => Some("PrintRehashPrinterList"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 21) => Some("PrintGetPageDimensions"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 22) => Some("PrintQueryScreens"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 23) => Some("PrintSetImageResolution"),
        #[cfg(feature = "xprint")]
        (Some(xprint::X11_EXTENSION_NAME), 24) => Some("PrintGetImageResolution"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 1) => Some("SetDeviceCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 2) => Some("GetDeviceCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 3) => Some("SetDeviceContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 4) => Some("GetDeviceContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 5) => Some("SetWindowCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 6) => Some("GetWindowCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 7) => Some("GetWindowContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 8) => Some("SetPropertyCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 9) => Some("GetPropertyCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 10) => Some("SetPropertyUseContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 11) => Some("GetPropertyUseContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 12) => Some("GetPropertyContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 13) => Some("GetPropertyDataContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 14) => Some("ListProperties"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 15) => Some("SetSelectionCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 16) => Some("GetSelectionCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 17) => Some("SetSelectionUseContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 18) => Some("GetSelectionUseContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 19) => Some("GetSelectionContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 20) => Some("GetSelectionDataContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 21) => Some("ListSelections"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::X11_EXTENSION_NAME), 22) => Some("GetClientContext"),
        #[cfg(feature = "xtest")]
        (Some(xtest::X11_EXTENSION_NAME), 0) => Some("GetVersion"),
        #[cfg(feature = "xtest")]
        (Some(xtest::X11_EXTENSION_NAME), 1) => Some("CompareCursor"),
        #[cfg(feature = "xtest")]
        (Some(xtest::X11_EXTENSION_NAME), 2) => Some("FakeInput"),
        #[cfg(feature = "xtest")]
        (Some(xtest::X11_EXTENSION_NAME), 3) => Some("GrabControl"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 0) => Some("QueryExtension"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 1) => Some("QueryAdaptors"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 2) => Some("QueryEncodings"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 3) => Some("GrabPort"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 4) => Some("UngrabPort"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 5) => Some("PutVideo"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 6) => Some("PutStill"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 7) => Some("GetVideo"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 8) => Some("GetStill"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 9) => Some("StopVideo"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 10) => Some("SelectVideoNotify"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 11) => Some("SelectPortNotify"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 12) => Some("QueryBestSize"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 13) => Some("SetPortAttribute"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 14) => Some("GetPortAttribute"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 15) => Some("QueryPortAttributes"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 16) => Some("ListImageFormats"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 17) => Some("QueryImageAttributes"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 18) => Some("PutImage"),
        #[cfg(feature = "xv")]
        (Some(xv::X11_EXTENSION_NAME), 19) => Some("ShmPutImage"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::X11_EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::X11_EXTENSION_NAME), 1) => Some("ListSurfaceTypes"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::X11_EXTENSION_NAME), 2) => Some("CreateContext"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::X11_EXTENSION_NAME), 3) => Some("DestroyContext"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::X11_EXTENSION_NAME), 4) => Some("CreateSurface"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::X11_EXTENSION_NAME), 5) => Some("DestroySurface"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::X11_EXTENSION_NAME), 6) => Some("CreateSubpicture"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::X11_EXTENSION_NAME), 7) => Some("DestroySubpicture"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::X11_EXTENSION_NAME), 8) => Some("ListSubpictureTypes"),
        _ => None,
    }
}

/// Enumeration of all possible X11 error kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    Unknown(u8),
    Access,
    Alloc,
    Atom,
    Colormap,
    Cursor,
    Drawable,
    Font,
    GContext,
    IDChoice,
    Implementation,
    Length,
    Match,
    Name,
    Pixmap,
    Request,
    Value,
    Window,
    #[cfg(feature = "damage")]
    DamageBadDamage,
    #[cfg(feature = "glx")]
    GlxBadContext,
    #[cfg(feature = "glx")]
    GlxBadContextState,
    #[cfg(feature = "glx")]
    GlxBadContextTag,
    #[cfg(feature = "glx")]
    GlxBadCurrentDrawable,
    #[cfg(feature = "glx")]
    GlxBadCurrentWindow,
    #[cfg(feature = "glx")]
    GlxBadDrawable,
    #[cfg(feature = "glx")]
    GlxBadFBConfig,
    #[cfg(feature = "glx")]
    GlxBadLargeRequest,
    #[cfg(feature = "glx")]
    GlxBadPbuffer,
    #[cfg(feature = "glx")]
    GlxBadPixmap,
    #[cfg(feature = "glx")]
    GlxBadRenderRequest,
    #[cfg(feature = "glx")]
    GlxBadWindow,
    #[cfg(feature = "glx")]
    GlxGLXBadProfileARB,
    #[cfg(feature = "glx")]
    GlxUnsupportedPrivateRequest,
    #[cfg(feature = "randr")]
    RandrBadCrtc,
    #[cfg(feature = "randr")]
    RandrBadMode,
    #[cfg(feature = "randr")]
    RandrBadOutput,
    #[cfg(feature = "randr")]
    RandrBadProvider,
    #[cfg(feature = "record")]
    RecordBadContext,
    #[cfg(feature = "render")]
    RenderGlyph,
    #[cfg(feature = "render")]
    RenderGlyphSet,
    #[cfg(feature = "render")]
    RenderPictFormat,
    #[cfg(feature = "render")]
    RenderPictOp,
    #[cfg(feature = "render")]
    RenderPicture,
    #[cfg(feature = "shm")]
    ShmBadSeg,
    #[cfg(feature = "sync")]
    SyncAlarm,
    #[cfg(feature = "sync")]
    SyncCounter,
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadClock,
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadHTimings,
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadVTimings,
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeClientNotLocal,
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeExtensionDisabled,
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeModeUnsuitable,
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeZoomLocked,
    #[cfg(feature = "xfixes")]
    XfixesBadRegion,
    #[cfg(feature = "xinput")]
    XinputClass,
    #[cfg(feature = "xinput")]
    XinputDevice,
    #[cfg(feature = "xinput")]
    XinputDeviceBusy,
    #[cfg(feature = "xinput")]
    XinputEvent,
    #[cfg(feature = "xinput")]
    XinputMode,
    #[cfg(feature = "xkb")]
    XkbKeyboard,
    #[cfg(feature = "xprint")]
    XprintBadContext,
    #[cfg(feature = "xprint")]
    XprintBadSequence,
    #[cfg(feature = "xv")]
    XvBadControl,
    #[cfg(feature = "xv")]
    XvBadEncoding,
    #[cfg(feature = "xv")]
    XvBadPort,
}

impl ErrorKind {
    #[allow(clippy::match_single_binding)]
    #[must_use]
    pub fn from_wire_error_code(error_code: u8, ext_info_provider: &ExtensionInfoProvider) -> Self {
        // Check if this is a core protocol error
        match error_code {
            xproto::ACCESS_ERROR => return Self::Access,
            xproto::ALLOC_ERROR => return Self::Alloc,
            xproto::ATOM_ERROR => return Self::Atom,
            xproto::COLORMAP_ERROR => return Self::Colormap,
            xproto::CURSOR_ERROR => return Self::Cursor,
            xproto::DRAWABLE_ERROR => return Self::Drawable,
            xproto::FONT_ERROR => return Self::Font,
            xproto::G_CONTEXT_ERROR => return Self::GContext,
            xproto::ID_CHOICE_ERROR => return Self::IDChoice,
            xproto::IMPLEMENTATION_ERROR => return Self::Implementation,
            xproto::LENGTH_ERROR => return Self::Length,
            xproto::MATCH_ERROR => return Self::Match,
            xproto::NAME_ERROR => return Self::Name,
            xproto::PIXMAP_ERROR => return Self::Pixmap,
            xproto::REQUEST_ERROR => return Self::Request,
            xproto::VALUE_ERROR => return Self::Value,
            xproto::WINDOW_ERROR => return Self::Window,
            _ => {}
        }

        // Find the extension that this error could belong to
        let ext_info = ext_info_provider.get_from_error_code(error_code);
        match ext_info {
            #[cfg(feature = "damage")]
            Some((damage::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    damage::BAD_DAMAGE_ERROR => Self::DamageBadDamage,
                    _ => Self::Unknown(error_code),
                }
            }
            #[cfg(feature = "glx")]
            Some((glx::X11_EXTENSION_NAME, ext_info)) => match error_code - ext_info.first_error {
                glx::BAD_CONTEXT_ERROR => Self::GlxBadContext,
                glx::BAD_CONTEXT_STATE_ERROR => Self::GlxBadContextState,
                glx::BAD_CONTEXT_TAG_ERROR => Self::GlxBadContextTag,
                glx::BAD_CURRENT_DRAWABLE_ERROR => Self::GlxBadCurrentDrawable,
                glx::BAD_CURRENT_WINDOW_ERROR => Self::GlxBadCurrentWindow,
                glx::BAD_DRAWABLE_ERROR => Self::GlxBadDrawable,
                glx::BAD_FB_CONFIG_ERROR => Self::GlxBadFBConfig,
                glx::BAD_LARGE_REQUEST_ERROR => Self::GlxBadLargeRequest,
                glx::BAD_PBUFFER_ERROR => Self::GlxBadPbuffer,
                glx::BAD_PIXMAP_ERROR => Self::GlxBadPixmap,
                glx::BAD_RENDER_REQUEST_ERROR => Self::GlxBadRenderRequest,
                glx::BAD_WINDOW_ERROR => Self::GlxBadWindow,
                glx::GLX_BAD_PROFILE_ARB_ERROR => Self::GlxGLXBadProfileARB,
                glx::UNSUPPORTED_PRIVATE_REQUEST_ERROR => Self::GlxUnsupportedPrivateRequest,
                _ => Self::Unknown(error_code),
            },
            #[cfg(feature = "randr")]
            Some((randr::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    randr::BAD_CRTC_ERROR => Self::RandrBadCrtc,
                    randr::BAD_MODE_ERROR => Self::RandrBadMode,
                    randr::BAD_OUTPUT_ERROR => Self::RandrBadOutput,
                    randr::BAD_PROVIDER_ERROR => Self::RandrBadProvider,
                    _ => Self::Unknown(error_code),
                }
            }
            #[cfg(feature = "record")]
            Some((record::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    record::BAD_CONTEXT_ERROR => Self::RecordBadContext,
                    _ => Self::Unknown(error_code),
                }
            }
            #[cfg(feature = "render")]
            Some((render::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    render::GLYPH_ERROR => Self::RenderGlyph,
                    render::GLYPH_SET_ERROR => Self::RenderGlyphSet,
                    render::PICT_FORMAT_ERROR => Self::RenderPictFormat,
                    render::PICT_OP_ERROR => Self::RenderPictOp,
                    render::PICTURE_ERROR => Self::RenderPicture,
                    _ => Self::Unknown(error_code),
                }
            }
            #[cfg(feature = "shm")]
            Some((shm::X11_EXTENSION_NAME, ext_info)) => match error_code - ext_info.first_error {
                shm::BAD_SEG_ERROR => Self::ShmBadSeg,
                _ => Self::Unknown(error_code),
            },
            #[cfg(feature = "sync")]
            Some((sync::X11_EXTENSION_NAME, ext_info)) => match error_code - ext_info.first_error {
                sync::ALARM_ERROR => Self::SyncAlarm,
                sync::COUNTER_ERROR => Self::SyncCounter,
                _ => Self::Unknown(error_code),
            },
            #[cfg(feature = "xf86vidmode")]
            Some((xf86vidmode::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xf86vidmode::BAD_CLOCK_ERROR => Self::Xf86vidmodeBadClock,
                    xf86vidmode::BAD_H_TIMINGS_ERROR => Self::Xf86vidmodeBadHTimings,
                    xf86vidmode::BAD_V_TIMINGS_ERROR => Self::Xf86vidmodeBadVTimings,
                    xf86vidmode::CLIENT_NOT_LOCAL_ERROR => Self::Xf86vidmodeClientNotLocal,
                    xf86vidmode::EXTENSION_DISABLED_ERROR => Self::Xf86vidmodeExtensionDisabled,
                    xf86vidmode::MODE_UNSUITABLE_ERROR => Self::Xf86vidmodeModeUnsuitable,
                    xf86vidmode::ZOOM_LOCKED_ERROR => Self::Xf86vidmodeZoomLocked,
                    _ => Self::Unknown(error_code),
                }
            }
            #[cfg(feature = "xfixes")]
            Some((xfixes::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xfixes::BAD_REGION_ERROR => Self::XfixesBadRegion,
                    _ => Self::Unknown(error_code),
                }
            }
            #[cfg(feature = "xinput")]
            Some((xinput::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xinput::CLASS_ERROR => Self::XinputClass,
                    xinput::DEVICE_ERROR => Self::XinputDevice,
                    xinput::DEVICE_BUSY_ERROR => Self::XinputDeviceBusy,
                    xinput::EVENT_ERROR => Self::XinputEvent,
                    xinput::MODE_ERROR => Self::XinputMode,
                    _ => Self::Unknown(error_code),
                }
            }
            #[cfg(feature = "xkb")]
            Some((xkb::X11_EXTENSION_NAME, ext_info)) => match error_code - ext_info.first_error {
                xkb::KEYBOARD_ERROR => Self::XkbKeyboard,
                _ => Self::Unknown(error_code),
            },
            #[cfg(feature = "xprint")]
            Some((xprint::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xprint::BAD_CONTEXT_ERROR => Self::XprintBadContext,
                    xprint::BAD_SEQUENCE_ERROR => Self::XprintBadSequence,
                    _ => Self::Unknown(error_code),
                }
            }
            #[cfg(feature = "xv")]
            Some((xv::X11_EXTENSION_NAME, ext_info)) => match error_code - ext_info.first_error {
                xv::BAD_CONTROL_ERROR => Self::XvBadControl,
                xv::BAD_ENCODING_ERROR => Self::XvBadEncoding,
                xv::BAD_PORT_ERROR => Self::XvBadPort,
                _ => Self::Unknown(error_code),
            },
            _ => Self::Unknown(error_code),
        }
    }
}

/// Enumeration of all possible X11 events.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    Unknown(Vec<u8>),
    Error(X11Error),
    ButtonPress(xproto::ButtonPressEvent),
    ButtonRelease(xproto::ButtonReleaseEvent),
    CirculateNotify(xproto::CirculateNotifyEvent),
    CirculateRequest(xproto::CirculateRequestEvent),
    ClientMessage(xproto::ClientMessageEvent),
    ColormapNotify(xproto::ColormapNotifyEvent),
    ConfigureNotify(xproto::ConfigureNotifyEvent),
    ConfigureRequest(xproto::ConfigureRequestEvent),
    CreateNotify(xproto::CreateNotifyEvent),
    DestroyNotify(xproto::DestroyNotifyEvent),
    EnterNotify(xproto::EnterNotifyEvent),
    Expose(xproto::ExposeEvent),
    FocusIn(xproto::FocusInEvent),
    FocusOut(xproto::FocusOutEvent),
    GeGeneric(xproto::GeGenericEvent),
    GraphicsExposure(xproto::GraphicsExposureEvent),
    GravityNotify(xproto::GravityNotifyEvent),
    KeyPress(xproto::KeyPressEvent),
    KeyRelease(xproto::KeyReleaseEvent),
    KeymapNotify(xproto::KeymapNotifyEvent),
    LeaveNotify(xproto::LeaveNotifyEvent),
    MapNotify(xproto::MapNotifyEvent),
    MapRequest(xproto::MapRequestEvent),
    MappingNotify(xproto::MappingNotifyEvent),
    MotionNotify(xproto::MotionNotifyEvent),
    NoExposure(xproto::NoExposureEvent),
    PropertyNotify(xproto::PropertyNotifyEvent),
    ReparentNotify(xproto::ReparentNotifyEvent),
    ResizeRequest(xproto::ResizeRequestEvent),
    SelectionClear(xproto::SelectionClearEvent),
    SelectionNotify(xproto::SelectionNotifyEvent),
    SelectionRequest(xproto::SelectionRequestEvent),
    UnmapNotify(xproto::UnmapNotifyEvent),
    VisibilityNotify(xproto::VisibilityNotifyEvent),
    #[cfg(feature = "damage")]
    DamageNotify(damage::NotifyEvent),
    #[cfg(feature = "dri2")]
    Dri2BufferSwapComplete(dri2::BufferSwapCompleteEvent),
    #[cfg(feature = "dri2")]
    Dri2InvalidateBuffers(dri2::InvalidateBuffersEvent),
    #[cfg(feature = "glx")]
    GlxBufferSwapComplete(glx::BufferSwapCompleteEvent),
    #[cfg(feature = "glx")]
    GlxPbufferClobber(glx::PbufferClobberEvent),
    #[cfg(feature = "present")]
    PresentCompleteNotify(present::CompleteNotifyEvent),
    #[cfg(feature = "present")]
    PresentConfigureNotify(present::ConfigureNotifyEvent),
    #[cfg(feature = "present")]
    PresentGeneric(present::GenericEvent),
    #[cfg(feature = "present")]
    PresentIdleNotify(present::IdleNotifyEvent),
    #[cfg(feature = "present")]
    PresentRedirectNotify(present::RedirectNotifyEvent),
    #[cfg(feature = "randr")]
    RandrNotify(randr::NotifyEvent),
    #[cfg(feature = "randr")]
    RandrScreenChangeNotify(randr::ScreenChangeNotifyEvent),
    #[cfg(feature = "screensaver")]
    ScreensaverNotify(screensaver::NotifyEvent),
    #[cfg(feature = "shape")]
    ShapeNotify(shape::NotifyEvent),
    #[cfg(feature = "shm")]
    ShmCompletion(shm::CompletionEvent),
    #[cfg(feature = "sync")]
    SyncAlarmNotify(sync::AlarmNotifyEvent),
    #[cfg(feature = "sync")]
    SyncCounterNotify(sync::CounterNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesCursorNotify(xfixes::CursorNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesSelectionNotify(xfixes::SelectionNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputBarrierHit(xinput::BarrierHitEvent),
    #[cfg(feature = "xinput")]
    XinputBarrierLeave(xinput::BarrierLeaveEvent),
    #[cfg(feature = "xinput")]
    XinputButtonPress(xinput::ButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputButtonRelease(xinput::ButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceNotify(xinput::ChangeDeviceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonPress(xinput::DeviceButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonRelease(xinput::DeviceButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonStateNotify(xinput::DeviceButtonStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceChanged(xinput::DeviceChangedEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusIn(xinput::DeviceFocusInEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusOut(xinput::DeviceFocusOutEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyPress(xinput::DeviceKeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyRelease(xinput::DeviceKeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyStateNotify(xinput::DeviceKeyStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMappingNotify(xinput::DeviceMappingNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMotionNotify(xinput::DeviceMotionNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePresenceNotify(xinput::DevicePresenceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePropertyNotify(xinput::DevicePropertyNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceStateNotify(xinput::DeviceStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceValuator(xinput::DeviceValuatorEvent),
    #[cfg(feature = "xinput")]
    XinputEnter(xinput::EnterEvent),
    #[cfg(feature = "xinput")]
    XinputFocusIn(xinput::FocusInEvent),
    #[cfg(feature = "xinput")]
    XinputFocusOut(xinput::FocusOutEvent),
    #[cfg(feature = "xinput")]
    XinputHierarchy(xinput::HierarchyEvent),
    #[cfg(feature = "xinput")]
    XinputKeyPress(xinput::KeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputKeyRelease(xinput::KeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputLeave(xinput::LeaveEvent),
    #[cfg(feature = "xinput")]
    XinputMotion(xinput::MotionEvent),
    #[cfg(feature = "xinput")]
    XinputProperty(xinput::PropertyEvent),
    #[cfg(feature = "xinput")]
    XinputProximityIn(xinput::ProximityInEvent),
    #[cfg(feature = "xinput")]
    XinputProximityOut(xinput::ProximityOutEvent),
    #[cfg(feature = "xinput")]
    XinputRawButtonPress(xinput::RawButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputRawButtonRelease(xinput::RawButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputRawKeyPress(xinput::RawKeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputRawKeyRelease(xinput::RawKeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputRawMotion(xinput::RawMotionEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchBegin(xinput::RawTouchBeginEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchEnd(xinput::RawTouchEndEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchUpdate(xinput::RawTouchUpdateEvent),
    #[cfg(feature = "xinput")]
    XinputTouchBegin(xinput::TouchBeginEvent),
    #[cfg(feature = "xinput")]
    XinputTouchEnd(xinput::TouchEndEvent),
    #[cfg(feature = "xinput")]
    XinputTouchOwnership(xinput::TouchOwnershipEvent),
    #[cfg(feature = "xinput")]
    XinputTouchUpdate(xinput::TouchUpdateEvent),
    #[cfg(feature = "xkb")]
    XkbAccessXNotify(xkb::AccessXNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbActionMessage(xkb::ActionMessageEvent),
    #[cfg(feature = "xkb")]
    XkbBellNotify(xkb::BellNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbCompatMapNotify(xkb::CompatMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbControlsNotify(xkb::ControlsNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbExtensionDeviceNotify(xkb::ExtensionDeviceNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorMapNotify(xkb::IndicatorMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorStateNotify(xkb::IndicatorStateNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbMapNotify(xkb::MapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNamesNotify(xkb::NamesNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNewKeyboardNotify(xkb::NewKeyboardNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbStateNotify(xkb::StateNotifyEvent),
    #[cfg(feature = "xprint")]
    XprintAttributNotify(xprint::AttributNotifyEvent),
    #[cfg(feature = "xprint")]
    XprintNotify(xprint::NotifyEvent),
    #[cfg(feature = "xv")]
    XvPortNotify(xv::PortNotifyEvent),
    #[cfg(feature = "xv")]
    XvVideoNotify(xv::VideoNotifyEvent),
}

impl Event {
    /// Parse a generic X11 event into a concrete event type.
    #[allow(clippy::cognitive_complexity, clippy::match_single_binding)]
    pub fn parse(
        event: &[u8],
        ext_info_provider: &ExtensionInfoProvider,
    ) -> Result<Self, ParseError> {
        let event_code = response_type(event)?;

        // Check if this is a core protocol event or error, or from the generic event extension
        match event_code {
            0 => return Ok(Self::Error(X11Error::try_parse(event, ext_info_provider)?)),
            xproto::BUTTON_PRESS_EVENT => {
                return Ok(Self::ButtonPress(TryParse::try_parse(event)?.0))
            }
            xproto::BUTTON_RELEASE_EVENT => {
                return Ok(Self::ButtonRelease(TryParse::try_parse(event)?.0))
            }
            xproto::CIRCULATE_NOTIFY_EVENT => {
                return Ok(Self::CirculateNotify(TryParse::try_parse(event)?.0))
            }
            xproto::CIRCULATE_REQUEST_EVENT => {
                return Ok(Self::CirculateRequest(TryParse::try_parse(event)?.0))
            }
            xproto::CLIENT_MESSAGE_EVENT => {
                return Ok(Self::ClientMessage(TryParse::try_parse(event)?.0))
            }
            xproto::COLORMAP_NOTIFY_EVENT => {
                return Ok(Self::ColormapNotify(TryParse::try_parse(event)?.0))
            }
            xproto::CONFIGURE_NOTIFY_EVENT => {
                return Ok(Self::ConfigureNotify(TryParse::try_parse(event)?.0))
            }
            xproto::CONFIGURE_REQUEST_EVENT => {
                return Ok(Self::ConfigureRequest(TryParse::try_parse(event)?.0))
            }
            xproto::CREATE_NOTIFY_EVENT => {
                return Ok(Self::CreateNotify(TryParse::try_parse(event)?.0))
            }
            xproto::DESTROY_NOTIFY_EVENT => {
                return Ok(Self::DestroyNotify(TryParse::try_parse(event)?.0))
            }
            xproto::ENTER_NOTIFY_EVENT => {
                return Ok(Self::EnterNotify(TryParse::try_parse(event)?.0))
            }
            xproto::EXPOSE_EVENT => return Ok(Self::Expose(TryParse::try_parse(event)?.0)),
            xproto::FOCUS_IN_EVENT => return Ok(Self::FocusIn(TryParse::try_parse(event)?.0)),
            xproto::FOCUS_OUT_EVENT => return Ok(Self::FocusOut(TryParse::try_parse(event)?.0)),
            xproto::GRAPHICS_EXPOSURE_EVENT => {
                return Ok(Self::GraphicsExposure(TryParse::try_parse(event)?.0))
            }
            xproto::GRAVITY_NOTIFY_EVENT => {
                return Ok(Self::GravityNotify(TryParse::try_parse(event)?.0))
            }
            xproto::KEY_PRESS_EVENT => return Ok(Self::KeyPress(TryParse::try_parse(event)?.0)),
            xproto::KEY_RELEASE_EVENT => {
                return Ok(Self::KeyRelease(TryParse::try_parse(event)?.0))
            }
            xproto::KEYMAP_NOTIFY_EVENT => {
                return Ok(Self::KeymapNotify(TryParse::try_parse(event)?.0))
            }
            xproto::LEAVE_NOTIFY_EVENT => {
                return Ok(Self::LeaveNotify(TryParse::try_parse(event)?.0))
            }
            xproto::MAP_NOTIFY_EVENT => return Ok(Self::MapNotify(TryParse::try_parse(event)?.0)),
            xproto::MAP_REQUEST_EVENT => {
                return Ok(Self::MapRequest(TryParse::try_parse(event)?.0))
            }
            xproto::MAPPING_NOTIFY_EVENT => {
                return Ok(Self::MappingNotify(TryParse::try_parse(event)?.0))
            }
            xproto::MOTION_NOTIFY_EVENT => {
                return Ok(Self::MotionNotify(TryParse::try_parse(event)?.0))
            }
            xproto::NO_EXPOSURE_EVENT => {
                return Ok(Self::NoExposure(TryParse::try_parse(event)?.0))
            }
            xproto::PROPERTY_NOTIFY_EVENT => {
                return Ok(Self::PropertyNotify(TryParse::try_parse(event)?.0))
            }
            xproto::REPARENT_NOTIFY_EVENT => {
                return Ok(Self::ReparentNotify(TryParse::try_parse(event)?.0))
            }
            xproto::RESIZE_REQUEST_EVENT => {
                return Ok(Self::ResizeRequest(TryParse::try_parse(event)?.0))
            }
            xproto::SELECTION_CLEAR_EVENT => {
                return Ok(Self::SelectionClear(TryParse::try_parse(event)?.0))
            }
            xproto::SELECTION_NOTIFY_EVENT => {
                return Ok(Self::SelectionNotify(TryParse::try_parse(event)?.0))
            }
            xproto::SELECTION_REQUEST_EVENT => {
                return Ok(Self::SelectionRequest(TryParse::try_parse(event)?.0))
            }
            xproto::UNMAP_NOTIFY_EVENT => {
                return Ok(Self::UnmapNotify(TryParse::try_parse(event)?.0))
            }
            xproto::VISIBILITY_NOTIFY_EVENT => {
                return Ok(Self::VisibilityNotify(TryParse::try_parse(event)?.0))
            }
            xproto::GE_GENERIC_EVENT => return Self::from_generic_event(event, ext_info_provider),
            _ => {}
        }
        // Find the extension that this event could belong to
        let ext_info = ext_info_provider.get_from_event_code(event_code);
        match ext_info {
            #[cfg(feature = "damage")]
            Some((damage::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    damage::NOTIFY_EVENT => Ok(Self::DamageNotify(TryParse::try_parse(event)?.0)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "dri2")]
            Some((dri2::X11_EXTENSION_NAME, ext_info)) => match event_code - ext_info.first_event {
                dri2::BUFFER_SWAP_COMPLETE_EVENT => {
                    Ok(Self::Dri2BufferSwapComplete(TryParse::try_parse(event)?.0))
                }
                dri2::INVALIDATE_BUFFERS_EVENT => {
                    Ok(Self::Dri2InvalidateBuffers(TryParse::try_parse(event)?.0))
                }
                _ => Ok(Self::Unknown(event.to_vec())),
            },
            #[cfg(feature = "glx")]
            Some((glx::X11_EXTENSION_NAME, ext_info)) => match event_code - ext_info.first_event {
                glx::BUFFER_SWAP_COMPLETE_EVENT => {
                    Ok(Self::GlxBufferSwapComplete(TryParse::try_parse(event)?.0))
                }
                glx::PBUFFER_CLOBBER_EVENT => {
                    Ok(Self::GlxPbufferClobber(TryParse::try_parse(event)?.0))
                }
                _ => Ok(Self::Unknown(event.to_vec())),
            },
            #[cfg(feature = "present")]
            Some((present::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    present::GENERIC_EVENT => {
                        Ok(Self::PresentGeneric(TryParse::try_parse(event)?.0))
                    }
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "randr")]
            Some((randr::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    randr::NOTIFY_EVENT => Ok(Self::RandrNotify(TryParse::try_parse(event)?.0)),
                    randr::SCREEN_CHANGE_NOTIFY_EVENT => {
                        Ok(Self::RandrScreenChangeNotify(TryParse::try_parse(event)?.0))
                    }
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "screensaver")]
            Some((screensaver::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    screensaver::NOTIFY_EVENT => {
                        Ok(Self::ScreensaverNotify(TryParse::try_parse(event)?.0))
                    }
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "shape")]
            Some((shape::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    shape::NOTIFY_EVENT => Ok(Self::ShapeNotify(TryParse::try_parse(event)?.0)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "shm")]
            Some((shm::X11_EXTENSION_NAME, ext_info)) => match event_code - ext_info.first_event {
                shm::COMPLETION_EVENT => Ok(Self::ShmCompletion(TryParse::try_parse(event)?.0)),
                _ => Ok(Self::Unknown(event.to_vec())),
            },
            #[cfg(feature = "sync")]
            Some((sync::X11_EXTENSION_NAME, ext_info)) => match event_code - ext_info.first_event {
                sync::ALARM_NOTIFY_EVENT => {
                    Ok(Self::SyncAlarmNotify(TryParse::try_parse(event)?.0))
                }
                sync::COUNTER_NOTIFY_EVENT => {
                    Ok(Self::SyncCounterNotify(TryParse::try_parse(event)?.0))
                }
                _ => Ok(Self::Unknown(event.to_vec())),
            },
            #[cfg(feature = "xfixes")]
            Some((xfixes::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xfixes::CURSOR_NOTIFY_EVENT => {
                        Ok(Self::XfixesCursorNotify(TryParse::try_parse(event)?.0))
                    }
                    xfixes::SELECTION_NOTIFY_EVENT => {
                        Ok(Self::XfixesSelectionNotify(TryParse::try_parse(event)?.0))
                    }
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xinput")]
            Some((xinput::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xinput::CHANGE_DEVICE_NOTIFY_EVENT => Ok(Self::XinputChangeDeviceNotify(
                        TryParse::try_parse(event)?.0,
                    )),
                    xinput::DEVICE_BUTTON_PRESS_EVENT => {
                        Ok(Self::XinputDeviceButtonPress(TryParse::try_parse(event)?.0))
                    }
                    xinput::DEVICE_BUTTON_RELEASE_EVENT => Ok(Self::XinputDeviceButtonRelease(
                        TryParse::try_parse(event)?.0,
                    )),
                    xinput::DEVICE_BUTTON_STATE_NOTIFY_EVENT => Ok(
                        Self::XinputDeviceButtonStateNotify(TryParse::try_parse(event)?.0),
                    ),
                    xinput::DEVICE_FOCUS_IN_EVENT => {
                        Ok(Self::XinputDeviceFocusIn(TryParse::try_parse(event)?.0))
                    }
                    xinput::DEVICE_FOCUS_OUT_EVENT => {
                        Ok(Self::XinputDeviceFocusOut(TryParse::try_parse(event)?.0))
                    }
                    xinput::DEVICE_KEY_PRESS_EVENT => {
                        Ok(Self::XinputDeviceKeyPress(TryParse::try_parse(event)?.0))
                    }
                    xinput::DEVICE_KEY_RELEASE_EVENT => {
                        Ok(Self::XinputDeviceKeyRelease(TryParse::try_parse(event)?.0))
                    }
                    xinput::DEVICE_KEY_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceKeyStateNotify(
                        TryParse::try_parse(event)?.0,
                    )),
                    xinput::DEVICE_MAPPING_NOTIFY_EVENT => Ok(Self::XinputDeviceMappingNotify(
                        TryParse::try_parse(event)?.0,
                    )),
                    xinput::DEVICE_MOTION_NOTIFY_EVENT => Ok(Self::XinputDeviceMotionNotify(
                        TryParse::try_parse(event)?.0,
                    )),
                    xinput::DEVICE_PRESENCE_NOTIFY_EVENT => Ok(Self::XinputDevicePresenceNotify(
                        TryParse::try_parse(event)?.0,
                    )),
                    xinput::DEVICE_PROPERTY_NOTIFY_EVENT => Ok(Self::XinputDevicePropertyNotify(
                        TryParse::try_parse(event)?.0,
                    )),
                    xinput::DEVICE_STATE_NOTIFY_EVENT => {
                        Ok(Self::XinputDeviceStateNotify(TryParse::try_parse(event)?.0))
                    }
                    xinput::DEVICE_VALUATOR_EVENT => {
                        Ok(Self::XinputDeviceValuator(TryParse::try_parse(event)?.0))
                    }
                    xinput::PROXIMITY_IN_EVENT => {
                        Ok(Self::XinputProximityIn(TryParse::try_parse(event)?.0))
                    }
                    xinput::PROXIMITY_OUT_EVENT => {
                        Ok(Self::XinputProximityOut(TryParse::try_parse(event)?.0))
                    }
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xkb")]
            Some((xkb::X11_EXTENSION_NAME, ext_info)) => {
                if event_code != ext_info.first_event {
                    return Ok(Self::Unknown(event.to_vec()));
                }
                match *event.get(1).ok_or(ParseError::InsufficientData)? {
                    xkb::ACCESS_X_NOTIFY_EVENT => {
                        Ok(Self::XkbAccessXNotify(TryParse::try_parse(event)?.0))
                    }
                    xkb::ACTION_MESSAGE_EVENT => {
                        Ok(Self::XkbActionMessage(TryParse::try_parse(event)?.0))
                    }
                    xkb::BELL_NOTIFY_EVENT => {
                        Ok(Self::XkbBellNotify(TryParse::try_parse(event)?.0))
                    }
                    xkb::COMPAT_MAP_NOTIFY_EVENT => {
                        Ok(Self::XkbCompatMapNotify(TryParse::try_parse(event)?.0))
                    }
                    xkb::CONTROLS_NOTIFY_EVENT => {
                        Ok(Self::XkbControlsNotify(TryParse::try_parse(event)?.0))
                    }
                    xkb::EXTENSION_DEVICE_NOTIFY_EVENT => Ok(Self::XkbExtensionDeviceNotify(
                        TryParse::try_parse(event)?.0,
                    )),
                    xkb::INDICATOR_MAP_NOTIFY_EVENT => {
                        Ok(Self::XkbIndicatorMapNotify(TryParse::try_parse(event)?.0))
                    }
                    xkb::INDICATOR_STATE_NOTIFY_EVENT => {
                        Ok(Self::XkbIndicatorStateNotify(TryParse::try_parse(event)?.0))
                    }
                    xkb::MAP_NOTIFY_EVENT => Ok(Self::XkbMapNotify(TryParse::try_parse(event)?.0)),
                    xkb::NAMES_NOTIFY_EVENT => {
                        Ok(Self::XkbNamesNotify(TryParse::try_parse(event)?.0))
                    }
                    xkb::NEW_KEYBOARD_NOTIFY_EVENT => {
                        Ok(Self::XkbNewKeyboardNotify(TryParse::try_parse(event)?.0))
                    }
                    xkb::STATE_NOTIFY_EVENT => {
                        Ok(Self::XkbStateNotify(TryParse::try_parse(event)?.0))
                    }
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xprint")]
            Some((xprint::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xprint::ATTRIBUT_NOTIFY_EVENT => {
                        Ok(Self::XprintAttributNotify(TryParse::try_parse(event)?.0))
                    }
                    xprint::NOTIFY_EVENT => Ok(Self::XprintNotify(TryParse::try_parse(event)?.0)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xv")]
            Some((xv::X11_EXTENSION_NAME, ext_info)) => match event_code - ext_info.first_event {
                xv::PORT_NOTIFY_EVENT => Ok(Self::XvPortNotify(TryParse::try_parse(event)?.0)),
                xv::VIDEO_NOTIFY_EVENT => Ok(Self::XvVideoNotify(TryParse::try_parse(event)?.0)),
                _ => Ok(Self::Unknown(event.to_vec())),
            },
            _ => Ok(Self::Unknown(event.to_vec())),
        }
    }

    #[allow(clippy::match_single_binding)]
    fn from_generic_event(
        event: &[u8],
        ext_info_provider: &ExtensionInfoProvider,
    ) -> Result<Self, ParseError> {
        let ge_event = xproto::GeGenericEvent::try_parse(event)?.0;
        let ext_name = ext_info_provider
            .get_from_major_opcode(ge_event.extension)
            .map(|(name, _)| name);
        match ext_name {
            #[cfg(feature = "present")]
            Some(present::X11_EXTENSION_NAME) => match ge_event.event_type {
                present::COMPLETE_NOTIFY_EVENT => {
                    Ok(Self::PresentCompleteNotify(TryParse::try_parse(event)?.0))
                }
                present::CONFIGURE_NOTIFY_EVENT => {
                    Ok(Self::PresentConfigureNotify(TryParse::try_parse(event)?.0))
                }
                present::IDLE_NOTIFY_EVENT => {
                    Ok(Self::PresentIdleNotify(TryParse::try_parse(event)?.0))
                }
                present::REDIRECT_NOTIFY_EVENT => {
                    Ok(Self::PresentRedirectNotify(TryParse::try_parse(event)?.0))
                }
                _ => Ok(Self::Unknown(event.to_vec())),
            },
            #[cfg(feature = "xinput")]
            Some(xinput::X11_EXTENSION_NAME) => match ge_event.event_type {
                xinput::BARRIER_HIT_EVENT => {
                    Ok(Self::XinputBarrierHit(TryParse::try_parse(event)?.0))
                }
                xinput::BARRIER_LEAVE_EVENT => {
                    Ok(Self::XinputBarrierLeave(TryParse::try_parse(event)?.0))
                }
                xinput::BUTTON_PRESS_EVENT => {
                    Ok(Self::XinputButtonPress(TryParse::try_parse(event)?.0))
                }
                xinput::BUTTON_RELEASE_EVENT => {
                    Ok(Self::XinputButtonRelease(TryParse::try_parse(event)?.0))
                }
                xinput::DEVICE_CHANGED_EVENT => {
                    Ok(Self::XinputDeviceChanged(TryParse::try_parse(event)?.0))
                }
                xinput::ENTER_EVENT => Ok(Self::XinputEnter(TryParse::try_parse(event)?.0)),
                xinput::FOCUS_IN_EVENT => Ok(Self::XinputFocusIn(TryParse::try_parse(event)?.0)),
                xinput::FOCUS_OUT_EVENT => Ok(Self::XinputFocusOut(TryParse::try_parse(event)?.0)),
                xinput::HIERARCHY_EVENT => Ok(Self::XinputHierarchy(TryParse::try_parse(event)?.0)),
                xinput::KEY_PRESS_EVENT => Ok(Self::XinputKeyPress(TryParse::try_parse(event)?.0)),
                xinput::KEY_RELEASE_EVENT => {
                    Ok(Self::XinputKeyRelease(TryParse::try_parse(event)?.0))
                }
                xinput::LEAVE_EVENT => Ok(Self::XinputLeave(TryParse::try_parse(event)?.0)),
                xinput::MOTION_EVENT => Ok(Self::XinputMotion(TryParse::try_parse(event)?.0)),
                xinput::PROPERTY_EVENT => Ok(Self::XinputProperty(TryParse::try_parse(event)?.0)),
                xinput::RAW_BUTTON_PRESS_EVENT => {
                    Ok(Self::XinputRawButtonPress(TryParse::try_parse(event)?.0))
                }
                xinput::RAW_BUTTON_RELEASE_EVENT => {
                    Ok(Self::XinputRawButtonRelease(TryParse::try_parse(event)?.0))
                }
                xinput::RAW_KEY_PRESS_EVENT => {
                    Ok(Self::XinputRawKeyPress(TryParse::try_parse(event)?.0))
                }
                xinput::RAW_KEY_RELEASE_EVENT => {
                    Ok(Self::XinputRawKeyRelease(TryParse::try_parse(event)?.0))
                }
                xinput::RAW_MOTION_EVENT => {
                    Ok(Self::XinputRawMotion(TryParse::try_parse(event)?.0))
                }
                xinput::RAW_TOUCH_BEGIN_EVENT => {
                    Ok(Self::XinputRawTouchBegin(TryParse::try_parse(event)?.0))
                }
                xinput::RAW_TOUCH_END_EVENT => {
                    Ok(Self::XinputRawTouchEnd(TryParse::try_parse(event)?.0))
                }
                xinput::RAW_TOUCH_UPDATE_EVENT => {
                    Ok(Self::XinputRawTouchUpdate(TryParse::try_parse(event)?.0))
                }
                xinput::TOUCH_BEGIN_EVENT => {
                    Ok(Self::XinputTouchBegin(TryParse::try_parse(event)?.0))
                }
                xinput::TOUCH_END_EVENT => Ok(Self::XinputTouchEnd(TryParse::try_parse(event)?.0)),
                xinput::TOUCH_OWNERSHIP_EVENT => {
                    Ok(Self::XinputTouchOwnership(TryParse::try_parse(event)?.0))
                }
                xinput::TOUCH_UPDATE_EVENT => {
                    Ok(Self::XinputTouchUpdate(TryParse::try_parse(event)?.0))
                }
                _ => Ok(Self::Unknown(event.to_vec())),
            },
            _ => Ok(Self::Unknown(event.to_vec())),
        }
    }

    /// Get the sequence number contained in this X11 event
    #[must_use]
    pub fn wire_sequence_number(&self) -> Option<u16> {
        match self {
            Event::Unknown(value) => sequence_number(value).ok(),
            Event::Error(value) => Some(value.sequence),
            Event::ButtonPress(value) => Some(value.sequence),
            Event::ButtonRelease(value) => Some(value.sequence),
            Event::CirculateNotify(value) => Some(value.sequence),
            Event::CirculateRequest(value) => Some(value.sequence),
            Event::ClientMessage(value) => Some(value.sequence),
            Event::ColormapNotify(value) => Some(value.sequence),
            Event::ConfigureNotify(value) => Some(value.sequence),
            Event::ConfigureRequest(value) => Some(value.sequence),
            Event::CreateNotify(value) => Some(value.sequence),
            Event::DestroyNotify(value) => Some(value.sequence),
            Event::EnterNotify(value) => Some(value.sequence),
            Event::Expose(value) => Some(value.sequence),
            Event::FocusIn(value) => Some(value.sequence),
            Event::FocusOut(value) => Some(value.sequence),
            Event::GeGeneric(value) => Some(value.sequence),
            Event::GraphicsExposure(value) => Some(value.sequence),
            Event::GravityNotify(value) => Some(value.sequence),
            Event::KeyPress(value) => Some(value.sequence),
            Event::KeyRelease(value) => Some(value.sequence),
            Event::KeymapNotify(_) => None,
            Event::LeaveNotify(value) => Some(value.sequence),
            Event::MapNotify(value) => Some(value.sequence),
            Event::MapRequest(value) => Some(value.sequence),
            Event::MappingNotify(value) => Some(value.sequence),
            Event::MotionNotify(value) => Some(value.sequence),
            Event::NoExposure(value) => Some(value.sequence),
            Event::PropertyNotify(value) => Some(value.sequence),
            Event::ReparentNotify(value) => Some(value.sequence),
            Event::ResizeRequest(value) => Some(value.sequence),
            Event::SelectionClear(value) => Some(value.sequence),
            Event::SelectionNotify(value) => Some(value.sequence),
            Event::SelectionRequest(value) => Some(value.sequence),
            Event::UnmapNotify(value) => Some(value.sequence),
            Event::VisibilityNotify(value) => Some(value.sequence),
            #[cfg(feature = "damage")]
            Event::DamageNotify(value) => Some(value.sequence),
            #[cfg(feature = "dri2")]
            Event::Dri2BufferSwapComplete(value) => Some(value.sequence),
            #[cfg(feature = "dri2")]
            Event::Dri2InvalidateBuffers(value) => Some(value.sequence),
            #[cfg(feature = "glx")]
            Event::GlxBufferSwapComplete(value) => Some(value.sequence),
            #[cfg(feature = "glx")]
            Event::GlxPbufferClobber(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentCompleteNotify(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentConfigureNotify(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentGeneric(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentIdleNotify(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentRedirectNotify(value) => Some(value.sequence),
            #[cfg(feature = "randr")]
            Event::RandrNotify(value) => Some(value.sequence),
            #[cfg(feature = "randr")]
            Event::RandrScreenChangeNotify(value) => Some(value.sequence),
            #[cfg(feature = "screensaver")]
            Event::ScreensaverNotify(value) => Some(value.sequence),
            #[cfg(feature = "shape")]
            Event::ShapeNotify(value) => Some(value.sequence),
            #[cfg(feature = "shm")]
            Event::ShmCompletion(value) => Some(value.sequence),
            #[cfg(feature = "sync")]
            Event::SyncAlarmNotify(value) => Some(value.sequence),
            #[cfg(feature = "sync")]
            Event::SyncCounterNotify(value) => Some(value.sequence),
            #[cfg(feature = "xfixes")]
            Event::XfixesCursorNotify(value) => Some(value.sequence),
            #[cfg(feature = "xfixes")]
            Event::XfixesSelectionNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputBarrierHit(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputBarrierLeave(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputButtonPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputButtonRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputChangeDeviceNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceChanged(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceFocusIn(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceFocusOut(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceMappingNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceMotionNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDevicePresenceNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDevicePropertyNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceValuator(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputEnter(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputFocusIn(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputFocusOut(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputHierarchy(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputKeyPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputKeyRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputLeave(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputMotion(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputProperty(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputProximityIn(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputProximityOut(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawButtonPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawButtonRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawKeyPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawKeyRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawMotion(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchBegin(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchEnd(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchUpdate(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputTouchBegin(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputTouchEnd(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputTouchOwnership(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputTouchUpdate(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbAccessXNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbActionMessage(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbBellNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbCompatMapNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbControlsNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbExtensionDeviceNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbIndicatorMapNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbIndicatorStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbMapNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbNamesNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbNewKeyboardNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xprint")]
            Event::XprintAttributNotify(value) => Some(value.sequence),
            #[cfg(feature = "xprint")]
            Event::XprintNotify(value) => Some(value.sequence),
            #[cfg(feature = "xv")]
            Event::XvPortNotify(value) => Some(value.sequence),
            #[cfg(feature = "xv")]
            Event::XvVideoNotify(value) => Some(value.sequence),
        }
    }

    /// Get the raw response type of this X11 event
    ///
    /// Response types have seven bits in X11. The eight bit indicates whether
    /// the packet was generated through the `SendEvent` request. This function
    /// returns all eight bits.
    ///
    /// See also the `response_type()`, `server_generated()` and `sent_event()` methods.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn raw_response_type(&self) -> u8 {
        match self {
            Event::Unknown(value) => response_type(value).unwrap(),
            Event::Error(_) => 0,
            Event::ButtonPress(value) => value.response_type,
            Event::ButtonRelease(value) => value.response_type,
            Event::CirculateNotify(value) => value.response_type,
            Event::CirculateRequest(value) => value.response_type,
            Event::ClientMessage(value) => value.response_type,
            Event::ColormapNotify(value) => value.response_type,
            Event::ConfigureNotify(value) => value.response_type,
            Event::ConfigureRequest(value) => value.response_type,
            Event::CreateNotify(value) => value.response_type,
            Event::DestroyNotify(value) => value.response_type,
            Event::EnterNotify(value) => value.response_type,
            Event::Expose(value) => value.response_type,
            Event::FocusIn(value) => value.response_type,
            Event::FocusOut(value) => value.response_type,
            Event::GeGeneric(value) => value.response_type,
            Event::GraphicsExposure(value) => value.response_type,
            Event::GravityNotify(value) => value.response_type,
            Event::KeyPress(value) => value.response_type,
            Event::KeyRelease(value) => value.response_type,
            Event::KeymapNotify(value) => value.response_type,
            Event::LeaveNotify(value) => value.response_type,
            Event::MapNotify(value) => value.response_type,
            Event::MapRequest(value) => value.response_type,
            Event::MappingNotify(value) => value.response_type,
            Event::MotionNotify(value) => value.response_type,
            Event::NoExposure(value) => value.response_type,
            Event::PropertyNotify(value) => value.response_type,
            Event::ReparentNotify(value) => value.response_type,
            Event::ResizeRequest(value) => value.response_type,
            Event::SelectionClear(value) => value.response_type,
            Event::SelectionNotify(value) => value.response_type,
            Event::SelectionRequest(value) => value.response_type,
            Event::UnmapNotify(value) => value.response_type,
            Event::VisibilityNotify(value) => value.response_type,
            #[cfg(feature = "damage")]
            Event::DamageNotify(value) => value.response_type,
            #[cfg(feature = "dri2")]
            Event::Dri2BufferSwapComplete(value) => value.response_type,
            #[cfg(feature = "dri2")]
            Event::Dri2InvalidateBuffers(value) => value.response_type,
            #[cfg(feature = "glx")]
            Event::GlxBufferSwapComplete(value) => value.response_type,
            #[cfg(feature = "glx")]
            Event::GlxPbufferClobber(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentCompleteNotify(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentConfigureNotify(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentGeneric(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentIdleNotify(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentRedirectNotify(value) => value.response_type,
            #[cfg(feature = "randr")]
            Event::RandrNotify(value) => value.response_type,
            #[cfg(feature = "randr")]
            Event::RandrScreenChangeNotify(value) => value.response_type,
            #[cfg(feature = "screensaver")]
            Event::ScreensaverNotify(value) => value.response_type,
            #[cfg(feature = "shape")]
            Event::ShapeNotify(value) => value.response_type,
            #[cfg(feature = "shm")]
            Event::ShmCompletion(value) => value.response_type,
            #[cfg(feature = "sync")]
            Event::SyncAlarmNotify(value) => value.response_type,
            #[cfg(feature = "sync")]
            Event::SyncCounterNotify(value) => value.response_type,
            #[cfg(feature = "xfixes")]
            Event::XfixesCursorNotify(value) => value.response_type,
            #[cfg(feature = "xfixes")]
            Event::XfixesSelectionNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputBarrierHit(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputBarrierLeave(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputButtonPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputButtonRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputChangeDeviceNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonStateNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceChanged(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceFocusIn(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceFocusOut(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyStateNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceMappingNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceMotionNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDevicePresenceNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDevicePropertyNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceStateNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceValuator(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputEnter(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputFocusIn(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputFocusOut(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputHierarchy(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputKeyPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputKeyRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputLeave(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputMotion(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputProperty(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputProximityIn(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputProximityOut(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawButtonPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawButtonRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawKeyPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawKeyRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawMotion(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchBegin(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchEnd(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchUpdate(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputTouchBegin(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputTouchEnd(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputTouchOwnership(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputTouchUpdate(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbAccessXNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbActionMessage(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbBellNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbCompatMapNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbControlsNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbExtensionDeviceNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbIndicatorMapNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbIndicatorStateNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbMapNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbNamesNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbNewKeyboardNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbStateNotify(value) => value.response_type,
            #[cfg(feature = "xprint")]
            Event::XprintAttributNotify(value) => value.response_type,
            #[cfg(feature = "xprint")]
            Event::XprintNotify(value) => value.response_type,
            #[cfg(feature = "xv")]
            Event::XvPortNotify(value) => value.response_type,
            #[cfg(feature = "xv")]
            Event::XvVideoNotify(value) => value.response_type,
        }
    }

    /// Get the response type of this X11 event
    #[must_use]
    pub fn response_type(&self) -> u8 {
        self.raw_response_type() & 0x7f
    }

    /// Was this event generated by the X11 server?
    ///
    /// If this function returns true, then this event comes from the X11 server.
    /// Otherwise, it was sent from another client via the `SendEvent` request.
    #[must_use]
    pub fn server_generated(&self) -> bool {
        self.raw_response_type() & 0x80 == 0
    }

    /// Was this event generated by another X11 client?
    ///
    /// If this function returns true, then this event comes from another client via
    /// the `SendEvent` request. Otherwise, it was generated by the X11 server.
    #[must_use]
    pub fn sent_event(&self) -> bool {
        self.raw_response_type() & 0x80 != 0
    }
}

/// Get the response type out of the raw bytes of an X11 error or event.
fn response_type(raw_bytes: &[u8]) -> Result<u8, ParseError> {
    raw_bytes
        .get(0)
        .map(|x| x & 0x7f)
        .ok_or(ParseError::InsufficientData)
}

/// Get the sequence number out of an X11 packet.
fn sequence_number(raw_bytes: &[u8]) -> Result<u16, ParseError> {
    raw_bytes
        .get(2..4)
        .map(|b| u16::from_ne_bytes(b.try_into().unwrap()))
        .ok_or(ParseError::InsufficientData)
}
