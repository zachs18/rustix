use crate::backend::c;
use bitflags::bitflags;

/// `struct termios` for use with [`tcgetattr`] and [`tcsetattr`].
///
/// [`tcgetattr`]: crate::termios::tcgetattr
/// [`tcsetattr`]: crate::termios::tcsetattr
#[repr(C)]
pub struct Termios {
    /// How is input interpreted?
    #[doc(alias = "c_iflag")]
    pub input_modes: InputModes,

    /// How is output translated?
    #[doc(alias = "c_oflag")]
    pub output_modes: OutputModes,

    /// Low-level configuration flags.
    #[doc(alias = "c_cflag")]
    pub control_modes: ControlModes,

    /// High-level configuration flags.
    #[doc(alias = "c_lflag")]
    pub local_modes: LocalModes,

    /// How are various special control codes handled?
    #[doc(alias = "c_cc")]
    pub special_codes: SpecialCodes,

    /// Input communication speed.
    #[doc(alias = "c_ispeed")]
    pub input_speed: c::speed_t,

    /// Ouptut communication speed.
    #[doc(alias = "c_ospeed")]
    pub output_speed: c::speed_t,
}

bitflags! {
    /// Flags controlling terminal input.
    pub struct InputModes: c::tcflag_t {
        /// `IGNBRK`
        const IGNBRK = c::IGNBRK;

        /// `BRKINT`
        const BRKINT = c::BRKINT;

        /// `IGNPAR`
        const IGNPAR = c::IGNPAR;

        /// `PARMRK`
        const PARMRK = c::PARMRK;

        /// `INPCK`
        const INPCK = c::INPCK;

        /// `ISTRIP`
        const ISTRIP = c::ISTRIP;

        /// `INLCR`
        const INLCR = c::INLCR;

        /// `IGNCR`
        const IGNCR = c::IGNCR;

        /// `ICRNL`
        const ICRNL = c::ICRNL;

        /// `IUCLC`
        #[cfg(any(solarish, target_os = "haiku"))]
        const IUCLC = c::IUCLC;

        /// `IXON`
        const IXON = c::IXON;

        /// `IXANY`
        #[cfg(not(target_os = "redox"))]
        const IXANY = c::IXANY;

        /// `IXOFF`
        const IXOFF = c::IXOFF;

        /// `IMAXBEL`
        #[cfg(not(any(target_os = "haiku", target_os = "redox")))]
        const IMAXBEL = c::IMAXBEL;

        /// `IUTF8`
        #[cfg(not(any(
            solarish,
            target_os = "aix",
            target_os = "dragonfly",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "haiku",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const IUTF8 = c::IUTF8;
    }
}

