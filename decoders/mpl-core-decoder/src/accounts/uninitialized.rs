use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x84edb17eb33c1a99")]
pub struct Uninitialized {}
