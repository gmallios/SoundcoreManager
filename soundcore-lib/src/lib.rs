use enum_dispatch::enum_dispatch;

pub(crate) mod bt;
pub mod device_descriptor;
pub mod device_registry;
mod devices;
pub mod error;
pub(crate) mod mac;
mod types;

#[enum_dispatch]
trait KnobControl {
    //...
    fn a(&self);
}

#[enum_dispatch(KnobControl)]
enum Knob {
    LinearKnob,
    LogarithmicKnob,
}

struct LinearKnob {
    //...
}

impl KnobControl for LinearKnob {
    fn a(&self) {
        println!("LinearKnob");
    }
}

struct LogarithmicKnob {
    //...
}

impl KnobControl for LogarithmicKnob {
    fn a(&self) {
        println!("LinearKnob");
    }
}

fn a() {
    let knob1 = LinearKnob {};
    let knob2 = LogarithmicKnob {};
    let knob3 = Knob::from(knob1);
    let knob4 = Knob::from(knob2);
    let knob5 = vec![knob3, knob4];
    knob5[0].a();
}