bitflags! {
    /// Flags controlling terminal output.
    pub struct OutputModes: c::tcflag_t {
        /// `OPOST`
        const OPOST = c::OPOST;

        /// `OLCUC`
        #[cfg(not(any(
            apple,
            target_os = "aix",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "redox",
        )))]
        const OLCUC = c::OLCUC;

        /// `ONLCR`
        const ONLCR = c::ONLCR;

        /// `OCRNL`
        const OCRNL = c::OCRNL;

        /// `ONOCR`
        const ONOCR = c::ONOCR;

        /// `ONLRET`
        const ONLRET = c::ONLRET;

        /// `OFILL`
        #[cfg(not(bsd))]
        const OFILL = c::OFILL;

        /// `OFDEL`
        #[cfg(not(bsd))]
        const OFDEL = c::OFDEL;

        /// `NLDLY`
        #[cfg(not(any(bsd, solarish, target_os = "redox")))]
        const NLDLY = c::NLDLY;

        /// `NL0`
        #[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
        const NL0 = c::NL0;

        /// `NL1`
        #[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
        const NL1 = c::NL1;

        /// `CRDLY`
        #[cfg(not(any(bsd, solarish, target_os = "redox")))]
        const CRDLY = c::CRDLY;

        /// `CR0`
        #[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
        const CR0 = c::CR0;

        /// `CR1`
        #[cfg(not(any(
            target_env = "musl",
            bsd,
            solarish,
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const CR1 = c::CR1;

        /// `CR2`
        #[cfg(not(any(
            target_env = "musl",
            bsd,
            solarish,
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const CR2 = c::CR2;

        /// `CR3`
        #[cfg(not(any(
            target_env = "musl",
            bsd,
            solarish,
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const CR3 = c::CR3;

        /// `TABDLY`
        #[cfg(not(any(
            netbsdlike,
            solarish,
            target_os = "dragonfly",
            target_os = "redox",
        )))]
        const TABDLY = c::TABDLY;

        /// `TAB0`
        #[cfg(not(any(
            netbsdlike,
            solarish,
            target_os = "dragonfly",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const TAB0 = c::TAB0;

        /// `TAB1`
        #[cfg(not(any(
            target_env = "musl",
            bsd,
            solarish,
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const TAB1 = c::TAB1;

        /// `TAB2`
        #[cfg(not(any(
            target_env = "musl",
            bsd,
            solarish,
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const TAB2 = c::TAB2;

        /// `TAB3`
        #[cfg(not(any(
            target_env = "musl",
            bsd,
            solarish,
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const TAB3 = c::TAB3;

        /// `XTABS`
        #[cfg(not(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "haiku",
            target_os = "redox",
        )))]
        const XTABS = c::XTABS;

        /// `BSDLY`
        #[cfg(not(any(bsd, solarish, target_os = "redox")))]
        const BSDLY = c::BSDLY;

        /// `BS0`
        #[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
        const BS0 = c::BS0;

        /// `BS1`
        #[cfg(not(any(
            target_env = "musl",
            bsd,
            solarish,
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const BS1 = c::BS1;

        /// `FFDLY`
        #[cfg(not(any(target_env = "musl", bsd, solarish, target_os = "redox")))]
        const FFDLY = c::FFDLY;

        /// `FF0`
        #[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
        const FF0 = c::FF0;

        /// `FF1`
        #[cfg(not(any(
            target_env = "musl",
            bsd,
            solarish,
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const FF1 = c::FF1;

        /// `VTDLY`
        #[cfg(not(any(target_env = "musl", bsd, solarish, target_os = "redox")))]
        const VTDLY = c::VTDLY;

        /// `VT0`
        #[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
        const VT0 = c::VT0;

        /// `VT1`
        #[cfg(not(any(
            target_env = "musl",
            bsd,
            solarish,
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "redox",
        )))]
        const VT1 = c::VT1;
    }
}

bitflags! {
    /// Flags controlling special terminal modes.
    pub struct ControlModes: c::tcflag_t {
        /// `CSIZE`
        const CSIZE = c::CSIZE;

        /// `CS5`
        const CS5 = c::CS5;

        /// `CS6`
        const CS6 = c::CS6;

        /// `CS7`
        const CS7 = c::CS7;

        /// `CS8`
        const CS8 = c::CS8;

        /// `CSTOPB`
        const CSTOPB = c::CSTOPB;

        /// `CREAD`
        const CREAD = c::CREAD;

        /// `PARENB`
        const PARENB = c::PARENB;

        /// `PARODD`
        const PARODD = c::PARODD;

        /// `HUPCL`
        const HUPCL = c::HUPCL;

        /// `CLOCAL`
        const CLOCAL = c::CLOCAL;

        /// `CRTSCTS`
        #[cfg(not(any(target_os = "aix", target_os = "redox")))]
        const CRTSCTS = c::CRTSCTS;

        /// `CBAUD`
        #[cfg(not(any(bsd, target_os = "haiku", target_os = "redox")))]
        const CBAUD = c::CBAUD;

        /// `CBAUDEX`
        #[cfg(not(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "haiku",
            target_os = "redox",
        )))]
        const CBAUDEX = c::CBAUDEX;

        /// `CIBAUD`
        #[cfg(not(any(
            bsd,
            target_os = "emscripten",
            target_os = "haiku",
            target_os = "redox",
        )))]
        const CIBAUD = c::CIBAUD;

        /// `CMSPAR`
        #[cfg(not(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "emscripten",
            target_os = "haiku",
            target_os = "redox",
        )))]
        const CMSPAR = c::CMSPAR;
    }
}

