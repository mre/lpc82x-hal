//! APIs for the switch matrix (SWM)
//!
//! See user manual, chapter 7.


use lpc82x;

use gpio::PinName;
use init_state::{
    self,
    InitState,
};
use syscon;


/// Interface to the switch matrix (SWM)
///
/// This API expects to be the sole owner of the SWM peripheral. Don't use
/// [`lpc82x::SWM`] directly, unless you know what you're doing.
///
/// [`lpc82x::SWM`]: ../../lpc82x/struct.SWM.html
pub struct SWM<'swm> {
    /// Main SWM API
    pub api: Api<'swm, init_state::Unknown>,

    /// Movable functions
    pub movable_functions: MovableFunctions,

    /// Fixed functions
    pub fixed_functions: FixedFunctions,
}

impl<'swm> SWM<'swm> {
    pub(crate) fn new(swm: &'swm lpc82x::SWM) -> Self {
        SWM {
            api              : Api::new(swm),
            movable_functions: MovableFunctions::new(),
            fixed_functions  : FixedFunctions::new(),
        }
    }
}


/// Main API of the SWM peripheral
pub struct Api<'swm, State: InitState = init_state::Initialized> {
    swm   : &'swm lpc82x::SWM,
    _state: State,
}

impl<'swm> Api<'swm, init_state::Unknown> {
    pub(crate) fn new(swm: &'swm lpc82x::SWM) -> Self {
        Api {
            swm   : swm,
            _state: init_state::Unknown,
        }
    }

    /// Initialize the switch matrix
    pub fn init(mut self, syscon: &mut syscon::Api)
        -> Api<'swm, init_state::Initialized>
    {
        syscon.enable_clock(&mut self.swm);

        Api {
            swm   : self.swm,
            _state: init_state::Initialized,
        }
    }
}


/// Implemented for types that represent movable functions
pub trait MovableFunction {
    /// Assigns the movable function to a pin
    ///
    /// # Limitations
    ///
    /// This method can be used to assign the movable function to pins that are
    /// currently used for something else. The HAL user needs to make sure that
    /// this assignment doesn't conflict with any other uses of the pin.
    fn assign<P: PinName>(&mut self, swm: &mut Api);
}

macro_rules! movable_functions {
    ($($field:ident, $type:ident, $register:ident, $reg_field:ident;)*) => {
        /// Provides access to all movable functions
        #[allow(missing_docs)]
        pub struct MovableFunctions {
            $(pub $field: $type,)*
        }

        impl MovableFunctions {
            fn new() -> Self {
                MovableFunctions {
                    $($field: $type,)*
                }
            }
        }


        $(
            /// Represents a movable function
            #[allow(non_camel_case_types)]
            pub struct $type;

            impl MovableFunction for $type {
                fn assign<P: PinName>(&mut self, swm: &mut Api) {
                    swm.swm.$register.modify(|_, w|
                        unsafe { w.$reg_field().bits(P::ID)
                    })
                }
            }
        )*
    }
}

