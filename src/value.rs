pub const DEFAULT_TEXT_VALUE_TOKEN: &str = "$";

pub enum Value {
    Fixed(f64),
    Magic(MagicValue),
    Mutiple(Box<Value>, ValueOperation, Box<Value>),
}
pub enum ValueOperation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone, Debug)]
pub enum MagicValue {
    ScreenSizeW,
    ScreenSizeH,
    MousePosX,
    MousePosY,
}
impl Value {
    pub fn compute(&self, ctx: &mut ggez::Context) -> f64 {
        match self {
            Value::Fixed(v) => *v,
            Value::Magic(v) => v.resolve(ctx),
            Value::Mutiple(v1, op, v2) => match op {
                ValueOperation::Add => v1.compute(ctx) + v2.compute(ctx),
                ValueOperation::Sub => v1.compute(ctx) - v2.compute(ctx),
                ValueOperation::Mul => v1.compute(ctx) * v2.compute(ctx),
                ValueOperation::Div => v1.compute(ctx) / v2.compute(ctx),
            },
        }
    }
}

impl MagicValue {
    pub fn resolve(&self, ctx: &ggez::Context) -> f64 {
        match self {
            MagicValue::ScreenSizeW => ctx.gfx.drawable_size().0 as f64,
            MagicValue::ScreenSizeH => ctx.gfx.drawable_size().1 as f64,
            MagicValue::MousePosX => ctx.mouse.position().x as f64,
            MagicValue::MousePosY => ctx.mouse.position().y as f64,
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Fixed(value)
    }
}

impl From<MagicValue> for Value {
    fn from(value: MagicValue) -> Self {
        Value::Magic(value)
    }
}

impl From<(Value, ValueOperation, Value)> for Value {
    fn from(value: (Value, ValueOperation, Value)) -> Self {
        Value::Mutiple(Box::new(value.0), value.1, Box::new(value.2))
    }
}