bitflags! {
    /// Flags controlling "local" terminal modes.
    pub struct LocalModes: c::tcflag_t {
        /// `XCASE`
        #[cfg(any(target_arch = "s390x", target_os = "haiku"))]
        const XCASE = c::XCASE;

        /// `ECHOCTL`
        #[cfg(not(target_os = "redox"))]
        const ECHOCTL = c::ECHOCTL;

        /// `ECHOPRT`
        #[cfg(not(target_os = "redox"))]
        const ECHOPRT = c::ECHOPRT;

        /// `ECHOKE`
        #[cfg(not(target_os = "redox"))]
        const ECHOKE = c::ECHOKE;

        /// `FLUSHO`
        #[cfg(not(target_os = "redox"))]
        const FLUSHO = c::FLUSHO;

        /// `PENDIN`
        #[cfg(not(target_os = "redox"))]
        const PENDIN = c::PENDIN;

        /// `EXTPROC`
        #[cfg(not(any(target_os = "aix", target_os = "haiku", target_os = "redox")))]
        const EXTPROC = c::EXTPROC;

        /// `ISIG`
        const ISIG = c::ISIG;

        /// `ICANON`—A flag for the `c_lflag` field of [`Termios`] indicating
        /// canonical mode.
        const ICANON = c::ICANON;

        /// `ECHO`
        const ECHO = c::ECHO;

        /// `ECHOE`
        const ECHOE = c::ECHOE;

        /// `ECHOK`
        const ECHOK = c::ECHOK;

        /// `ECHONL`
        const ECHONL = c::ECHONL;

        /// `NOFLSH`
        const NOFLSH = c::NOFLSH;

        /// `TOSTOP`
        const TOSTOP = c::TOSTOP;

        /// `IEXTEN`
        const IEXTEN = c::IEXTEN;
    }
}

/// `speed_t`—A return type for [`cfsetspeed`] and similar.
///
/// [`cfsetspeed`]: crate::termios::cfsetspeed
#[repr(transparent)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Speed(pub(crate) c::speed_t);

impl Speed {
    /// `B0`
    pub const B0: Self = Self(c::B0);

    /// `B50`
    pub const B50: Self = Self(c::B50);

    /// `B75`
    pub const B75: Self = Self(c::B75);

    /// `B110`
    pub const B110: Self = Self(c::B110);

    /// `B134`
    pub const B134: Self = Self(c::B134);

    /// `B150`
    pub const B150: Self = Self(c::B150);

    /// `B200`
    pub const B200: Self = Self(c::B200);

    /// `B300`
    pub const B300: Self = Self(c::B300);

    /// `B600`
    pub const B600: Self = Self(c::B600);

    /// `B1200`
    pub const B1200: Self = Self(c::B1200);

    /// `B1800`
    pub const B1800: Self = Self(c::B1800);

    /// `B2400`
    pub const B2400: Self = Self(c::B2400);

    /// `B4800`
    pub const B4800: Self = Self(c::B4800);

    /// `B9600`
    pub const B9600: Self = Self(c::B9600);

    /// `B19200`
    pub const B19200: Self = Self(c::B19200);

    /// `B38400`
    pub const B38400: Self = Self(c::B38400);

    /// `B57600`
    #[cfg(not(target_os = "aix"))]
    pub const B57600: Self = Self(c::B57600);

    /// `B115200`
    #[cfg(not(target_os = "aix"))]
    pub const B115200: Self = Self(c::B115200);

