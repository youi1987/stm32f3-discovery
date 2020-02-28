//! Provides access to User LEDs LD3-LD10
use stm32f3xx_hal::gpio::gpioe;
use stm32f3xx_hal::gpio::{Floating, Input, Output, PushPull};

use switch_hal::{ActiveHigh, IntoSwitch, OutputSwitch, Switch};

/// GpioE after Led pins (PE8-PE15) have been moved
/// If you intend to use those pins for other functions, DO NOT call Leds::init().
/// You'll have to initialize the pins yourself.
#[deprecated(since = "0.3.4", note = "Will be removed in 0.4.0")]
pub struct GpioE {
    /// Opaque AFRH register
    pub afrh: gpioe::AFRH,
    /// Opaque AFRL register
    pub afrl: gpioe::AFRL,
    /// Opaque MODER register
    pub moder: gpioe::MODER,
    /// Opaque OTYPER register
    pub otyper: gpioe::OTYPER,
    /// Opaque PUPDR register
    pub pupdr: gpioe::PUPDR,

    pub pe0: gpioe::PE0<Input<Floating>>,
    pub pe1: gpioe::PE1<Input<Floating>>,
    pub pe2: gpioe::PE2<Input<Floating>>,
    pub pe3: gpioe::PE3<Input<Floating>>,
    pub pe4: gpioe::PE4<Input<Floating>>,
    pub pe5: gpioe::PE5<Input<Floating>>,
    pub pe6: gpioe::PE6<Input<Floating>>,
    pub pe7: gpioe::PE7<Input<Floating>>,
}

pub struct Leds {
    /// North
    pub ld3: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// NorthWest
    pub ld4: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// NorthEast
    pub ld5: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// West
    pub ld6: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// East
    pub ld7: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// SouthWest
    pub ld8: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// SouthEast
    pub ld9: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// South
    pub ld10: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
}

impl Leds {
    /// Initializes the user LEDs
    ///
    /// ## Returns
    /// A tuple of `(Leds, GpioE)`, where `Leds` has taken ownership of PE8-PE15
    /// and `GpioE` contains the remaining members of `stm32f3xx_hal::gpio::GPIOE`
    ///
    /// **If you intend to use those pins for other functions, DO NOT call Leds::init().**
    /// You'll have to initialize the pins yourself.
    #[deprecated(since = "0.3.4", note = "Will be removed in 0.4.0. Use `new` instead.")]
    pub fn init(mut gpioe: gpioe::Parts) -> (Self, GpioE) {
        let mut leds = Leds {
            ld3: gpioe
                .pe9
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                .downgrade()
                .into_active_high_switch(),
            ld4: gpioe
                .pe8
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                .downgrade()
                .into_active_high_switch(),
            ld5: gpioe
                .pe10
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                .downgrade()
                .into_active_high_switch(),
            ld6: gpioe
                .pe15
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                .downgrade()
                .into_active_high_switch(),
            ld7: gpioe
                .pe11
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                .downgrade()
                .into_active_high_switch(),
            ld8: gpioe
                .pe14
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                .downgrade()
                .into_active_high_switch(),
            ld9: gpioe
                .pe12
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                .downgrade()
                .into_active_high_switch(),
            ld10: gpioe
                .pe13
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                .downgrade()
                .into_active_high_switch(),
        };

        //TODO: expose an iterator
        leds.ld3.off().ok();
        leds.ld4.off().ok();
        leds.ld5.off().ok();
        leds.ld6.off().ok();
        leds.ld7.off().ok();
        leds.ld8.off().ok();
        leds.ld9.off().ok();
        leds.ld10.off().ok();

        (
            leds,
            GpioE {
                afrh: gpioe.afrh,
                afrl: gpioe.afrl,
                moder: gpioe.moder,
                otyper: gpioe.otyper,
                pupdr: gpioe.pupdr,
                pe0: gpioe.pe0,
                pe1: gpioe.pe1,
                pe2: gpioe.pe2,
                pe3: gpioe.pe3,
                pe4: gpioe.pe4,
                pe5: gpioe.pe5,
                pe6: gpioe.pe6,
                pe7: gpioe.pe7,
            },
        )
    }

    /// Initializes the user LEDs to OFF
    pub fn new<PE8Mode, PE9Mode, PE10Mode, PE11Mode, PE12Mode, PE13Mode, PE14Mode, PE15Mode>(
        pe8: gpioe::PE8<PE8Mode>,
        pe9: gpioe::PE9<PE9Mode>,
        pe10: gpioe::PE10<PE10Mode>,
        pe11: gpioe::PE11<PE11Mode>,
        pe12: gpioe::PE12<PE12Mode>,
        pe13: gpioe::PE13<PE13Mode>,
        pe14: gpioe::PE14<PE14Mode>,
        pe15: gpioe::PE15<PE15Mode>,
        moder: &mut gpioe::MODER,
        otyper: &mut gpioe::OTYPER,
    ) -> Self {
        let mut leds = Leds {
            ld3: pe9
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld4: pe8
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld5: pe10
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld6: pe15
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld7: pe11
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld8: pe14
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld9: pe12
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld10: pe13
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
        };

        //TODO: expose an iterator
        leds.ld3.off().ok();
        leds.ld4.off().ok();
        leds.ld5.off().ok();
        leds.ld6.off().ok();
        leds.ld7.off().ok();
        leds.ld8.off().ok();
        leds.ld9.off().ok();
        leds.ld10.off().ok();

        leds
    }

    /// Consumes the `Leds` struct and returns an array
    /// where index 0 is N and each incrementing index
    /// rotates clockwise around the compass
    pub fn into_array(self) -> [Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>; 8] {
        [
            self.ld3,  //N
            self.ld5,  //NE
            self.ld7,  //E
            self.ld9,  //SE
            self.ld10, //S
            self.ld8,  //SW
            self.ld6,  //W
            self.ld4,  //NW
        ]
    }
}