movable_functions!(
    u0_txd       , U0_TXD       , pinassign0 , u0_txd_o;
    u0_rxd       , U0_RXD       , pinassign0 , u0_rxd_i;
    u0_rts       , U0_RTS       , pinassign0 , u0_rts_o;
    u0_cts       , U0_CTS       , pinassign0 , u0_cts_i;
    u0_sclk      , U0_SCLK      , pinassign1 , u0_sclk_io;
    u1_txd       , U1_TXD       , pinassign1 , u1_txd_o;
    u1_rxd       , U1_RXD       , pinassign1 , u1_rxd_i;
    u1_rts       , U1_RTS       , pinassign1 , u1_rts_o;
    u1_cts       , U1_CTS       , pinassign2 , u1_cts_i;
    u1_sclk      , U1_SCLK      , pinassign2 , u1_sclk_io;
    u2_txd       , U2_TXD       , pinassign2 , u2_txd_o;
    u2_rxd       , U2_RXD       , pinassign2 , u2_rxd_i;
    u2_rts       , U2_RTS       , pinassign3 , u2_rts_o;
    u2_cts       , U2_CTS       , pinassign3 , u2_cts_i;
    u2_sclk      , U2_SCLK      , pinassign3 , u2_sclk_io;
    spi0_sck     , SPI0_SCK     , pinassign3 , spi0_sck_io;
    spi0_mosi    , SPI0_MOSI    , pinassign4 , spi0_mosi_io;
    spi0_miso    , SPI0_MISO    , pinassign4 , spi0_miso_io;
    spi0_ssel0   , SPI0_SSEL0   , pinassign4 , spi0_ssel0_io;
    spi0_ssek1   , SPI0_SSEL1   , pinassign4 , spi0_ssel1_io;
    spi0_ssel2   , SPI0_SSEL2   , pinassign5 , spi0_ssel2_io;
    spi0_ssel3   , SPI0_SSEL3   , pinassign5 , spi0_ssel3_io;
    spi1_sck     , SPI1_SCK     , pinassign5 , spi1_sck_io;
    spi1_mosi    , SPI1_MOSI    , pinassign5 , spi1_mosi_io;
    spi1_miso    , SPI1_MISO    , pinassign6 , spi1_miso_io;
    spi1_ssel0   , SPI1_SSEL0   , pinassign6 , spi1_ssel0_io;
    spi1_ssel1   , SPI1_SSEL1   , pinassign6 , spi1_ssel1_io;
    sct_pin0     , SCT_PIN0     , pinassign6 , sct_in0_i;
    sct_pin1     , SCT_PIN1     , pinassign7 , sct_in1_i;
    sct_pin2     , SCT_PIN2     , pinassign7 , sct_in2_i;
    sct_pin3     , SCT_PIN3     , pinassign7 , sct_in3_i;
    sct_out0     , SCT_OUT0     , pinassign7 , sct_out0_o;
    sct_out1     , SCT_OUT1     , pinassign8 , sct_out1_o;
    sct_out2     , SCT_OUT2     , pinassign8 , sct_out2_o;
    sct_out3     , SCT_OUT3     , pinassign8 , sct_out3_o;
    sct_out4     , SCT_OUT4     , pinassign8 , sct_out4_o;
    sct_out5     , SCT_OUT5     , pinassign9 , sct_out5_o;
    i2c1_sda     , I2C1_SDA     , pinassign9 , i2c1_sda_io;
    i2c1_scl     , I2C1_SCL     , pinassign9 , i2c1_scl_io;
    i2c2_sda     , I2C2_SDA     , pinassign9 , i2c2_sda_io;
    i2c2_scl     , I2C2_SCL     , pinassign10, i2c2_scl_io;
    i2c3_sda     , I2C3_SDA     , pinassign10, i2c3_sda_io;
    i2c3_scl     , I2C3_SCL     , pinassign10, i2c3_scl_io;
    adc_pintrig0 , ADC_PINTRIG0 , pinassign10, adc_pintrig0_i;
    acd_pintrig1 , ADC_PINTRIG1 , pinassign11, adc_pintrig1_i;
    acmp_o       , ACMP_O       , pinassign11, acmp_o_o;
    clkout       , CLKOUT       , pinassign11, clkout_o;
    gpio_int_bmat, GPIO_INT_BMAT, pinassign11, gpio_int_bmat_o;
);


/// Implemented for types that represent fixed functions
pub trait FixedFunction {
    /// Enable the fixed function
    ///
    /// # Limitations
    ///
    /// The fixed function can be enabled on a pin that is currently used for
    /// something else. The HAL user needs to make sure that this assignment
    /// doesn't conflict with any other uses of the pin.
    fn enable(&mut self, swm: &mut Api);

    /// Disable the fixed function
    fn disable(&mut self, swm: &mut Api);
}

macro_rules! fixed_functions {
    ($($type:ident, $field:ident;)*) => {
        // Provides access to all fixed functions
        #[allow(missing_docs)]
        pub struct FixedFunctions {
            $(pub $field: $type,)*
        }

        impl FixedFunctions {
            fn new() -> Self {
                FixedFunctions {
                    $($field: $type,)*
                }
            }
        }


        $(
            /// Represents a fixed function
            #[allow(non_camel_case_types)]
            pub struct $type;

            impl FixedFunction for $type {
                fn enable(&mut self, swm: &mut Api) {
                    swm.swm.pinenable0.modify(|_, w| w.$field().clear_bit());
                }

                fn disable(&mut self, swm: &mut Api) {
                    swm.swm.pinenable0.modify(|_, w| w.$field().set_bit());
                }
            }
        )*
    }
}

fixed_functions!(
    ACMP_I1 , acmp_i1;
    ACMP_I2 , acmp_i2;
    ACMP_I3 , acmp_i3;
    ACMP_I4 , acmp_i4;
    SWCLK   , swclk;
    SWDIO   , swdio;
    XTALIN  , xtalin;
    XTALOUT , xtalout;
    RESETN  , resetn;
    CLKIN   , clkin;
    VDDCMP  , vddcmp;
    I2C0_SDA, i2c0_sda;
    I2C0_SCL, i2c0_scl;
    ADC_0   , adc_0;
    ADC_1   , adc_1;
    ADC_2   , adc_2;
    ADC_3   , adc_3;
    ADC_4   , adc_4;
    ADC_5   , adc_5;
    ADC_6   , adc_6;
    ADC_7   , adc_7;
    ADC_8   , adc_8;
    ADC_9   , adc_9;
    ADC_10  , adc_10;
    ADC_11  , adc_11;
);