    /// `B230400`
    #[cfg(not(target_os = "aix"))]
    pub const B230400: Self = Self(c::B230400);

    /// `B460800`
    #[cfg(not(any(
        apple,
        target_os = "aix",
        target_os = "dragonfly",
        target_os = "haiku",
        target_os = "openbsd"
    )))]
    pub const B460800: Self = Self(c::B460800);

    /// `B500000`
    #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
    pub const B500000: Self = Self(c::B500000);

    /// `B576000`
    #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
    pub const B576000: Self = Self(c::B576000);

    /// `B921600`
    #[cfg(not(any(
        apple,
        target_os = "aix",
        target_os = "dragonfly",
        target_os = "haiku",
        target_os = "openbsd"
    )))]
    pub const B921600: Self = Self(c::B921600);

    /// `B1000000`
    #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
    pub const B1000000: Self = Self(c::B1000000);

    /// `B1152000`
    #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
    pub const B1152000: Self = Self(c::B1152000);

    /// `B1500000`
    #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
    pub const B1500000: Self = Self(c::B1500000);

    /// `B2000000`
    #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
    pub const B2000000: Self = Self(c::B2000000);

    /// `B2500000`
    #[cfg(not(any(
        target_arch = "sparc",
        target_arch = "sparc64",
        bsd,
        target_os = "aix",
        target_os = "haiku",
        target_os = "solaris",
    )))]
    pub const B2500000: Self = Self(c::B2500000);

    /// `B3000000`
    #[cfg(not(any(
        target_arch = "sparc",
        target_arch = "sparc64",
        bsd,
        target_os = "aix",
        target_os = "haiku",
        target_os = "solaris",
    )))]
    pub const B3000000: Self = Self(c::B3000000);

    /// `B3500000`
    #[cfg(not(any(
        target_arch = "sparc",
        target_arch = "sparc64",
        bsd,
        target_os = "aix",
        target_os = "haiku",
        target_os = "solaris",
    )))]
    pub const B3500000: Self = Self(c::B3500000);

    /// `B4000000`
    #[cfg(not(any(
        target_arch = "sparc",
        target_arch = "sparc64",
        bsd,
        target_os = "aix",
        target_os = "haiku",
        target_os = "solaris",
    )))]
    pub const B4000000: Self = Self(c::B4000000);

    /// `BOTHER`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    pub const BOTHER: Self = Self(c::BOTHER);

    /// `EXTA`
    #[cfg(not(any(
        solarish,
        target_os = "emscripten",
        target_os = "haiku",
        target_os = "redox",
    )))]
    pub const EXTA: Self = Self(c::EXTA);

    /// `EXTB`
    #[cfg(not(any(
        solarish,
        target_os = "emscripten",
        target_os = "haiku",
        target_os = "redox",
    )))]
    pub const EXTB: Self = Self(c::EXTB);
}

/// An array indexed by `SpecialCodeIndex` indicating the current values
/// of various special control codes.
#[repr(transparent)]
pub struct SpecialCodes([c::cc_t; c::NCCS as usize]);

impl std::ops::Index<SpecialCodeIndex> for SpecialCodes {
    type Output = c::cc_t;

    fn index(&self, index: SpecialCodeIndex) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::IndexMut<SpecialCodeIndex> for SpecialCodes {
    fn index_mut(&mut self, index: SpecialCodeIndex) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

/// Indices for use with `Termios::special_codes`.
pub struct SpecialCodeIndex(usize);

impl SpecialCodeIndex {
    /// `VINTR`
    pub const VINTR: Self = Self(c::VINTR as usize);

    /// `VQUIT`
    pub const VQUIT: Self = Self(c::VQUIT as usize);

    /// `VERASE`
    pub const VERASE: Self = Self(c::VERASE as usize);

    /// `VKILL`
    pub const VKILL: Self = Self(c::VKILL as usize);

    /// `VEOF`
    pub const VEOF: Self = Self(c::VEOF as usize);

    /// `VTIME`
    pub const VTIME: Self = Self(c::VTIME as usize);

    /// `VMIN`
    pub const VMIN: Self = Self(c::VMIN as usize);

    /// `VSWTC`
    #[cfg(not(any(
        apple,
        solarish,
        target_os = "aix",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "haiku",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const VSWTC: Self = Self(c::VSWTC as usize);

    /// `VSTART`
    pub const VSTART: Self = Self(c::VSTART as usize);

    /// `VSTOP`
    pub const VSTOP: Self = Self(c::VSTOP as usize);

    /// `VSUSP`
    pub const VSUSP: Self = Self(c::VSUSP as usize);

    /// `VEOL`
    pub const VEOL: Self = Self(c::VEOL as usize);

    /// `VREPRINT`
    #[cfg(not(target_os = "haiku"))]
    pub const VREPRINT: Self = Self(c::VREPRINT as usize);

    /// `VDISCARD`
    #[cfg(not(any(target_os = "aix", target_os = "haiku")))]
    pub const VDISCARD: Self = Self(c::VDISCARD as usize);

    /// `VWERASE`
    #[cfg(not(any(target_os = "aix", target_os = "haiku")))]
    pub const VWERASE: Self = Self(c::VWERASE as usize);

    /// `VLNEXT`
    #[cfg(not(target_os = "haiku"))]
    pub const VLNEXT: Self = Self(c::VLNEXT as usize);

    /// `VEOL2`
    pub const VEOL2: Self = Self(c::VEOL2 as usize);
}

/// Translate from a `Speed` code to a speed value `u32`.
///
/// ```
/// let speed = rustix::termios::speed_value(rustix::termios::B57600);
/// assert_eq!(speed, Some(57600));
/// ```
pub fn speed_value(speed: Speed) -> Option<u32> {
    match speed {
        Speed::B0 => Some(0),
        Speed::B50 => Some(50),
        Speed::B75 => Some(75),
        Speed::B110 => Some(110),
        Speed::B134 => Some(134),
        Speed::B150 => Some(150),
        Speed::B200 => Some(200),
        Speed::B300 => Some(300),
        Speed::B600 => Some(600),
        Speed::B1200 => Some(1200),
        Speed::B1800 => Some(1800),
        Speed::B2400 => Some(2400),
        Speed::B4800 => Some(4800),
        Speed::B9600 => Some(9600),
        Speed::B19200 => Some(19200),
        Speed::B38400 => Some(38400),
        #[cfg(not(target_os = "aix"))]
        Speed::B57600 => Some(57600),
        #[cfg(not(target_os = "aix"))]
        Speed::B115200 => Some(115_200),
        #[cfg(not(target_os = "aix"))]
        Speed::B230400 => Some(230_400),
        #[cfg(not(any(
            apple,
            target_os = "aix",
            target_os = "dragonfly",
            target_os = "haiku",
            target_os = "openbsd"
        )))]
        Speed::B460800 => Some(460_800),
        #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
        Speed::B500000 => Some(500_000),
        #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
        Speed::B576000 => Some(576_000),
        #[cfg(not(any(
            apple,
            target_os = "aix",
            target_os = "dragonfly",
            target_os = "haiku",
            target_os = "openbsd"
        )))]
        Speed::B921600 => Some(921_600),
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        Speed::B1000000 => Some(1_000_000),
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        Speed::B1152000 => Some(1_152_000),
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        Speed::B1500000 => Some(1_500_000),
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        Speed::B2000000 => Some(2_000_000),
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        Speed::B2500000 => Some(2_500_000),
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        Speed::B3000000 => Some(3_000_000),
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        Speed::B3500000 => Some(3_500_000),
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        Speed::B4000000 => Some(4_000_000),
        _ => None,
    }
}

/// `TCSA*` values for use with [`tcsetattr`].
///
/// [`tcsetattr`]: crate::termios::tcsetattr
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum OptionalActions {
    /// `TCSANOW`—Make the change immediately.
    #[doc(alias = "TCSANOW")]
    Now = c::TCSANOW as u32,

    /// `TCSADRAIN`—Make the change after all output has been transmitted.
    #[doc(alias = "TCSADRAIN")]
    Drain = c::TCSADRAIN as u32,

    /// `TCSAFLUSH`—Discard any pending input and then make the change
    /// after all output has been transmitted.
    #[doc(alias = "TCSAFLUSH")]
    Flush = c::TCSAFLUSH as u32,
}

/// `TC*` values for use with [`tcflush`].
///
/// [`tcflush`]: crate::termios::tcflush
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum QueueSelector {
    /// `TCIFLUSH`—Flush data received but not read.
    #[doc(alias = "TCIFLUSH")]
    IFlush = c::TCIFLUSH as u32,

    /// `TCOFLUSH`—Flush data written but not transmitted.
    #[doc(alias = "TCOFLUSH")]
    OFlush = c::TCOFLUSH as u32,

    /// `TCIOFLUSH`—`IFlush` and `OFlush` combined.
    #[doc(alias = "TCIOFLUSH")]
    IOFlush = c::TCIOFLUSH as u32,
}

/// `TC*` values for use with [`tcflow`].
///
/// [`tcflow`]: crate::termios::tcflow
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum Action {
    /// `TCOOFF`—Suspend output.
    #[doc(alias = "TCOOFF")]
    OOff = c::TCOOFF as u32,

    /// `TCOON`—Restart suspended output.
    #[doc(alias = "TCOON")]
    OOn = c::TCOON as u32,

    /// `TCIOFF`—Transmits a STOP byte.
    #[doc(alias = "TCIOFF")]
    IOff = c::TCIOFF as u32,

    /// `TCION`—Transmits a START byte.
    #[doc(alias = "TCION")]
    IOn = c::TCION as u32,
}

/// `struct winsize` for use with [`tcgetwinsize`].
///
/// [`tcgetwinsize`]: crate::termios::tcgetwinsize
#[doc(alias = "winsize")]
pub type Winsize = c::winsize;

#[test]
fn termios_layouts() {
    #[cfg(any(target_os = "android", target_os = "linux"))]
    type RawTermios = c::termios2;

    #[cfg(not(any(target_os = "android", target_os = "linux")))]
    type RawTermios = c::termios;

    check_renamed_type(Termios, RawTermios);
    check_renamed_struct_renamed_field!(Termios, RawTermios, input_flags, c_iflag);
    check_renamed_struct_renamed_field!(Termios, RawTermios, output_flags, c_oflag);
    check_renamed_struct_renamed_field!(Termios, RawTermios, control_flags, c_cflag);
    check_renamed_struct_renamed_field!(Termios, RawTermios, local_flags, c_lflag);
    check_renamed_struct_renamed_field!(Termios, RawTermios, special_codes, c_cc);
    check_renamed_struct_renamed_field!(Termios, RawTermios, input_speed, c_ispeed);
    check_renamed_struct_renamed_field!(Termios, RawTermios, output_speed, c_ospeed);

    // On platforms with a termios/termios2 split, check `termios`.
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        check_renamed_struct_renamed_field!(Termios, termios, input_flags, c_iflag);
        check_renamed_struct_renamed_field!(Termios, termios, output_flags, c_oflag);
        check_renamed_struct_renamed_field!(Termios, termios, control_flags, c_cflag);
        check_renamed_struct_renamed_field!(Termios, termios, local_flags, c_lflag);
        check_renamed_struct_renamed_field!(Termios, termios, special_codes, c_cc);
        assert_eq!(offset_of!(Termios, input_speed), size_of::<termios);
    }

    check_renamed_type(OptionalActions, c::c_int);
    check_renamed_type(QueueSelector, c::c_int);
    check_renamed_type(Action, c::c_int);
}
